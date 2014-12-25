// Swarm Agent
//
// The worker bee of the Swarm framework.
//
// The agent uses the Swarm Network Protocol to communicate with the other swarm
// agents. The operations defined here are mostly abstract operations that should
// be implemented by the user of the framework in accordance with their specific
// use case.
use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};
use byteid::ByteId;
use std::vec::Vec;
use std::io::IoResult;
use std::io::net::ip::{Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddr};

#[deriving(Clone, Eq, PartialEq, Show)]
pub struct SwarmAddr(SocketAddr);

impl<'a> ToSocketAddr for &'a SwarmAddr {
    fn to_socket_addr(&self) -> IoResult<SocketAddr> {
        let &&SwarmAddr(addr) = self;
        Ok(addr)
    }

    fn to_socket_addr_all(&self) -> IoResult<Vec<SocketAddr>> {
        let &&SwarmAddr(addr) = self;
        let mut vec = Vec::new();
        vec.push(addr);
        Ok(vec)
    }
}

#[deriving(Clone, Eq, PartialEq, Show, RustcDecodable, RustcEncodable)]
pub struct SwarmAgent<L> {
    swarm_id: ByteId,
    loc: L,
    addr: SwarmAddr
}

impl<L> SwarmAgent<L> {
    pub fn new(location: L, address: SocketAddr) -> SwarmAgent<L> {
        SwarmAgent {
            loc: location,
            swarm_id: ByteId::random_id(),
            addr: SwarmAddr(address)
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

    pub fn address(&self) -> &SwarmAddr {
        &self.addr
    }
}

impl<E, S:Encoder<E>> Encodable<S,E> for SwarmAddr {
    fn encode(&self, s: &mut S) -> Result<(),E> {
        let &SwarmAddr(ref addr) = self;
        match addr.ip {
            Ipv4Addr(a,b,c,d) => {
               try!(s.emit_u8(a));
               try!(s.emit_u8(b));
               try!(s.emit_u8(c));
               try!(s.emit_u8(d));
            }
            Ipv6Addr(_,_,_,_,_,_,_,_) => {
                panic!("not yet implemented");
            }
        }
        s.emit_u16(addr.port)
    }
}

impl<E, D:Decoder<E>> Decodable<D,E> for SwarmAddr {
    fn decode(dec: &mut D) -> Result<SwarmAddr,E> {
        let (a,b,c,d) = (try!(dec.read_u8()),
                         try!(dec.read_u8()),
                         try!(dec.read_u8()),
                         try!(dec.read_u8()));
        let port = try!(dec.read_u16());

        Ok(SwarmAddr(SocketAddr { ip: Ipv4Addr(a,b,c,d), port: port }))
    }
}

#[cfg(test)]
mod tests {
    extern crate bincode;

    use agent::{SwarmAgent, SwarmAddr};
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
        assert_eq!(*agent.address(), agent.addr);
    }

    #[test]
    fn bincode_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;

        let agent = SwarmAgent::new(loc, addr);
        let encoded = bincode::encode(&agent).ok().unwrap();
        let dec_agnt: SwarmAgent<int> =
            bincode::decode(encoded).ok().unwrap();

        assert_eq!(agent.address(), dec_agnt.address());
        assert_eq!(agent.location(), dec_agnt.location());
        assert_eq!(agent.id(), dec_agnt.id());
    }
}
