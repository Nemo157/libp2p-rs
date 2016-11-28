use std::io;

use maddr::MultiAddr;

use tcp;

#[derive(Debug)]
pub enum Transport {
    Tcp(tcp::Transport),
}

pub fn connect(addr: &MultiAddr) -> io::Result<Transport> {
    if tcp::can_handle(addr) {
        Ok(Transport::Tcp(tcp::connect(addr)?))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "No transports can handle"))
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
