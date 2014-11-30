// Swarm Agent
//
// The worker bee of the Swarm framework.
//
// The agent uses the Swarm Network Protocol to communicate with the other swarm
// agents. The operations defined here are mostly abstract operations that should
// be implemented by the user of the framework in accordance with their specific
// use case.
use Location;
use byteid::ByteId;
use std::io::net::ip::SocketAddr;

pub trait ReactToSwarm {

}

pub trait SwarmAgent {
    fn location<T: Location>(&self) -> T;
    fn update_location<T: Location>(&mut self, T);
    fn id(&self) -> ByteId;
    fn addr(&self) -> SocketAddr;
}

struct IronSwarmAgent<T: Location> {
    location: T,
    id: ByteId,
    addr: SocketAddr
}
