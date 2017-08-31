#![feature(conservative_impl_trait)]

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
#[macro_use]
extern crate slice_as_array;
extern crate protobuf;

#[macro_use]
mod macros;

mod peer;
mod peerinfo;
mod swarm;
pub mod tcp;
mod transport;
mod ping;
mod mux;
mod muxmux;
mod id;
mod identify;

pub use peerinfo::PeerInfo;
pub use swarm::Swarm;
