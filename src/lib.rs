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
    use agent::{SwarmAgent, IronSwarmAgent};
    use artifact::{SwarmArtifact, IronSwarmArtifact};
    use std::io::pipe::PipeStream;
    use std::io::IoResult;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};

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
                &Artifact(art) => {
                    assert_eq!(*art.location(), 9);
                    handle_io_result(self.artifacts_sent.write_u8(0))
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

    fn test_addr() -> SocketAddr {
        SocketAddr{ ip: Ipv4Addr(127,0,0,0), port: 55555 }
    }

    #[test]
    fn send_artifact_msg_test() {
        let mut pair = handle_io_result(PipeStream::pair());
        let tester = Tester { artifacts_sent: pair.writer };
        let mut swarm = Swarm::new(tester);
        let agent = SwarmAgent::new(9, test_addr());
        let artifact = SwarmArtifact::new(9);

        swarm.send_artifact(agent, artifact);
        assert_eq!(handle_io_result(pair.reader.read_byte()), 0);
    }
}
