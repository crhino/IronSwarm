// Swarm Agent
//
// The worker bee of the Swarm framework.
//
// The agent uses the Swarm Network Protocol to communicate with the other swarm
// agents. The operations defined here are mostly abstract operations that should
// be implemented by the user of the framework in accordance with their specific
// use case.
use Location;
use self::byteid::ByteId;
mod byteid;

pub trait SwarmAgent<T: Location> {
    fn update_location(&mut self, T);
}

pub struct IronSwarmAgent<T> {
    pub location: T,
    id: ByteId
}
