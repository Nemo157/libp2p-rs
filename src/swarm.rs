use std::io;
use std::rc::Rc;
use std::cell::RefCell;

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

struct State {
    id: HostId,
    agent: String,
    event_loop: reactor::Handle,
    peers: RefCell<Vec<Peer>>,
    listen_addresses: Vec<MultiAddr>,
    listeners: RefCell<Vec<Box<Stream<Item=(transport::Transport, MultiAddr), Error=io::Error>>>>,
    accepting: RefCell<Vec<Box<Future<Item=(PeerId, mux::Stream, MultiAddr), Error=io::Error>>>>,
    id_service: Rc<IdService>,
    accepting_services: RefCell<Vec<Box<Future<Item=Box<Future<Item=(), Error=()> + 'static>, Error=io::Error> + 'static>>>,
    connected_services: RefCell<Vec<Box<Future<Item=(), Error=()> + 'static>>>,
}

pub struct Swarm(Rc<State>);

impl Clone for Swarm {
    fn clone(&self) -> Self { Swarm(self.0.clone()) }
}

fn accept_stream(state: Rc<State>, stream: mplex::Stream, _peer: &Peer) -> impl Future<Item=Box<Future<Item=(), Error=()> + 'static>, Error=io::Error> {
    let ping = PingService::new();
    let services: Vec<(&'static [u8], Rc<Service<mplex::Stream>>)> = vec![
        (b"/ipfs/ping/1.0.0", state.id_service.clone()),
        (b"/ipfs/id/1.0.0", Rc::new(ping)),
    ];
    let mut negotiator = Negotiator::start(stream, false);
    for (name, service) in services {
        negotiator = negotiator.negotiate(name, move |parts: FramedParts<mplex::Stream>| -> Box<Future<Item=_, Error=_>> {
            Box::new(future::ok(service.accept(parts)))
        });
    }
    negotiator.finish()
}

impl Swarm {
    pub fn new(id: HostId, agent: String, listen_addresses: Vec<MultiAddr>, event_loop: reactor::Handle) -> io::Result<Swarm> {
        let listeners: io::Result<Vec<_>> = listen_addresses.iter()
            .map(|addr| transport::listen(addr, &event_loop).map(|transport| Box::new(transport) as Box<_>))
            .collect();
        let swarm = Swarm(Rc::new(State {
            id: id.clone(),
            agent,
            event_loop: event_loop.clone(),
            peers: RefCell::new(Vec::new()),
            listen_addresses: listen_addresses,
            listeners: RefCell::new(listeners?),
            accepting: RefCell::new(Vec::new()),
            id_service: Rc::new(IdService::new()),
            accepting_services: RefCell::new(Vec::new()),
            connected_services: RefCell::new(Vec::new()),
        }));
        swarm.0.id_service.update_swarm(swarm.clone());
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

    pub fn protocols(&self) -> &[&str] {
        static PROTOS: &[&str] = &["/ipfs/id/1.0.0", "/ipfs/ping/1.0.0"];
        PROTOS
    }

    pub fn add_peer(&mut self, info: PeerInfo) {
        println!("Adding peer {:?}", info);
        let peer = Peer::new(self.0.id.clone(), info, self.0.event_loop.clone());
        self.0.peers.borrow_mut().push(peer);
    }

    pub fn add_peers(&mut self, infos: Vec<PeerInfo>) {
        println!("Adding peers {:?}", infos);
        let id = self.0.id.clone();
        let event_loop = self.0.event_loop.clone();
        let peers = infos.into_iter().map(|info| Peer::new(id.clone(), info, event_loop.clone()));
        self.0.peers.borrow_mut().extend(peers);
    }

    pub fn open_stream(&mut self, id: PeerId, protocol: &'static [u8]) -> impl Future<Item=FramedParts<mplex::Stream>, Error=io::Error> {
        if let Some(peer) = self.0.peers.borrow_mut().iter_mut().find(|peer| id.matches(&peer.id())) {
            future::Either::A(peer.open_stream(protocol))
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

        for listener in &mut *self.0.listeners.borrow_mut() {
            while let Async::Ready(Some((conn, addr))) = listener.poll()? {
                accepting.push(Box::new(Peer::start_accept(
                        self.0.id.clone(),
                        conn,
                        addr)));
            }
        }

        {
            let mut i = 0;
            while i < accepting.len() {
                match accepting[i].poll() {
                    Ok(Async::Ready((id, conn, addr))) => {
                        accepting.swap_remove(i);
                        if let Some(peer) = peers.iter_mut().find(|peer| peer.id().matches(&id)) {
                            peer.finish_accept(conn, addr);
                            continue;
                        } // else
                        let info = PeerInfo::new(id, vec![]);
                        let mut peer = Peer::new(self.0.id.clone(), info, self.0.event_loop.clone());
                        peer.finish_accept(conn, addr);
                        peers.push(peer);
                    }
                    Ok(Async::NotReady) => {
                        i += 1;
                    }
                    Err(err) => {
                        println!("Error while accepting peers stream: {:?}", err);
                        accepting.swap_remove(i);
                    }
                }
            }
        }

        for peer in peers.iter_mut() {
            while let Async::Ready(Some(stream)) = peer.poll_accept()? {
                accepting_services.push(Box::new(accept_stream(self.0.clone(), stream, &peer)));
            }
        }

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
                        println!("Error while accepting peers muxed stream: {:?}", err);
                        accepting_services.swap_remove(i);
                    }
                }
            }
        }

        {
            let mut i = 0;
            while i < connected_services.len() {
                match connected_services[i].poll() {
                    Ok(Async::Ready(())) | Err(()) => {
                        connected_services.swap_remove(i);
                    }
                    Ok(Async::NotReady) => {
                        i += 1;
                    }
                }
            }
        }

        Ok(Async::NotReady)
    }
}
