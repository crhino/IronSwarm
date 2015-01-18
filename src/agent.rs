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

#[derive(Clone, Eq, PartialEq, Show)]
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

#[derive(Clone, Eq, PartialEq, Show, RustcDecodable, RustcEncodable)]
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

impl Encodable for SwarmAddr {
    fn encode<S:Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
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

impl Decodable for SwarmAddr {
    fn decode<D:Decoder>(dec: &mut D) -> Result<SwarmAddr,D::Error> {
        let (a,b,c,d) = (try!(dec.read_u8()),
                         try!(dec.read_u8()),
                         try!(dec.read_u8()),
                         try!(dec.read_u8()));
        let port = try!(dec.read_u16());

        Ok(SwarmAddr(SocketAddr { ip: Ipv4Addr(a,b,c,d), port: port }))
    }
}

#[cfg(test)]
mod test {
    extern crate bincode;

    use agent::{SwarmAgent, SwarmAddr};
    use Location;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::num::SignedInt;

    impl Location for isize {
        fn distance(&self, other: &isize) -> usize {
            (*self - *other).abs() as usize
        }
    }

    #[test]
    fn new_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9is;

        let agent = SwarmAgent::new(loc, addr);
    }

    #[test]
    fn location_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9is;

        let agent = SwarmAgent::new(loc, addr);
        assert_eq!(agent.location(), &loc);
    }

    #[test]
    fn update_location_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9is;
        let upd_loc = -9is;

        let mut agent = SwarmAgent::new(loc, addr);
        agent.update_location(upd_loc);
        assert_eq!(agent.location(), &upd_loc);
    }

    #[test]
    fn id_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9is;

        let agent = SwarmAgent::new(loc, addr);
        assert_eq!(agent.id(), &agent.swarm_id);
    }

    #[test]
    fn address_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9is;

        let agent = SwarmAgent::new(loc, addr);
        assert_eq!(*agent.address(), agent.addr);
    }

    #[test]
    fn bincode_test() {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9is;

        let agent = SwarmAgent::new(loc, addr);
        let limit = bincode::SizeLimit::Infinite;
        let encoded = bincode::encode(&agent, limit).ok().unwrap();
        let dec_agnt: SwarmAgent<isize> =
            bincode::decode(encoded.as_slice()).ok().unwrap();

        assert_eq!(agent.address(), dec_agnt.address());
        assert_eq!(agent.location(), dec_agnt.location());
        assert_eq!(agent.id(), dec_agnt.id());
    }
}
