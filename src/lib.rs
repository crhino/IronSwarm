#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(globs)]
use swarm::{SwarmMsg};
use agent::SwarmAgent;
use artifact::SwarmArtifact;

pub mod agent;
pub mod byteid;
pub mod artifact;
pub mod swarm;

pub trait ReactToSwarm<Loc: Location,
                       Agn: SwarmAgent<Loc>,
                       Art: SwarmArtifact<Loc>>
{
    fn react(&self, msg: &SwarmMsg<Loc, Agn, Art>);
}

pub type SwarmLocation<'a> = Box<Location+'a>;

pub trait Location: Clone {
    fn distance(&self, other: &Self) -> uint;
}

#[cfg(test)]
mod tests {
    use Location;
    use ReactToSwarm;
    use swarm::{SwarmMsg};
    use swarm::SwarmEvent::*;
    use agent::IronSwarmAgent;
    use artifact::IronSwarmArtifact;

    // impl Location for int {
    //     fn distance(&self, other: &int) -> uint {
    //         (*self - *other).abs() as uint
    //     }
    // }

    struct Agent;
    impl ReactToSwarm<int,
                      IronSwarmAgent<int>,
                      IronSwarmArtifact<int>>
    for Agent {
        fn react(&self,
            msg: &SwarmMsg<int,
                           IronSwarmAgent<int>,
                           IronSwarmArtifact<int>>) {
            match msg.event() {
                &Artifact(_) => println!("ARTIFACT"),
                &ArtifactGone(_) => println!("ARTIFACT GONE"),
                &AvoidLocation(_) => println!("AVOID LOCATION"),
                &Converge(_) => println!("CONVERGE"),
                &MaliciousAgent(_) => println!("MALICIOUS AGENT")
            }
        }
    }
}
