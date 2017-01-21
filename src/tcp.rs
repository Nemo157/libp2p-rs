use std::io;
use std::net::{ IpAddr, SocketAddr };

use maddr::{ MultiAddr, Segment };
use tokio_core::net::TcpStream;
use tokio_core::reactor;
use futures::{ future, Future };

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

pub fn connect(addr: &MultiAddr, event_loop: &reactor::Handle) -> impl Future<Item=Transport, Error=io::Error> {
    let segments = addr.segments();
    if segments.len() != 2 {
        return future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, "Invalid address")));
    }

    match (&segments[0], &segments[1]) {
        (&Segment::IP4(addr), &Segment::Tcp(port)) => {
            future::Either::A(TcpStream::connect(&SocketAddr::new(IpAddr::V4(addr), port), event_loop).map(Transport))
        }
        (&Segment::IP6(addr), &Segment::Tcp(port)) => {
            future::Either::A(TcpStream::connect(&SocketAddr::new(IpAddr::V6(addr), port), event_loop).map(Transport))
        }
        _ => {
            future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, "Invalid address")))
        }
    }
}
