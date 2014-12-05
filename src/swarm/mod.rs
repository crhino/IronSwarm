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
use byteid::ByteId;
use agent::{SwarmAgent, IronSwarmAgent};
use artifact::{SwarmArtifact, IronSwarmArtifact};
use Location;
use ReactToSwarm;

pub struct Swarm<T, Loc, Agn, Art> {
    actor: T
}

impl<T: ReactToSwarm<Loc, Agn, Art>, Loc: Location,
     Agn: SwarmAgent<Loc>, Art: SwarmArtifact<Loc>>
     Swarm<T, Loc, Agn, Art>
{
    pub fn new(act: T) -> Swarm<T, Loc, Agn, Art> {
        Swarm { actor: act }
    }

    fn send_msg(&mut self, msg: &SwarmMsg<Loc, Agn, Art>) {
        self.actor.react(msg);
    }

    pub fn send_artifact(&mut self, agent: Agn, art: Art) {
        let msg: SwarmMsg<Loc, Agn, Art> =
            SwarmMsg::new_artifact_msg(agent, art);

        self.send_msg(&msg);
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
