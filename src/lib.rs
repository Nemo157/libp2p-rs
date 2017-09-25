#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(proc_macro)]

extern crate slog;
extern crate bytes;
pub extern crate libp2p_identity as identity;
extern crate libp2p_secio as secio;
extern crate maddr;
extern crate multistream;
extern crate msgio;
extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate mplex;
extern crate slice_as_array;
extern crate protobuf;

#[macro_use]
mod macros;

mod dht;
mod id;
mod mux;
mod muxmux;
mod pb;
mod peer;
mod peerinfo;
mod ping;
mod service;
mod swarm;
mod transport;
mod tcp;

pub use peerinfo::PeerInfo;
pub use swarm::Swarm;
