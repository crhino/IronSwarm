// Swarm Peer-to-Peer Networking
//
// Agent's should have knowledge of their neighbors/connections, this module is
// to ensure knowledge transfer through the entire swarm. This module will also
// be used to ensure the self-healing nature of the swarm's network and make sure
// that a partition does not take place. Adding an agent or removing an agent from
// the swarm is also the responsibility of this module.
//
// RPC:
//
// - HRTBT
// - HRTBT_ACK
// - JOIN
// - INFO
// - BROADCAST
use agent::{SwarmAgent};
use artifact::{SwarmArtifact};
use Location;
use ReactToSwarm;

mod socket;
pub mod network;

pub struct SwarmController<T, Loc> {
    actor: T
}

impl<T: ReactToSwarm<Loc>, Loc: Location> SwarmController<T, Loc>
{
    pub fn new(act: T) -> SwarmController<T, Loc> {
        SwarmController { actor: act }
    }

    fn send_msg(&mut self, msg: &SwarmMsg<Loc>) {
        self.actor.react(msg);
    }

    fn send_artifact(&mut self, agent: SwarmAgent<Loc>,
                     art: SwarmArtifact<Loc>) {
        let msg: SwarmMsg<Loc> =
            SwarmMsg::new_artifact_msg(agent, art);

        self.send_msg(&msg);
    }

    fn send_artifact_gone(&mut self, agent: SwarmAgent<Loc>,
                          art: SwarmArtifact<Loc>) {
        let msg: SwarmMsg<Loc> =
            SwarmMsg::new_artifact_gone_msg(agent, art);

        self.send_msg(&msg);
    }

    fn send_avoid_location(&mut self, agent: SwarmAgent<Loc>, loc: Loc) {
        let msg: SwarmMsg<Loc> =
            SwarmMsg::new_avoid_loc_msg(agent, loc);

        self.send_msg(&msg);
    }

    fn send_converge(&mut self, agent: SwarmAgent<Loc>, loc: Loc) {
        let msg: SwarmMsg<Loc> =
            SwarmMsg::new_converge_msg(agent, loc);

        self.send_msg(&msg);
    }

    fn send_malicious_agent(&mut self, agent: SwarmAgent<Loc>,
                                       mal: SwarmAgent<Loc>) {
        let msg: SwarmMsg<Loc> =
            SwarmMsg::new_malicious_agent_msg(agent, mal);

        self.send_msg(&msg);
    }

}

#[derive(Clone, Eq, PartialEq, Show, RustcDecodable, RustcEncodable)]
pub enum SwarmEvent<Loc> {
    Artifact(SwarmArtifact<Loc>),
    ArtifactGone(SwarmArtifact<Loc>),
    AvoidLocation(Loc),
    Converge(Loc),
    MaliciousAgent(SwarmAgent<Loc>)
}

#[derive(Clone, Eq, PartialEq, Show, RustcDecodable, RustcEncodable)]
pub struct SwarmMsg<Loc> {
    from_agent: SwarmAgent<Loc>,
    event: SwarmEvent<Loc>
}

impl<Loc> SwarmMsg<Loc> {
    fn new_artifact_msg(agent: SwarmAgent<Loc>,
                        art: SwarmArtifact<Loc>) -> SwarmMsg<Loc> {
        SwarmMsg {
            from_agent: agent,
            event: SwarmEvent::Artifact(art)
        }
    }

    fn new_artifact_gone_msg(agent: SwarmAgent<Loc>,
                             art: SwarmArtifact<Loc>) -> SwarmMsg<Loc> {
        SwarmMsg {
            from_agent: agent,
            event: SwarmEvent::ArtifactGone(art)
        }
    }

    fn new_avoid_loc_msg(agent: SwarmAgent<Loc>, loc: Loc) -> SwarmMsg<Loc> {
        SwarmMsg {
            from_agent: agent,
            event: SwarmEvent::AvoidLocation(loc)
        }
    }

    fn new_converge_msg(agent: SwarmAgent<Loc>, loc: Loc) -> SwarmMsg<Loc> {
        SwarmMsg {
            from_agent: agent,
            event: SwarmEvent::Converge(loc)
        }
    }

    fn new_malicious_agent_msg(agent: SwarmAgent<Loc>,
                               mal: SwarmAgent<Loc>) -> SwarmMsg<Loc> {
        SwarmMsg {
            from_agent: agent,
            event: SwarmEvent::MaliciousAgent(mal)
        }
    }

    pub fn event<'a>(&'a self) -> &'a SwarmEvent<Loc> {
        &self.event
    }

    pub fn from_agent<'a>(&'a self) -> &'a SwarmAgent<Loc> {
        &self.from_agent
    }
}

#[cfg(test)]
mod test {
    use Location;
    use ReactToSwarm;
    use swarm::{SwarmMsg, SwarmController};
    use swarm::SwarmEvent::*;
    use agent::{SwarmAgent};
    use artifact::{SwarmArtifact};
    use std::io::pipe::PipeStream;
    use std::io::IoResult;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};

    const ART_EVENT_RECV: u8 = 1 << 0;
    const ART_GONE_EVENT_RECV: u8 = 1 << 1;
    const AVOID_LOC_EVENT_RECV: u8 = 1 << 2;
    const CONV_EVENT_RECV: u8 = 1 << 3;
    const MAL_AGN_EVENT_RECV: u8 = 1 << 4;

    const ART_LOC: isize = 1 << 0;
    const AGN_LOC: isize = 1 << 1;
    const AVD_LOC: isize = 1 << 2;
    const CNV_LOC: isize = 1 << 3;

    struct Tester {
        react_writer: PipeStream
    }

    impl ReactToSwarm<isize> for Tester {
        fn react(&mut self,
            msg: &SwarmMsg<isize>) {
            match msg.event() {
                &Artifact(ref art) => {
                    assert_eq!(*art.location(), ART_LOC);
                    handle_io_result(self.react_writer.write_u8(ART_EVENT_RECV))
                }
                &ArtifactGone(ref art) => {
                    assert_eq!(*art.location(), ART_LOC);
                    handle_io_result(self.react_writer.write_u8(ART_GONE_EVENT_RECV))
                }
                &AvoidLocation(ref loc) => {
                    assert_eq!(*loc, AVD_LOC);
                    handle_io_result(self.react_writer.write_u8(AVOID_LOC_EVENT_RECV))
                }
                &Converge(ref loc) => {
                    assert_eq!(*loc, CNV_LOC);
                    handle_io_result(self.react_writer.write_u8(CONV_EVENT_RECV))
                }
                &MaliciousAgent(ref agn) => {
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

    fn swarm_tester() -> (SwarmController<Tester,isize>, PipeStream)
    {
        let pair = handle_io_result(PipeStream::pair());
        let tester = Tester { react_writer: pair.writer };
        (SwarmController::new(tester), pair.reader)
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
