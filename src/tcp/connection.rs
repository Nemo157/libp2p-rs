use std::net;

use { Connection, Transport };

#[derive(Debug)]
pub struct TcpConnection {
    stream: net::TcpStream,
}

impl TcpConnection {
    pub fn connect<A: net::ToSocketAddrs>(addr: A) -> Result<TcpConnection, ()> {
        Ok(TcpConnection {
            stream: try!(net::TcpStream::connect(addr).or(Err(()))),
        })
    }
}

impl Connection for TcpConnection {
}
