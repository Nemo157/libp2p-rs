use std::io;
use std::net::{ IpAddr, SocketAddr };

use maddr::{ MultiAddr, Segment };
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor;
use futures::{future, Future, Stream};

#[derive(Debug)]
pub struct Transport(TcpStream);

proxy_stream!(Transport, self.0);

pub fn can_handle(addr: &MultiAddr) -> bool {
    let segments = addr.segments();
    if segments.len() != 2 {
        return false;
    }
    match (&segments[0], &segments[1]) {
        (&Segment::IP4(..), &Segment::Tcp(..)) => true,
        (&Segment::IP6(..), &Segment::Tcp(..)) => true,
        _ => false,
    }
}

fn multiaddr_to_socketaddr(addr: &MultiAddr) -> io::Result<SocketAddr> {
    let segments = addr.segments();
    if segments.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid address"));
    }

    Ok(match (&segments[0], &segments[1]) {
        (&Segment::IP4(addr), &Segment::Tcp(port)) => SocketAddr::new(IpAddr::V4(addr), port),
        (&Segment::IP6(addr), &Segment::Tcp(port)) => SocketAddr::new(IpAddr::V6(addr), port),
        _ => {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid address"));
        },
    })
}

fn socketaddr_to_multiaddr(addr: &SocketAddr) -> MultiAddr {
    let ip = match addr.ip() {
        IpAddr::V4(ip) => Segment::IP4(ip),
        IpAddr::V6(ip) => Segment::IP6(ip),
    };
    let port = Segment::Tcp(addr.port());
    MultiAddr::new(vec![ip, port])
}

pub fn connect(addr: &MultiAddr, event_loop: &reactor::Handle) -> impl Future<Item=Transport, Error=io::Error> {
    match multiaddr_to_socketaddr(addr) {
        Ok(addr) => future::Either::A(TcpStream::connect(&addr, event_loop).map(Transport)),
        Err(err) => future::Either::B(future::err(err)),
    }
}

pub fn listen(addr: &MultiAddr, event_loop: &reactor::Handle) -> io::Result<impl Stream<Item=(Transport, MultiAddr), Error=io::Error>> {
    let addr = multiaddr_to_socketaddr(addr)?;
    Ok(TcpListener::bind(&addr, event_loop)?
        .incoming() 
        .map(|(transport, addr)| (Transport(transport), socketaddr_to_multiaddr(&addr))))
}
