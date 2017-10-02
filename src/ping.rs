use std::fmt;
use std::io;

use slog::{b, log, kv, record, record_static};
use slog::{error, info, Logger};
use futures::{future, Future, Stream, Sink};
use bytes::{BufMut, BytesMut};
use slice_as_array::slice_to_array_clone;
use tokio_io::codec::{Decoder, Encoder, Framed, FramedParts};
use tokio_io::{AsyncRead, AsyncWrite};

use service::Service;

const PING_LENGTH: usize = 16;

#[derive(Debug)]
struct Codec;

pub struct PingService(());

impl PingService {
    pub fn new() -> PingService {
        PingService(())
    }
}

impl<S: AsyncRead + AsyncWrite + 'static> Service<S> for PingService {
    fn name(&self) -> &'static str {
        "/ipfs/ping/1.0.0"
    }

    fn accept(&self, logger: Logger, parts: FramedParts<S>) -> Box<Future<Item=(), Error=()> + 'static> {
        Box::new(Framed::from_parts(parts, Codec)
            .into_future()
            .map_err(|(err, _)| err)
            .and_then(|(ping, stream)| {
                if let Some(ping) = ping {
                    future::Either::A(stream.send(ping).map(|_| ()))
                } else {
                    future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, "Stream closed before receiving ping")))
                }
            })
            .then(move |res| match res {
                Err(err) => {
                    error!(logger, "Error during ping: {:?}", err);
                    Err(())
                }
                Ok(()) => {
                    info!(logger, "Ping successful");
                    Ok(())
                }
            }))
    }
}

impl Decoder for Codec {
    type Item = [u8; PING_LENGTH];
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() >= PING_LENGTH {
            let bytes = src.split_to(PING_LENGTH);
            let bytes = slice_to_array_clone!(&*bytes, [u8; PING_LENGTH])
                .expect("Already verified the length");
            Ok(Some(bytes))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for Codec {
    type Item = [u8; PING_LENGTH];
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(PING_LENGTH);
        dst.put(&item[..]);
        Ok(())
    }
}

impl fmt::Debug for PingService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PingService")
            .finish()
    }
}
