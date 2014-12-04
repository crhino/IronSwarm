#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(globs, unboxed_closures)]
use swarm::{SwarmMsg};
use agent::SwarmAgent;
use artifact::SwarmArtifact;

pub mod agent;
pub mod byteid;
pub mod artifact;
pub mod swarm;

// pub trait ReactToSwarm<Loc: Location,
//                        Agn: SwarmAgent<Loc>,
//                        Art: SwarmArtifact<Loc>>
// {
//     fn react(&mut self, msg: &SwarmMsg<Loc, Agn, Art>);
// }

pub trait Location: Clone {
    fn distance(&self, other: &Self) -> uint;
}

#[cfg(test)]
mod tests {
    use Location;
    use swarm::{SwarmMsg, Swarm, ReactFn};
    use swarm::SwarmEvent::*;
    use agent::IronSwarmAgent;
    use artifact::IronSwarmArtifact;
    use std::fmt::Show;
    use std::ops::FnMut;

    #[deriving(Show)]
    struct Tester {
        artifacts_sent: int
    }

    impl Tester {
        fn react(&mut self,
                 msg: SwarmMsg<int,
                 IronSwarmAgent<int>,
                 IronSwarmArtifact<int>>) {
            match msg.event() {
                &Artifact(_) => {
                    self.artifacts_sent += 1
                }
                &ArtifactGone(_) => println!("ARTIFACT GONE"),
                &AvoidLocation(_) => println!("AVOID LOCATION"),
                &Converge(_) => println!("CONVERGE"),
                &MaliciousAgent(_) => println!("MALICIOUS AGENT")
            }
        }
    }

    fn create_react_fn<'a>(tester: &'a mut Tester) -> ReactFn<'a, int,
        IronSwarmAgent<int>,
        IronSwarmArtifact<int>>
        {
            (box move |&mut:
             msg: SwarmMsg<int, IronSwarmAgent<int>, IronSwarmArtifact<int>>| {
                 tester.react(msg);
             }) as ReactFn<'a, int, IronSwarmAgent<int>, IronSwarmArtifact<int>>
        }

    #[test]
    fn send_artifact_msg_test() {
        let mut tester = Tester { artifacts_sent: 0 };
        let mut swarm = Swarm::new(create_react_fn(&mut tester));
        swarm.send_artifact(9);
        assert_eq!(tester.artifacts_sent, 1);
    }
}
