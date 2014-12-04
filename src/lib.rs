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
    fn react(&mut self, msg: &SwarmMsg<Loc, Agn, Art>);
}

pub trait Location: Clone {
    fn distance(&self, other: &Self) -> uint;
}

#[cfg(test)]
mod tests {
    use Location;
    use ReactToSwarm;
    use swarm::{SwarmMsg, Swarm};
    use swarm::SwarmEvent::*;
    use agent::IronSwarmAgent;
    use artifact::IronSwarmArtifact;
    use std::fmt::Show;
    use std::io::pipe::PipeStream;
    use std::io::IoResult;

    struct Tester {
        artifacts_sent: PipeStream
    }

    impl ReactToSwarm<int,
                      IronSwarmAgent<int>,
                      IronSwarmArtifact<int>>
    for Tester {
        fn react(&mut self,
            msg: &SwarmMsg<int,
                           IronSwarmAgent<int>,
                           IronSwarmArtifact<int>>) {
            match msg.event() {
                &Artifact(_) => {
                    handle_io_result(self.artifacts_sent.write_u8(1))
                }
                &ArtifactGone(_) => println!("ARTIFACT GONE"),
                &AvoidLocation(_) => println!("AVOID LOCATION"),
                &Converge(_) => println!("CONVERGE"),
                &MaliciousAgent(_) => println!("MALICIOUS AGENT")
            }
        }
    }

    fn handle_io_result<T>(res: IoResult<T>) -> T {
        match res {
            Ok(ret) => ret,
            Err(err) => panic!("{}\n", err)
        }
    }

    #[test]
    fn send_artifact_msg_test() {
        let mut pair = handle_io_result(PipeStream::pair());
        let tester = Tester { artifacts_sent: pair.writer };
        let mut swarm = Swarm::new(tester);
        swarm.send_artifact(9);
        assert_eq!(handle_io_result(pair.reader.read_byte()), 1);
    }
}
