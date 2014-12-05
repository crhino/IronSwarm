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

    const ART_EVENT_RECV: u8 = 1 << 0;
    const ART_GONE_EVENT_RECV: u8 = 1 << 1;
    const AVOID_LOC_EVENT_RECV: u8 = 1 << 2;
    const CONV_EVENT_RECV: u8 = 1 << 3;
    const MAL_AGN_EVENT_RECV: u8 = 1 << 4;

    const ART_LOC: int = 1 << 0;
    const AGN_LOC: int = 1 << 1;
    const AVD_LOC: int = 1 << 2;
    const CNV_LOC: int = 1 << 3;

    struct Tester {
        react_writer: PipeStream
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
                    assert_eq!(*art.location(), ART_LOC);
                    handle_io_result(self.react_writer.write_u8(ART_EVENT_RECV))
                }
                &ArtifactGone(art) => {
                    assert_eq!(*art.location(), ART_LOC);
                    handle_io_result(self.react_writer.write_u8(ART_GONE_EVENT_RECV))
                }
                &AvoidLocation(loc) => {
                    assert_eq!(loc, AVD_LOC);
                    handle_io_result(self.react_writer.write_u8(AVOID_LOC_EVENT_RECV))
                }
                &Converge(loc) => {
                    assert_eq!(loc, CNV_LOC);
                    handle_io_result(self.react_writer.write_u8(CONV_EVENT_RECV))
                }
                &MaliciousAgent(agn) => {
                    assert_eq!(*agn.location(), AGN_LOC);
                    handle_io_result(self.react_writer.write_u8(MAL_AGN_EVENT_RECV))
                }
            }

            assert_eq!(*msg.from_agent().location(), AGN_LOC);
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

    fn swarm_tester() -> (
        Swarm<Tester, int, IronSwarmAgent<int>, IronSwarmArtifact<int>>,
        PipeStream)
    {
        let mut pair = handle_io_result(PipeStream::pair());
        let tester = Tester { react_writer: pair.writer };
        (Swarm::new(tester), pair.reader)
    }

    #[test]
    fn send_artifact_msg_test() {
        let (mut swarm, mut reader) = swarm_tester();

        let agent = SwarmAgent::new(AGN_LOC, test_addr());
        let artifact = SwarmArtifact::new(ART_LOC);

        swarm.send_artifact(agent, artifact);
        assert_eq!(handle_io_result(reader.read_byte()), ART_EVENT_RECV);
    }

    #[test]
    fn send_artifact_gone_msg_test() {
        let (mut swarm, mut reader) = swarm_tester();

        let agent = SwarmAgent::new(AGN_LOC, test_addr());
        let artifact = SwarmArtifact::new(ART_LOC);

        swarm.send_artifact_gone(agent, artifact);
        assert_eq!(handle_io_result(reader.read_byte()), ART_GONE_EVENT_RECV);
    }

    #[test]
    fn send_avoid_location_msg_test() {
        let (mut swarm, mut reader) = swarm_tester();

        let agent = SwarmAgent::new(AGN_LOC, test_addr());

        swarm.send_avoid_location(agent, AVD_LOC);
        assert_eq!(handle_io_result(reader.read_byte()), AVOID_LOC_EVENT_RECV);
    }

    #[test]
    fn send_converge_msg_test() {
        let (mut swarm, mut reader) = swarm_tester();

        let agent = SwarmAgent::new(AGN_LOC, test_addr());

        swarm.send_converge(agent, CNV_LOC);
        assert_eq!(handle_io_result(reader.read_byte()), CONV_EVENT_RECV);
    }

    #[test]
    fn send_malicious_agent_msg_test() {
        let (mut swarm, mut reader) = swarm_tester();

        let agent = SwarmAgent::new(AGN_LOC, test_addr());
        let mal_agent = SwarmAgent::new(AGN_LOC, test_addr());

        swarm.send_malicious_agent(agent, mal_agent);
        assert_eq!(handle_io_result(reader.read_byte()), MAL_AGN_EVENT_RECV);
    }
}
