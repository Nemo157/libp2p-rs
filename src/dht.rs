use std::fmt;
use std::io;

use slog::{b, log, kv, record, record_static};
use slog::{error, info, Logger};
use bytes::Bytes;
use futures::{future, Future, Stream, Sink};
use msgio;
use protobuf::{ProtobufError, Message as M, parse_from_bytes};
use tokio_io::codec::{Framed, FramedParts};
use tokio_io::{AsyncRead, AsyncWrite};
use futures::prelude::{async, await};
use identity::PeerId;

use pb::dht::{Message, Message_MessageType};
use swarm::Swarm;
use service::Service;

#[derive(Clone)]
pub struct DhtService {
    // TODO: Yay RC loop...
    swarm: Swarm,
}

fn pbetio(e: ProtobufError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

fn setup_stream<S: AsyncRead + AsyncWrite + 'static>(parts: FramedParts<S>) -> impl Stream<Item=Message, Error=io::Error> + Sink<SinkItem=Message, SinkError=io::Error> {
    Framed::from_parts(parts, msgio::LengthPrefixed(msgio::Prefix::VarInt, msgio::Suffix::None))
        .with(|msg: Message| msg.write_to_bytes().map(Bytes::from).map_err(pbetio))
        .and_then(|buf| parse_from_bytes(&buf).map_err(pbetio))
}

impl DhtService {
    pub fn new(swarm: Swarm) -> DhtService {
        DhtService { swarm }
    }

    #[async]
    fn accept<S: AsyncRead + AsyncWrite + 'static>(self, logger: Logger, parts: FramedParts<S>) -> Box<Future<Item=(), Error=io::Error> + 'static> {
        let logger = logger.clone();
        let (mut tx, rx) = setup_stream(parts).split();

        #[async]
        for msg in rx {
            let (logger, this) = (logger.clone(), self.clone());
            info!(logger, "kad msg: {:?}", msg);
            match { let logger = logger.clone(); this.handle(logger, msg) } {
                Ok(response) => {
                    tx = await!(tx.send(response))?;
                }
                Err(err) => {
                    error!(logger, "Error handling kad msg: {}", err);
                }
            }
        }
        Ok(())
    }

    fn handle(self, logger: Logger, msg: Message) -> io::Result<Message> {
        match msg.get_field_type() {
            Message_MessageType::PUT_VALUE => {
                unimplemented!();
            }
            Message_MessageType::GET_VALUE => {
                unimplemented!();
            }
            Message_MessageType::ADD_PROVIDER => {
                unimplemented!();
            }
            Message_MessageType::GET_PROVIDERS => {
                unimplemented!();
            }
            Message_MessageType::FIND_NODE => {
                // let mut response = Message::new();
                // response.set_field_type(Message_MessageType::FIND_NODE);
                // let id = PeerId::from_protobuf(msg.key());
                // let peers = if id == self.swarm.id() {
                //     vec![{
                //         let mut peer = Message_peer::new();
                //         peer.set_id(self.swarm.id().to_string());
                //         peer.set_addrs(RepeatedField::from_vec(self.swarm.listen_addresses().iter().map(|addr| {
                //             let mut bytes = Vec::new();
                //             bytes.write_multiaddr(addr).unwrap();
                //             bytes
                //         }).collect()));
                //         peer
                //     }]
                // };
                // response.set_closerPeers(
                //     RepeatedField::from_vec(self.swarm.listen_addresses().iter().map(|addr| {
                //     let mut bytes = Vec::new();
                //     bytes.write_multiaddr(addr).unwrap();
                //     bytes
                // }).collect()));
                unimplemented!();
            }
            Message_MessageType::PING => {
                info!(logger, "kad ping: {:?}", msg);
                Ok(msg)
            }
        }
    }
}

impl<S: AsyncRead + AsyncWrite + 'static> Service<S> for DhtService {
    fn name(&self) -> &'static str {
        "/ipfs/kad/1.0.0"
    }

    fn accept(&self, logger: Logger, parts: FramedParts<S>) -> Box<Future<Item=(), Error=io::Error> + 'static> {
        self.clone().accept(logger, parts)
    }
}

impl fmt::Debug for DhtService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DhtService")
            .finish()
    }
}
