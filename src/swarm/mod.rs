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
use std::vec::Vec;
use std::boxed::Box;
use byteid::ByteId;
use agent::{SwarmAgent, IronSwarmAgent};
use artifact::{SwarmArtifact, IronSwarmArtifact};
use Location;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

pub type ReactFn<'a, Loc, Agn, Art> =
    Box<FnMut<(SwarmMsg<Loc, Agn, Art>,),()>+'a>;

pub struct Swarm<'a, Loc, Agn, Art> {
    react: ReactFn<'a, Loc, Agn, Art>
}

impl<'a, Loc: Location, Agn: SwarmAgent<Loc>, Art: SwarmArtifact<Loc>>
     Swarm<'a, Loc, Agn, Art>
{
        pub fn new(func: ReactFn<'a, Loc, Agn, Art>)
        -> Swarm<'a, Loc, Agn, Art>
    {
        Swarm { react: func }
    }

    pub fn send_msg(&mut self, msg: SwarmMsg<Loc, Agn, Art>) {
        self.react.call_mut((msg,));
    }

    pub fn send_artifact(&mut self, loc: Loc) {
        let agn_loc = loc.clone();
        let art: Art = SwarmArtifact::new(loc);

        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };

        let agent: Agn = SwarmAgent::new(agn_loc, addr);

        let msg: SwarmMsg<Loc, Agn, Art> =
            SwarmMsg::new_artifact_msg(agent, art);

        self.send_msg(msg);
    }
}

enum IronSwarmMsg<Loc, Agn, Art> {
    HRTBT(Agn),
    HRTBTACK(HeartbeatAck<Agn>),
    JOIN(Agn),
    INFO(SwarmMsg<Loc, Agn, Art>),
    BROADCAST(SwarmMsg<Loc, Agn, Art>)
}

struct HeartbeatAck<Agn> {
    agents: Vec<Agn>
}

pub enum SwarmEvent<Loc, Agn, Art> {
    Artifact(Art),
    ArtifactGone(Art),
    AvoidLocation(Loc),
    Converge(Loc),
    MaliciousAgent(Agn)
}

pub struct SwarmMsg<Loc, Agn,
                    Art>
{
    from_agent: Agn,
    event: SwarmEvent<Loc, Agn, Art>
}

impl<Loc, Agn, Art>
    SwarmMsg<Loc, Agn, Art>
{
    pub fn new_artifact_msg(agent: Agn, art: Art) -> SwarmMsg<Loc, Agn, Art> {
        SwarmMsg {
            from_agent: agent,
            event: SwarmEvent::Artifact(art)
        }
    }

    pub fn event<'a>(&'a self) -> &'a SwarmEvent<Loc, Agn, Art> {
        &self.event
    }
}
