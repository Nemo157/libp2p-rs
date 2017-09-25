use std::fmt;

use futures::Future;
use tokio_io::codec::FramedParts;
use tokio_io::{AsyncRead, AsyncWrite};
use slog::Logger;


pub trait Service<S: AsyncRead + AsyncWrite + 'static>: fmt::Debug {
    fn name(&self) -> &'static str;

    fn accept(&self, logger: Logger, parts: FramedParts<S>) -> Box<Future<Item=(), Error=()> + 'static>;
}
