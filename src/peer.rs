use std::io;
use std::borrow::Cow;
use multiaddr::MultiAddr;
use multistream::MultiStream;

use { PeerInfo, Transport, Connection };

#[derive(Debug)]
pub struct Peer {
    info: PeerInfo,
    idle_connection: Option<MultiStream<Box<Connection>>>,
}

impl Peer {
    pub fn new(info: PeerInfo) -> Peer {
        Peer { info: info, idle_connection: None }
    }

    pub fn pre_connect(&mut self, transports: &mut [Box<Transport>]) -> io::Result<()> {
        if let None = self.idle_connection {
            for addr in self.info.addrs() {
                for transport in transports.iter_mut() {
                    if transport.can_handle(addr) {
                        let stream = transport.connect(addr)
                            .and_then(|conn| MultiStream::negotiate(conn, Cow::Borrowed(b"/plaintext/1.0.0")));
                        match stream {
                            Ok(connection) => {
                                self.idle_connection = Some(connection);
                            }
                            Err(error) => {
                                println!("{}", error);
                            }
                        }
                    }
                }
            }
        }
        self.idle_connection.as_ref().map(|_| ()).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to connect"))
    }
}
