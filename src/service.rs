use futures::Future;
use tokio_io::codec::FramedParts;
use tokio_io::{AsyncRead, AsyncWrite};


pub trait Service<S: AsyncRead + AsyncWrite + 'static> {
    fn name(&self) -> &'static str;

    fn accept(&self, parts: FramedParts<S>) -> Box<Future<Item=(), Error=()> + 'static>;
}
