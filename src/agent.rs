// Swarm Agent
//
// The worker bee of the Swarm framework.
//
// The agent uses the Swarm Network Protocol to communicate with the other swarm
// agents. The operations defined here are mostly abstract operations that should
// be implemented by the user of the framework in accordance with their specific
// use case.
use byteid::ByteId;
use std::io::net::ip::SocketAddr;

pub struct SwarmAgent<L> {
    loc: L,
    swarm_id: ByteId,
    addr: SocketAddr
}

impl<L> SwarmAgent<L> {
    pub fn new(location: L, address: SocketAddr) -> SwarmAgent<L> {
        SwarmAgent {
            loc: location,
            swarm_id: ByteId::random_id(),
            addr: address
        }
    }

    pub fn location(&self) -> &L {
       &self.loc
    }

    pub fn update_location(&mut self, location: L) {
        self.loc = location;
    }

    pub fn id(&self) -> &ByteId {
        &self.swarm_id
    }

    pub fn address(&self) -> SocketAddr {
        self.addr
    }
}

#[cfg(test)]
mod tests {
    use agent::{SwarmAgent};
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
        let loc = 9i;

        let agent = SwarmAgent::new(loc, addr);
    }

    #[test]
    fn location_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;

        let agent = SwarmAgent::new(loc, addr);
        assert_eq!(agent.location(), &loc);
    }

    #[test]
    fn update_location_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;
        let upd_loc = -9i;

        let mut agent = SwarmAgent::new(loc, addr);
        agent.update_location(upd_loc);
        assert_eq!(agent.location(), &upd_loc);
    }

    #[test]
    fn id_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;

        let agent = SwarmAgent::new(loc, addr);
        assert_eq!(agent.id(), &agent.swarm_id);
    }

    #[test]
    fn address_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;

        let agent = SwarmAgent::new(loc, addr);
        assert_eq!(agent.address(), agent.addr);
    }
}
