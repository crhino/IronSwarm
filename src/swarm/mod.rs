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
// use agent::SwarmAgent;
// use artifact::SwarmArtifact;
// use Location;

pub enum SwarmMsg<Ag, Art, L> {
    HRTBT(Ag),
    HRTBTACK(HeartbeatAck<Ag>),
    JOIN(Ag),
    INFO(IronSwarmEvent<Ag, Art, L>),
    BROADCAST(IronSwarmEvent<Ag, Art, L>)
}

pub enum SwarmEvent<Ag, Art, L> {
    Artifact(Art),
    ArtifactGone(Art),
    AvoidLocation(L),
    Converge(L),
    MaliciousAgent(Ag)
}

pub struct HeartbeatAck<A> {
    agents: Vec<A>
}

pub struct IronSwarmEvent<Ag, Art, L> {
    from_agent: Ag,
    event: SwarmEvent<Ag, Art, L>
}
