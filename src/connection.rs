use std::{ fmt, io };

pub trait Connection: fmt::Debug + io::Read + io::Write {
}
