use { PeerInfo, Transport, Connection };

#[derive(Debug)]
pub struct Peer {
    info: PeerInfo,
    connections: Vec<Box<Connection>>,
}

impl Peer {
    pub fn new(info: PeerInfo) -> Peer {
        Peer { info: info, connections: Vec::new() }
    }

    pub fn connect(&mut self, transports: &mut [Box<Transport>]) -> Result<(), ()> {
        for addr in self.info.addrs() {
            for transport in transports.iter_mut() {
                if transport.can_handle(addr) {
                    self.connections.push(try!(transport.connect(addr)));
                }
            }
        }
        Ok(())
    }
}
