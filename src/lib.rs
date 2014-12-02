#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(globs)]
use swarm::{SwarmMsg};

pub mod agent;
pub mod byteid;
pub mod artifact;
pub mod swarm;

pub trait ReactToSwarm {
    fn react<'a, L>(&self, msg: &SwarmMsg<'a, L>);
}

pub trait Location {
    fn distance(&self, other: &Self) -> uint;
}

#[cfg(test)]
mod tests {
    use Location;
    use ReactToSwarm;
    use swarm::{SwarmMsg};
    use swarm::SwarmEvent::*;

    struct Agent;
    impl ReactToSwarm for Agent {
        fn react<'a, L>(&self, msg : &SwarmMsg<'a, L>) {
            match msg.event() {
                Artifact(_) => println!("ARTIFACT"),
                ArtifactGone(_) => println!("ARTIFACT GONE"),
                AvoidLocation(_) => println!("AVOID LOCATION"),
                Converge(_) => println!("CONVERGE"),
                MaliciousAgent(_) => println!("MALICIOUS AGENT")
            }
        }
    }
}
