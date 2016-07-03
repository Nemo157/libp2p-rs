use std::io;
use multiaddr::{ MultiAddr, Segment };

use { Connection, Transport };
use tcp::TcpConnection;

#[derive(Debug)]
pub struct TcpTransport {
}

impl TcpTransport {
    pub fn new() -> TcpTransport {
        TcpTransport {
        }
    }
}

impl Transport for TcpTransport {
    fn can_handle(&self, addr: &MultiAddr) -> bool {
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

    fn connect(&mut self, addr: &MultiAddr) -> io::Result<Box<Connection>> {
        let segments = addr.segments();
        if segments.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid address"));
        }

        Ok(Box::new(match (segments[0].clone(), segments[1].clone()) {
            (Segment::IP4(addr), Segment::Tcp(port)) => {
                try!(TcpConnection::connect((addr, port)))
            }
            (Segment::IP6(addr), Segment::Tcp(port)) => {
                try!(TcpConnection::connect((addr, port)))
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid address"));
            }
        }))
    }
}
