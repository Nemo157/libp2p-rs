use std::io;
use std::cell::RefCell;

use bytes::Bytes;
use futures::{future, Future, Stream, Sink};
use msgio;
use maddr::WriteMultiAddr;
use protobuf::{ProtobufError, Message, parse_from_bytes, RepeatedField};
use tokio_io::codec::{Framed, FramedParts};
use tokio_io::{AsyncRead, AsyncWrite};

use identify::Identify;
use swarm::Swarm;

pub struct IdService {
    // TODO: Yay RC loop...
    swarm: RefCell<Option<Swarm>>
}

fn pbetio(e: ProtobufError) -> io::Error {
    match e {
        ProtobufError::IoError(error) => error,
        ProtobufError::WireError(message) => io::Error::new(io::ErrorKind::Other, message),
        ProtobufError::MessageNotInitialized { message } =>
            io::Error::new(io::ErrorKind::Other, message),
    }
}

fn setup_stream<S: AsyncRead + AsyncWrite + 'static>(parts: FramedParts<S>) -> impl Stream<Item=Identify, Error=io::Error> + Sink<SinkItem=Identify, SinkError=io::Error> {
    Framed::from_parts(parts, msgio::LengthPrefixed(msgio::Prefix::VarInt, msgio::Suffix::None))
        .with(|msg: Identify| msg.write_to_bytes().map(Bytes::from).map_err(pbetio))
        .and_then(|buf| parse_from_bytes(&buf).map_err(pbetio))
}

impl IdService {
    pub fn new() -> IdService {
        IdService { swarm: RefCell::new(None) }
    }

    pub fn update_swarm(&self, swarm: Swarm) {
        *self.swarm.borrow_mut() = Some(swarm);
    }

    pub fn accept<S: AsyncRead + AsyncWrite + 'static>(&self, parts: FramedParts<S>) -> Box<Future<Item=(), Error=()> + 'static> {
        let swarm = if let Some(swarm) = self.swarm.borrow().clone() { swarm } else { panic!("no swarm available") };

        let msg = {
            let mut msg = Identify::new();
            msg.set_protocolVersion("ipfs/0.1.0".to_owned());
            msg.set_agentVersion(swarm.agent().to_owned());
            msg.set_publicKey(swarm.id().pub_key().to_protobuf().unwrap()); // TODO: Might not be the right format
            msg.set_listenAddrs(RepeatedField::from_vec(swarm.listen_addresses().iter().map(|addr| {
                let mut bytes = Vec::new();
                bytes.write_multiaddr(addr).unwrap();
                bytes
            }).collect()));
            msg.set_protocols(RepeatedField::from_vec(swarm.protocols().iter().map(|&s| s.to_owned()).collect()));
            msg
        };

        // stream consists of protobuf encoded messages with a varint length prefix
        Box::new(setup_stream(parts)
            .send(msg)
            .and_then(|stream| stream.into_future().map_err(|(e, _)| e))
            .and_then(|(msg, stream)| {
                println!("idservice msg: {:?}", msg);
                future::ok(())
            })
            .map_err(|err| println!("idservice error: {:?}", err)))
    }
}
