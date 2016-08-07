use std::{ fmt, io };
use multistream::Negotiator;
use secio::SecStream;
use identity::HostId;

use msgio::{ ReadLpm, WriteLpm };

use { PeerInfo, Transport };

trait MessageStream: fmt::Debug + ReadLpm + WriteLpm {
}

impl<S> MessageStream for S where S: fmt::Debug + ReadLpm + WriteLpm {
}

#[derive(Debug)]
pub struct Peer {
    info: PeerInfo,
    allow_unencrypted: bool,
    idle_connection: Option<Box<MessageStream>>,
}

impl Peer {
    pub fn new(info: PeerInfo, allow_unencrypted: bool) -> Peer {
        Peer {
            info: info,
            allow_unencrypted: allow_unencrypted,
            idle_connection: None,
        }
    }

    pub fn pre_connect(&mut self, host: &HostId, transports: &mut [Box<Transport>]) -> io::Result<()> {
        if let None = self.idle_connection {
            for addr in self.info.addrs() {
                for transport in transports.iter_mut() {
                    if transport.can_handle(addr) {
                        let conn = transport.connect(addr).and_then(|conn| {
                            let mut negotiator = Negotiator::start(conn)
                                .negotiate(b"/secio/1.0.0", |conn| SecStream::new(conn, host, self.info.id()).map(|c| Box::new(c) as Box<MessageStream>));
                            if self.allow_unencrypted {
                                negotiator = negotiator.negotiate(b"/plaintext/1.0.0", |conn| Ok(Box::new(conn) as Box<MessageStream>));
                            }
                            negotiator.finish()
                        });
                        match conn {
                            Ok(conn) => {
                                self.idle_connection = Some(conn);
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
