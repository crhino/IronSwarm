// Swarm Agent
//
// The worker bee of the Swarm framework.
//
// The agent uses the Swarm Network Protocol to communicate with the other swarm
// agents. The operations defined here are mostly abstract operations that should
// be implemented by the user of the framework in accordance with their specific
// use case.
use byteid::ByteId;
use std::io::net::ip::{Ipv4Addr, Ipv6Addr, SocketAddr};
use SwarmSend;

#[deriving(Clone, Eq, PartialEq, Show)]
pub struct SwarmAgent<L> {
    swarm_id: ByteId,
    loc: L,
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

fn u16_to_u8s(n: u16) -> (u8, u8) {
    let lower = n as u8;
    let upper = (n >> 8) as u8;
    (upper, lower)
}

fn u8s_to_u16((u,l): (u8,u8)) -> u16 {
    let upper = (u as u16) << 8;
    let lower = (l as u16);
    upper | lower
}

// fn push_u8_tuple((a,b): (u8, u8), pkt: &mut Vec<u8>) {
//     pkt.push(a); pkt.push(b);
// }

impl SwarmSend for SocketAddr {
    fn swarm_encode(addr: SocketAddr, pkt: &mut Vec<u8>) {
        match addr.ip {
            Ipv4Addr(a,b,c,d) => {
                pkt.push(a);
                pkt.push(b);
                pkt.push(c);
                pkt.push(d);
            }
            Ipv6Addr(_,_,_,_,_,_,_,_) => {
                panic!("not yet implemented");
                // push_u8_tuple(u16_to_u8s(a), pkt);
                // push_u8_tuple(u16_to_u8s(b), pkt);
                // push_u8_tuple(u16_to_u8s(c), pkt);
                // push_u8_tuple(u16_to_u8s(d), pkt);
                // push_u8_tuple(u16_to_u8s(e), pkt);
                // push_u8_tuple(u16_to_u8s(f), pkt);
                // push_u8_tuple(u16_to_u8s(g), pkt);
                // push_u8_tuple(u16_to_u8s(h), pkt);
            }
        }
        let (u, l) = u16_to_u8s(addr.port);
        pkt.push(u);
        pkt.push(l);
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, SocketAddr) {
        let p = u8s_to_u16((pkt[4], pkt[5]));
        (6, SocketAddr { ip: Ipv4Addr(pkt[0], pkt[1], pkt[2], pkt[3]), port: p })
    }
}

impl<L: SwarmSend + Clone> SwarmSend for SwarmAgent<L> {
    fn swarm_encode(agn: SwarmAgent<L>, pkt: &mut Vec<u8>) {
        SwarmSend::swarm_encode(agn.id().clone(), pkt);
        SwarmSend::swarm_encode(agn.location().clone(), pkt);
        SwarmSend::swarm_encode(agn.address().clone(), pkt);
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, SwarmAgent<L>) {
        let mut pkt_ptr = 0;
        let (loc_idx, id) = SwarmSend::swarm_decode(pkt);
        pkt_ptr += loc_idx;
        let (addr_idx, loc) = SwarmSend::swarm_decode(pkt.slice_from(loc_idx));
        pkt_ptr += addr_idx;
        let (end, addr) = SwarmSend::swarm_decode(pkt.slice_from(loc_idx + addr_idx));
        pkt_ptr += end;

        (pkt_ptr, SwarmAgent {
            swarm_id: id,
            loc: loc,
            addr: addr
        })
    }
}

#[cfg(test)]
mod tests {
    use agent::{SwarmAgent};
    use Location;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::num::SignedInt;
    use std::vec::Vec;
    use std::path::BytesContainer;
    use super::{u16_to_u8s, u8s_to_u16};
    use SwarmSend;

    impl Location for int {
        fn distance(&self, other: &int) -> uint {
            (*self - *other).abs() as uint
        }
    }

    impl SwarmSend for int {
        // Naive impl that just truncates the into into a u8
        fn swarm_encode(n: int, pkt: &mut Vec<u8>) {
            pkt.push(n as u8);
        }

        fn swarm_decode(pkt: &[u8]) -> (uint, int) {
            let n = pkt[0] as int;
            (1, n)
        }
    }

    #[test]
    fn u16_to_u8s_test() {
        let n = 0b1111_0000_0000_1111;
        let (a,b) = u16_to_u8s(n);
        assert_eq!(a, 0b1111_0000);
        assert_eq!(b, 0b0000_1111);
    }

    #[test]
    fn u8s_to_u16_test() {
        let u = 0b1111_0000;
        let l = 0b0000_1111;
        let n = u8s_to_u16((u,l));
        assert_eq!(n, 0b1111_0000_0000_1111);
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

    #[test]
    fn swarm_send_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;

        let agent = SwarmAgent::new(loc, addr);
        let mut vec = Vec::new();
        let encoded = SwarmSend::swarm_encode(agent, &mut vec);
        let (_, dec_agnt): (uint, SwarmAgent<int>) =
                            SwarmSend::swarm_decode(vec.container_as_bytes());

        assert_eq!(agent.address(), dec_agnt.address());
        assert_eq!(agent.location(), dec_agnt.location());
        assert_eq!(agent.id(), dec_agnt.id());
    }
}
