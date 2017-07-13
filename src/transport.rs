use std::io;

use maddr::MultiAddr;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_core::reactor;
use futures::{ future, Future, Poll };

use tcp;

#[derive(Debug)]
pub enum Transport {
    Tcp(tcp::Transport),
}

pub fn connect(addr: &MultiAddr, event_loop: &reactor::Handle) -> impl Future<Item=Transport, Error=io::Error> {
    if tcp::can_handle(addr) {
        future::Either::A(tcp::connect(addr, event_loop).map(Transport::Tcp))
    } else {
        future::Either::B(future::err(io::Error::new(io::ErrorKind::Other, "No transports can handle")))
    }
}

impl io::Read for Transport {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.read(buf)
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.read_to_end(buf)
        }
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.read_to_string(buf)
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.read_exact(buf)
        }
    }
}

impl io::Write for Transport {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.flush()
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.write_all(buf)
        }
    }
}

impl AsyncRead for Transport { }
impl AsyncWrite for Transport {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        match *self {
            Transport::Tcp(ref mut transport) => transport.shutdown()
        }
    }
}
