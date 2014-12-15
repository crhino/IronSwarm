extern crate serialize;

use std::vec::Vec;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use std::io::net::ip::{ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::IoResult;
// use std::hash::RandomSipHasher;
// use std::mem::transmute;

const HRTBT_MAGIC: u8 = 0b0000_0001;
const HRTBTACK_MAGIC: u8 = 0b0000_0010;
const JOIN_MAGIC: u8 = 0b0000_0011;
const INFO_MAGIC: u8 = 0b0000_0100;
const BROADCAST_MAGIC: u8 = 0b0000_0101;
const END_MAGIC: u8 = 0b1111_1111;

#[deriving(Clone, Eq, PartialEq, Show, Decodable, Encodable)]
enum IronSwarmRPC<Loc> {
    HRTBT(SwarmAgent<Loc>),
    HRTBTACK(Vec<SwarmAgent<Loc>>),
    JOIN(SwarmAgent<Loc>),
    INFO(Loc, SwarmMsg<Loc>),
    BROADCAST(SwarmMsg<Loc>),
}

pub struct SwarmNetwork<Loc> {
    socket: UdpSocket,
    local_agent: SwarmAgent<Loc>,
    neighbors: Vec<SwarmAgent<Loc>>
}

// Public interface of the Swarm Network.
impl<Loc> SwarmNetwork<Loc> {
    pub fn new<A: ToSocketAddr>(loc: Loc, address: A) -> SwarmNetwork<Loc> {
        let mut socket = match UdpSocket::bind(address) {
            Ok(s) => s,
            Err(e) => panic!("Could not bind socket: {}", e)
        };
        let addr = match socket.socket_name() {
            Ok(a) => a,
            Err(e) => panic!("Could not get socket name: {}", e)
        };
        let agent = SwarmAgent::new(loc, addr);

        SwarmNetwork {
            socket: socket,
            local_agent: agent,
            neighbors: Vec::new()
        }
    }

    pub fn update_location(&mut self, location: Loc) {
        self.local_agent.update_location(location);
    }
}

// Implement sending and receiving of IronSwarmRPC through the UDP socket.
impl<Loc> SwarmNetwork<Loc> {
    fn recv_msg(&self) -> IoResult<IronSwarmRPC<Loc>> {
        panic!("not implemented")
    }

    fn send_heartbeat(&self, agn: SwarmAgent<Loc>) -> IoResult<()> {
        panic!("not implemented")
    }
}

fn assert_end_magic(pkt_end: u8) {
    assert!(pkt_end == END_MAGIC,
            "Could not find END_MAGIC value, unknown decoded values: {}", pkt_end)
}

#[cfg(test)]
mod test {
    extern crate bincode;

    use agent::{SwarmAgent};
    use artifact::SwarmArtifact;
    use swarm::SwarmMsg;
    use std::path::BytesContainer;
    use Location;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use swarm::network::IronSwarmRPC;
    use std::vec::Vec;


    fn bincode_rpc_tester(rpc: IronSwarmRPC<int>) {
        let orig_rpc = rpc.clone();
        let encoded = bincode::encode(&rpc).ok().unwrap();
        println!("ENCODED: {}", encoded);
        let dec_rpc: IronSwarmRPC<int> =
            bincode::decode(encoded).ok().unwrap();
        println!("DECODED: {}", dec_rpc);
        assert_eq!(orig_rpc, dec_rpc);
    }

    fn construct_artifact() -> SwarmArtifact<int> {
        let loc = 9i;

        SwarmArtifact::new(loc)
    }

    fn construct_agent() -> SwarmAgent<int> {
        let ipaddr = Ipv4Addr(127, 0, 0, 0);
        let p = 1234;
        let addr = SocketAddr{ ip: ipaddr, port: p };
        let loc = 9i;

        SwarmAgent::new(loc, addr)
    }

    fn construct_swarm_msg() -> SwarmMsg<int> {
        SwarmMsg::new_artifact_msg(construct_agent(), construct_artifact())
    }

    #[test]
    fn bincode_test() {
        let mut ack_vec = Vec::new();
        ack_vec.push(construct_agent());

        bincode_rpc_tester(IronSwarmRPC::HRTBT(construct_agent()));
        bincode_rpc_tester(IronSwarmRPC::HRTBTACK(ack_vec));
        bincode_rpc_tester(IronSwarmRPC::JOIN(construct_agent()));
        bincode_rpc_tester(IronSwarmRPC::INFO(10, construct_swarm_msg()));
        bincode_rpc_tester(IronSwarmRPC::BROADCAST(construct_swarm_msg()));
    }
}
