use futures::Future;

trait Service {
    type Future: Future<Item=(), Error=()>;
    pub fn accept<S: AsyncRead + AsyncWrite + 'static>(parts: FramedParts<S>) -> Self::Future;
}
