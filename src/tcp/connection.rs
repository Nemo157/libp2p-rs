use std::{ io, net };

use { Connection, Transport };

#[derive(Debug)]
pub struct TcpConnection {
    stream: net::TcpStream,
}

impl TcpConnection {
    pub fn connect<A: net::ToSocketAddrs>(addr: A) -> io::Result<TcpConnection> {
        Ok(TcpConnection {
            stream: try!(net::TcpStream::connect(addr)),
        })
    }
}

impl Connection for TcpConnection {
}

impl io::Read for TcpConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.stream.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.stream.read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.stream.read_exact(buf)
    }
}

impl io::Write for TcpConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.stream.write_all(buf)
    }
}
