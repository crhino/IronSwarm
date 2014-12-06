#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(globs)]
use swarm::{SwarmMsg, SwarmController};
use agent::{SwarmAgent, IronSwarmAgent};
use artifact::{SwarmArtifact, IronSwarmArtifact};
use swarm::network::SwarmNetwork;

pub mod agent;
mod byteid;
pub mod artifact;
mod swarm;

pub trait ReactToSwarm<Loc: Location,
                       Agn: SwarmAgent<Loc>,
                       Art: SwarmArtifact<Loc>>
{
    fn react(&mut self, msg: &SwarmMsg<Loc, Agn, Art>);
}

pub trait Location: Clone {
    fn distance(&self, other: &Self) -> uint;
}

pub struct Swarm<T, Loc> {
    controller: SwarmController<T,
                                Loc,
                                IronSwarmAgent<Loc>,
                                IronSwarmArtifact<Loc>>,
    network: SwarmNetwork<Loc,
                          IronSwarmAgent<Loc>,
                          IronSwarmArtifact<Loc>>
}
