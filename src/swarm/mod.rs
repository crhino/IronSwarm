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
use agent::SwarmAgent;
use artifact::SwarmArtifact;
use Location;

enum IronSwarmMsg<'a, L> {
    HRTBT(Box<SwarmAgent<L>+'a>),
    HRTBTACK(HeartbeatAck<'a, L>),
    JOIN(Box<SwarmAgent<L>+'a>),
    INFO(SwarmMsg<'a, L>),
    BROADCAST(SwarmMsg<'a, L>)
}

struct HeartbeatAck<'a, L> {
    agents: Vec<Box<SwarmAgent<L>+'a>>
}

pub enum SwarmEvent<'a, L> {
    Artifact(Box<SwarmArtifact<L>+'a>),
    ArtifactGone(Box<SwarmArtifact<L>+'a>),
    AvoidLocation(Box<Location+'a>),
    Converge(Box<Location+'a>),
    MaliciousAgent(Box<SwarmAgent<L>+'a>)
}

pub struct SwarmMsg<'a, L> {
    from_agent: Box<SwarmAgent<L>+'a>,
    event: SwarmEvent<'a, L>
}

impl SwarmMsg<'a, L> {
    fn event(&self) -> SwarmEvent<'a, L> {
        self.event
    }
}
