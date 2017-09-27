use std::fmt;
use std::io;

use slog::{b, log, kv, record, record_static};
use slog::{error, info, o, Logger};
use bytes::Bytes;
use futures::{future, Future, Stream, Sink};
use msgio;
use protobuf::{ProtobufError, Message as M, parse_from_bytes};
use tokio_io::codec::{Framed, FramedParts};
use tokio_io::{AsyncRead, AsyncWrite};

use pb::bitswap::Message;
use swarm::Swarm;
use service::Service;

pub struct BitswapService {
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

impl BitswapService {
    pub fn new(swarm: Swarm) -> BitswapService {
        BitswapService { swarm }
    }
}

impl<S: AsyncRead + AsyncWrite + 'static> Service<S> for BitswapService {
    fn name(&self) -> &'static str {
        "/ipfs/bitswap/1.1.0"
    }

    fn accept(&self, logger: Logger, parts: FramedParts<S>) -> Box<Future<Item=(), Error=()> + 'static> {
        Box::new({
            let logger = logger.clone();
            setup_stream(parts)
                .for_each(move |msg| {
                    info!(logger, "bitswap msg: {:?}", msg);
                    future::ok(())
                })
        }.map_err(move |err| error!(logger, "bitswap error: {}, {:?}", err, err)))
    }
}

impl fmt::Debug for BitswapService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BitswapService")
            .finish()
    }
}
