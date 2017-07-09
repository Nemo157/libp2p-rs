#![feature(conservative_impl_trait)]

pub extern crate libp2p_identity as identity;
extern crate libp2p_secio as secio;
extern crate maddr;
extern crate mhash;
extern crate multistream;
extern crate openssl;
extern crate msgio;
extern crate futures;
extern crate futures_mpsc;
extern crate futures_spawn;
extern crate tokio_core;
extern crate relay;
extern crate mplex;

#[macro_use]
mod macros;

mod peer;
mod peerinfo;
mod swarm;
pub mod tcp;
mod transport;

pub use peerinfo::PeerInfo;
pub use swarm::Swarm;
