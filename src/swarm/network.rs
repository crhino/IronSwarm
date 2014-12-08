use std::vec::Vec;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use SwarmSend;
use std::io::net::ip::{ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::IoResult;
// use std::hash::RandomSipHasher;
// use std::mem::transmute;

const HRTBT_MAGIC: uint = 1 << 0;
const HRTBTACK_MAGIC: uint = 1 << 1;
const JOIN_MAGIC: uint = 1 << 2;
const INFO_MAGIC: uint = 1 << 3;
const BROADCAST_MAGIC: uint = 1 << 4;

enum IronSwarmRPC<Loc> {
    HRTBT(SwarmAgent<Loc>),
    HRTBTACK(Vec<SwarmAgent<Loc>>),
    JOIN(SwarmAgent<Loc>),
    INFO(Loc, SwarmMsg<Loc>),
    BROADCAST(SwarmMsg<Loc>)
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

// Serializes an IronSwarmRPC into a byte array [u8] and back.
impl<Loc> SwarmSend for IronSwarmRPC<Loc> {
    fn swarm_encode(rpc: IronSwarmRPC<Loc>, packet: &mut Vec<u8>) {
        match rpc {
            IronSwarmRPC::HRTBT(agent) => {}
            IronSwarmRPC::HRTBTACK(agents) => {}
            IronSwarmRPC::JOIN(agent) => {}
            IronSwarmRPC::INFO(send_to, msg) => {}
            IronSwarmRPC::BROADCAST(msg) => {}
        }
    }

    fn swarm_decode(packet: &[u8]) -> (uint, IronSwarmRPC<Loc>) {
        panic!("not yet implemented")
    }
}
