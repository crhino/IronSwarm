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
use SwarmSend;

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

#[deriving(Clone, Eq, PartialEq, Show)]
pub enum SwarmEvent<Loc> {
    Artifact(SwarmArtifact<Loc>),
    ArtifactGone(SwarmArtifact<Loc>),
    AvoidLocation(Loc),
    Converge(Loc),
    MaliciousAgent(SwarmAgent<Loc>)
}

const ART_MAGIC: u8 = 0b0001_0000;
const ARTGONE_MAGIC: u8 = 0b0011_0000;
const AVDLOC_MAGIC: u8 = 0b0100_0000;
const CONV_MAGIC: u8 = 0b0101_0100;
const MALAGN_MAGIC: u8 = 0b0000_0000;
const EVT_END_MAGIC: u8 = 0b1111_0000;

fn assert_end_magic(pkt_end: u8) {
    assert!(pkt_end == EVT_END_MAGIC,
            "Could not find EVT_END_MAGIC value, unknown decoded values: {}", pkt_end)
}

impl<Loc: SwarmSend + Clone> SwarmSend for SwarmEvent<Loc> {
    fn swarm_encode(evt: SwarmEvent<Loc>, pkt: &mut Vec<u8>) {
        match evt {
            SwarmEvent::Artifact(art) => {
                pkt.push(ART_MAGIC);
                SwarmSend::swarm_encode(art, pkt);
            }
            SwarmEvent::ArtifactGone(art) => {
                pkt.push(ARTGONE_MAGIC);
                SwarmSend::swarm_encode(art, pkt);
            }
            SwarmEvent::AvoidLocation(loc) => {
                pkt.push(AVDLOC_MAGIC);
                SwarmSend::swarm_encode(loc, pkt);
            }
            SwarmEvent::Converge(loc) => {
                pkt.push(CONV_MAGIC);
                SwarmSend::swarm_encode(loc, pkt);
            }
            SwarmEvent::MaliciousAgent(agn) => {
                pkt.push(MALAGN_MAGIC);
                SwarmSend::swarm_encode(agn, pkt);
            }
        }
        pkt.push(EVT_END_MAGIC)
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, SwarmEvent<Loc>) {
        let mut pkt_ptr = 0;
        let magic = pkt[pkt_ptr];
        pkt_ptr += 1;

        match magic {
            ART_MAGIC => {
                let (idx, art): (uint, SwarmArtifact<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, SwarmEvent::Artifact(art))
            }
            ARTGONE_MAGIC => {
                let (idx, art): (uint, SwarmArtifact<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, SwarmEvent::ArtifactGone(art))
            }
            AVDLOC_MAGIC => {
                let (idx, loc): (uint, Loc) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, SwarmEvent::AvoidLocation(loc))
            }
            CONV_MAGIC => {
                let (idx, loc): (uint, Loc) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;

                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, SwarmEvent::Converge(loc))
            }
            MALAGN_MAGIC => {
                let (idx, agn): (uint, SwarmAgent<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, SwarmEvent::MaliciousAgent(agn))
            }
            _ => panic!("Unkown magic number: {}", magic)
        }
    }
}

#[deriving(Clone, Eq, PartialEq, Show)]
pub struct SwarmMsg<Loc> {
    from_agent: SwarmAgent<Loc>,
    event: SwarmEvent<Loc>
}

impl<Loc: SwarmSend + Clone> SwarmSend for SwarmMsg<Loc> {
    fn swarm_encode(msg: SwarmMsg<Loc>, pkt: &mut Vec<u8>) {
        SwarmSend::swarm_encode(msg.from_agent, pkt);
        SwarmSend::swarm_encode(msg.event, pkt);
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, SwarmMsg<Loc>) {
        let mut pkt_ptr = 0;
        let (idx, agn): (uint, SwarmAgent<Loc>) =
                         SwarmSend::swarm_decode(pkt);
        pkt_ptr += idx;

        let (idx, evt): (uint, SwarmEvent<Loc>) =
                         SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
        pkt_ptr += idx;
        (pkt_ptr, SwarmMsg { from_agent: agn, event: evt })
    }
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
mod tests {
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

    const ART_LOC: int = 1 << 0;
    const AGN_LOC: int = 1 << 1;
    const AVD_LOC: int = 1 << 2;
    const CNV_LOC: int = 1 << 3;

    struct Tester {
        react_writer: PipeStream
    }

    impl ReactToSwarm<int> for Tester {
        fn react(&mut self,
            msg: &SwarmMsg<int>) {
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

    fn swarm_tester() -> (SwarmController<Tester,int>, PipeStream)
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
