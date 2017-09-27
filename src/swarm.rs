use std::cell::RefCell;
use std::fmt;
use std::io;
use std::rc::Rc;

use slog::{b, log, kv, record, record_static};
use slog::{trace, error, info, o, Logger};
use futures::{ future, Future, Poll, Async, Stream };
use tokio_core::reactor;
use identity::{ HostId, PeerId };
use mplex;
use multistream::Negotiator;
use tokio_io::codec::FramedParts;
use maddr::MultiAddr;

use { PeerInfo };
use peer::Peer;
use ping::PingService;
use transport;
use mux;
use id::IdService;
use service::Service;
use dht::DhtService;
use bitswap::BitswapService;

struct State {
    logger: Logger,
    id: HostId,
    agent: String,
    event_loop: reactor::Handle,
    peers: RefCell<Vec<Peer>>,
    listen_addresses: Vec<MultiAddr>,
    listeners: RefCell<Vec<Box<Stream<Item=(transport::Transport, MultiAddr), Error=io::Error>>>>,
    accepting: RefCell<Vec<Box<Future<Item=(PeerId, mux::Stream, MultiAddr), Error=io::Error>>>>,
    services: RefCell<Vec<Rc<Service<mplex::Stream>>>>,
    accepting_services: RefCell<Vec<Box<Future<Item=Box<Future<Item=(), Error=()> + 'static>, Error=io::Error> + 'static>>>,
    connected_services: RefCell<Vec<Box<Future<Item=(), Error=()> + 'static>>>,
}

pub struct Swarm(Rc<State>);

impl Clone for Swarm {
    fn clone(&self) -> Self { Swarm(self.0.clone()) }
}

fn accept_stream(state: Rc<State>, stream: mplex::Stream, peer: &Peer) -> impl Future<Item=Box<Future<Item=(), Error=()> + 'static>, Error=io::Error> {
    let logger = state.logger.new(o!{
        "peer" => format!("{:#?}", peer.id()),
        "stream_id" => stream.id()
    });
    let mut negotiator = Negotiator::start(logger.clone(), stream, false);
    for service in &*state.services.borrow() {
        let service = service.clone();
        let logger = logger.new(o!{
            "service" => service.name()
        });
        negotiator = negotiator.negotiate(service.name(), move |parts| {
            info!(logger, "Accepted stream");
            Box::new(service.accept(logger.clone(), parts).then(move |result| {
                info!(logger, "Service done: {:?}", result);
                future::ok(())
            })) as Box<Future<Item=(), Error=()>>
        });
    }
    negotiator.finish()
}

impl Swarm {
    pub fn new(logger: Logger, id: HostId, agent: String, listen_addresses: Vec<MultiAddr>, event_loop: reactor::Handle) -> io::Result<Swarm> {
        let logger = logger.new(o!("host" => id.hash().to_string()));
        let listeners: io::Result<Vec<_>> = listen_addresses.iter()
            .map(|addr| transport::listen(addr, &event_loop).map(|transport| Box::new(transport) as Box<_>))
            .collect();
        let swarm = Swarm(Rc::new(State {
            logger, id, agent,
            event_loop: event_loop.clone(),
            peers: RefCell::new(Vec::new()),
            listen_addresses: listen_addresses,
            listeners: RefCell::new(listeners?),
            accepting: RefCell::new(Vec::new()),
            services: RefCell::new(Vec::new()),
            accepting_services: RefCell::new(Vec::new()),
            connected_services: RefCell::new(Vec::new()),
        }));
        *swarm.0.services.borrow_mut() = vec![
            Rc::new(PingService::new()),
            Rc::new(IdService::new(swarm.clone())),
            Rc::new(DhtService::new(swarm.clone())),
            Rc::new(BitswapService::new(swarm.clone())),
        ];
        Ok(swarm)
    }

    pub fn agent(&self) -> &str {
        &self.0.agent
    }

    pub fn id(&self) -> &HostId {
        &self.0.id
    }

    pub fn listen_addresses(&self) -> &[MultiAddr] {
        &self.0.listen_addresses
    }

    pub fn protocols(&self) -> impl Iterator<Item=&'static str> {
        self.0.services.borrow().iter().map(|service| service.name()).collect::<Vec<_>>().into_iter()
    }

    pub fn add_peer(&mut self, info: PeerInfo) {
        info!(self.0.logger, "Adding peer {:?}", info);
        let peer = Peer::new(self.0.logger.clone(), self.0.id.clone(), info, self.0.event_loop.clone());
        self.0.peers.borrow_mut().push(peer);
    }

    pub fn add_peers(&mut self, infos: Vec<PeerInfo>) {
        info!(self.0.logger, "Adding peers {:?}", infos);
        let id = self.0.id.clone();
        let event_loop = self.0.event_loop.clone();
        let peers = infos.into_iter().map(|info| Peer::new(self.0.logger.clone(), id.clone(), info, event_loop.clone()));
        self.0.peers.borrow_mut().extend(peers);
    }

    pub fn open_stream<P: AsRef<str>>(&mut self, id: PeerId, protocol: P) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        if let Some(peer) = self.0.peers.borrow_mut().iter_mut().find(|peer| id.matches(&peer.id())) {
            future::Either::A(peer.clone().open_stream(protocol))
        } else {
            future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, format!("Could not find peer {:?}", id))))
        }
    }
}

impl Future for Swarm {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut accepting = self.0.accepting.borrow_mut();
        let mut accepting_services = self.0.accepting_services.borrow_mut();
        let mut connected_services = self.0.connected_services.borrow_mut();
        let mut peers = self.0.peers.borrow_mut();

        trace!(self.0.logger, "Checking listeners");
        for listener in &mut *self.0.listeners.borrow_mut() {
            while let Async::Ready(Some((conn, addr))) = listener.poll()? {
                trace!(self.0.logger, "New connection from {}", addr);
                accepting.push(Box::new(Peer::start_accept(
                        self.0.logger.new(o!("addr" => addr.to_string())),
                        self.0.id.clone(),
                        conn,
                        addr)));
            }
        }

        trace!(self.0.logger, "Checking accepting");
        {
            let mut i = 0;
            while i < accepting.len() {
                match accepting[i].poll() {
                    Ok(Async::Ready((id, conn, addr))) => {
                        trace!(self.0.logger, "Finished accepting for {}", addr);
                        accepting.swap_remove(i);
                        if let Some(peer) = peers.iter_mut().find(|peer| peer.id().matches(&id)) {
                            peer.finish_accept(conn, addr);
                            continue;
                        } // else
                        let info = PeerInfo::new(id, vec![]);
                        let mut peer = Peer::new(self.0.logger.clone(), self.0.id.clone(), info, self.0.event_loop.clone());
                        peer.finish_accept(conn, addr);
                        peers.push(peer);
                    }
                    Ok(Async::NotReady) => {
                        i += 1;
                    }
                    Err(err) => {
                        error!(self.0.logger, "Error while accepting peers stream: {:?}", err);;
                        accepting.swap_remove(i);
                    }
                }
            }
        }

        trace!(self.0.logger, "Checking peers");
        for peer in peers.iter_mut() {
            while let Async::Ready(Some(stream)) = peer.poll_accept()? {
                trace!(self.0.logger, "Accepting new stream");
                accepting_services.push(Box::new(accept_stream(self.0.clone(), stream, &peer)));
            }
        }

        trace!(self.0.logger, "Checking accepting services");
        {
            let mut i = 0;
            while i < accepting_services.len() {
                match accepting_services[i].poll() {
                    Ok(Async::Ready(service)) => {
                        accepting_services.swap_remove(i);
                        connected_services.push(service);
                    }
                    Ok(Async::NotReady) => {
                        i += 1;
                    }
                    Err(err) => {
                        error!(self.0.logger, "Error while accepting peers muxed stream: {:?}", err);
                        accepting_services.swap_remove(i);
                    }
                }
            }
        }

        trace!(self.0.logger, "Checking connected services");
        {
            let mut i = 0;
            while i < connected_services.len() {
                match connected_services[i].poll() {
                    Ok(Async::Ready(())) | Err(()) => {
                        trace!(self.0.logger, "Connected service done");
                        connected_services.swap_remove(i);
                    }
                    Ok(Async::NotReady) => {
                        i += 1;
                    }
                }
            }
        }

        trace!(self.0.logger, "Finished checking swarm");
        Ok(Async::NotReady)
    }
}

impl fmt::Debug for Swarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("Swarm")
                .field("id", &self.0.id)
                .field("agent", &self.0.agent)
                .field("peers", &self.0.peers.borrow())
                .field("listen_addresses", &self.0.listen_addresses)
                .field("listeners", &self.0.listeners.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .field("accepting", &self.0.accepting.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .field("services", &self.0.services.borrow())
                .field("accepting_services", &self.0.accepting_services.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .field("connected_services", &self.0.connected_services.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .finish()
        } else {
            f.debug_struct("Swarm")
                .field("id", &self.0.id)
                .field("agent", &self.0.agent)
                .field("peers", &self.0.peers)
                .field("listen_addresses", &self.0.listen_addresses)
                .field("listeners", &self.0.listeners.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .field("accepting", &self.0.accepting.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .field("services", &self.0.services)
                .field("accepting_services", &self.0.accepting_services.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .field("connected_services", &self.0.connected_services.borrow().iter().map(|_| "<omitted>").collect::<Vec<_>>())
                .finish()
        }
    }
}
