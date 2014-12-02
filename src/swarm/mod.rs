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

pub enum SwarmMsg<'a, L: Location> {
    HRTBT(Box<SwarmAgent<L>+'a>),
    HRTBTACK(HeartbeatAck<'a, L>),
    JOIN(Box<SwarmAgent<L>+'a>),
    INFO(IronSwarmEvent<'a, L>),
    BROADCAST(IronSwarmEvent<'a, L>)
}

pub enum SwarmEvent<'a, L: Location> {
    Artifact(Box<SwarmArtifact<L>+'a>),
    ArtifactGone(Box<SwarmArtifact<L>+'a>),
    AvoidLocation(Box<Location+'a>),
    Converge(Box<Location+'a>),
    MaliciousAgent(Box<SwarmAgent<L>+'a>)
}

pub struct HeartbeatAck<'a, L: Location> {
    agents: Vec<Box<SwarmAgent<L>+'a>>
}

pub struct IronSwarmEvent<'a, L: Location> {
    from_agent: Box<SwarmAgent<L>+'a>,
    event: SwarmEvent<'a, L>
}
