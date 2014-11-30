// Swarm Agent
//
// The worker bee of the Swarm framework.
//
// The agent uses the Swarm Network Protocol to communicate with the other swarm
// agents. The operations defined here are mostly abstract operations that should
// be implemented by the user of the framework in accordance with their specific
// use case.
use std::boxed::Box;
use Location;
use byteid::ByteId;
use swarm::SwarmMsg;
use std::io::net::ip::SocketAddr;

pub trait ReactToSwarm {
    // fn react(&self, msg: &SwarmEvent);
}

pub trait SwarmAgent<L: Location> {
    fn new(loc: L, addr: SocketAddr) -> Self;
    fn location(&self) -> &L;
    fn update_location(&mut self, loc: L);
    fn id(&self) -> &ByteId;
    fn address(&self) -> SocketAddr;
}

struct IronSwarmAgent<L> {
    loc: L,
    swarm_id: ByteId,
    addr: SocketAddr
}

impl<L: Location> SwarmAgent<L> for IronSwarmAgent<L> {
    fn new(location: L, address: SocketAddr) -> IronSwarmAgent<L> {
        IronSwarmAgent {
            loc: location,
            swarm_id: ByteId::random_id(),
            addr: address
        }
    }

    fn location(&self) -> &L {
       &self.loc
    }

    fn update_location(&mut self, location: L) {
        self.loc = location;
    }

    fn id(&self) -> &ByteId {
        &self.swarm_id
    }

    fn address(&self) -> SocketAddr {
        self.addr
    }
}

#[cfg(test)]
mod tests {
    use agent::{SwarmAgent, IronSwarmAgent};
    use Location;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::num::SignedInt;

    impl Location for int {
        fn distance(&self, other: &int) -> uint {
            (*self - *other).abs() as uint
        }
    }

    #[test]
    fn new_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9;

        let agent: IronSwarmAgent<int> = SwarmAgent::new(loc, addr);
    }

    #[test]
    fn location_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9;

        let agent: IronSwarmAgent<int> = SwarmAgent::new(loc, addr);
        assert_eq!(agent.location(), &loc);
    }

    #[test]
    fn update_location_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9;
        let upd_loc = -9;

        let mut agent: IronSwarmAgent<int> = SwarmAgent::new(loc, addr);
        agent.update_location(upd_loc);
        assert_eq!(agent.location(), &upd_loc);
    }

    #[test]
    fn id_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9;

        let agent: IronSwarmAgent<int> = SwarmAgent::new(loc, addr);
        assert_eq!(agent.id(), &agent.swarm_id);
    }

    #[test]
    fn address_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9;

        let agent: IronSwarmAgent<int> = SwarmAgent::new(loc, addr);
        assert_eq!(agent.address(), agent.addr);
    }
}
