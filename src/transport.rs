use std::{ io, fmt };
use maddr::MultiAddr;

use { Connection };

pub trait Transport: fmt::Debug {
    fn can_handle(&self, addr: &MultiAddr) -> bool;
    fn connect(&mut self, addr: &MultiAddr) -> io::Result<Box<Connection>>;
}
