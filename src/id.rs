use std::fmt;
use std::io;

use slog::{b, log, kv, record, record_static};
use slog::{error, info, Logger};
use bytes::Bytes;
use futures::{future, Future, Stream, Sink};
use msgio;
use maddr::WriteMultiAddr;
use protobuf::{ProtobufError, Message, parse_from_bytes, RepeatedField};
use tokio_io::codec::{Framed, FramedParts};
use tokio_io::{AsyncRead, AsyncWrite};

use pb::identify::Identify;
use swarm::Swarm;
use service::Service;

pub struct IdService {
    // TODO: Yay RC loop...
    swarm: Swarm,
}

fn pbetio(e: ProtobufError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

fn setup_stream<S: AsyncRead + AsyncWrite + 'static>(parts: FramedParts<S>) -> impl Stream<Item=Identify, Error=io::Error> + Sink<SinkItem=Identify, SinkError=io::Error> {
    Framed::from_parts(parts, msgio::LengthPrefixed(msgio::Prefix::VarInt, msgio::Suffix::None))
        .with(|msg: Identify| msg.write_to_bytes().map(Bytes::from).map_err(pbetio))
        .and_then(|buf| parse_from_bytes(&buf).map_err(pbetio))
}

impl IdService {
    pub fn new(swarm: Swarm) -> IdService {
        IdService { swarm }
    }
}

impl<S: AsyncRead + AsyncWrite + 'static> Service<S> for IdService {
    fn name(&self) -> &'static str {
        "/ipfs/id/1.0.0"
    }

    fn accept(&self, logger: Logger, parts: FramedParts<S>) -> Box<Future<Item=(), Error=io::Error> + 'static> {
        let msg = {
            let mut msg = Identify::new();
            msg.set_protocolVersion("ipfs/0.1.0".to_owned());
            msg.set_agentVersion(self.swarm.agent().to_owned());
            msg.set_publicKey(self.swarm.id().pub_key().to_protobuf().unwrap()); // TODO: Might not be the right format
            msg.set_listenAddrs(RepeatedField::from_vec(self.swarm.listen_addresses().iter().map(|addr| {
                let mut bytes = Vec::new();
                bytes.write_multiaddr(addr).unwrap();
                bytes
            }).collect()));
            msg.set_protocols(RepeatedField::from_vec(self.swarm.protocols().map(|s| s.to_owned()).collect()));
            msg
        };

        // stream consists of protobuf encoded messages with a varint length prefix
        Box::new({
            let logger = logger.clone();
            setup_stream(parts)
            .send(msg)
            .and_then(|stream| stream.into_future().map_err(|(e, _)| e))
            .and_then(move |(msg, _stream)| {
                info!(logger, "idservice msg: {:?}", msg);
                future::ok(())
            })
        })
    }
}

impl fmt::Debug for IdService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IdService")
            .finish()
    }
}
