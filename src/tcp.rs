use std::{ io, net };
use maddr::{ MultiAddr, Segment };

#[derive(Debug)]
pub struct Transport(net::TcpStream);

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

pub fn connect(addr: &MultiAddr) -> io::Result<Transport> {
    let segments = addr.segments();
    if segments.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid address"));
    }

    Ok(match (&segments[0], &segments[1]) {
        (&Segment::IP4(ref addr), &Segment::Tcp(ref port)) => {
            Transport(net::TcpStream::connect((*addr, *port))?)
        }
        (&Segment::IP6(ref addr), &Segment::Tcp(ref port)) => {
            Transport(net::TcpStream::connect((*addr, *port))?)
        }
        _ => {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid address"));
        }
    })
}
