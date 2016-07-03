use std::fmt;
use multiaddr::MultiAddr;

use { Connection };

pub trait Transport: fmt::Debug {
    fn can_handle(&self, addr: &MultiAddr) -> bool;
    fn connect(&mut self, addr: &MultiAddr) -> Result<Box<Connection>, ()>;
}
