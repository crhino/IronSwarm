#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(globs)]
extern crate "rustc-serialize" as rustc_serialize;
extern crate bincode;

use swarm::{SwarmMsg, SwarmController};
use agent::{SwarmAgent};
use artifact::{SwarmArtifact};
use swarm::network::SwarmNetwork;


pub mod agent;
mod byteid;
pub mod artifact;
mod swarm;

pub trait ReactToSwarm<Loc: Location> {
    fn react(&mut self, msg: &SwarmMsg<Loc>);
}

pub trait Location {
    fn distance(&self, other: &Self) -> uint;
}

pub struct Swarm<T, Loc> {
    controller: SwarmController<T, Loc>,
    network: SwarmNetwork<Loc>
}
