use std::vec::Vec;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use SwarmSend;
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

#[deriving(Clone, Eq, PartialEq, Show)]
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

// Serializes an IronSwarmRPC into a byte array [u8] and back.
impl<Loc: SwarmSend + Clone> SwarmSend for IronSwarmRPC<Loc> {
    fn swarm_encode(rpc: IronSwarmRPC<Loc>, pkt: &mut Vec<u8>) {
        match rpc {
            IronSwarmRPC::HRTBT(agent) => {
                pkt.push(HRTBT_MAGIC);
                SwarmSend::swarm_encode(agent, pkt);
            }
            IronSwarmRPC::HRTBTACK(agents) => {
                pkt.push(HRTBTACK_MAGIC);
                pkt.push(agents.len() as u8);
                for agn in agents.into_iter() {
                    SwarmSend::swarm_encode(agn, pkt);
                }
            }
            IronSwarmRPC::JOIN(agent) => {
                pkt.push(JOIN_MAGIC);
                SwarmSend::swarm_encode(agent, pkt);
            }
            IronSwarmRPC::INFO(send_to, msg) => {
                pkt.push(INFO_MAGIC);
                SwarmSend::swarm_encode(send_to, pkt);
                SwarmSend::swarm_encode(msg, pkt);
            }
            IronSwarmRPC::BROADCAST(msg) => {
                pkt.push(BROADCAST_MAGIC);
                SwarmSend::swarm_encode(msg, pkt);
            }
        }
        pkt.push(END_MAGIC);
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, IronSwarmRPC<Loc>) {
        let mut pkt_ptr = 0;
        let magic = pkt[pkt_ptr];
        pkt_ptr += 1;

        match magic {
            HRTBT_MAGIC => {
                let (idx, agn): (uint, SwarmAgent<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, IronSwarmRPC::HRTBT(agn))
            }
            HRTBTACK_MAGIC => {
                let len = pkt[pkt_ptr];
                pkt_ptr += 1;

                let mut agnts = Vec::new();
                for _ in range(0, len) {
                    let (idx, agn): (uint, SwarmAgent<Loc>) =
                                     SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                    pkt_ptr += idx;
                    agnts.push(agn);
                }
                assert_end_magic(pkt[pkt_ptr]);

                pkt_ptr += 1;
                (pkt_ptr, IronSwarmRPC::HRTBTACK(agnts))
            }
            JOIN_MAGIC => {
                let (idx, agn): (uint, SwarmAgent<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, IronSwarmRPC::JOIN(agn))
            }
            INFO_MAGIC => {
                let (idx, loc): (uint, Loc) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                let (idx, msg): (uint, SwarmMsg<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;

                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, IronSwarmRPC::INFO(loc, msg))
            }
            BROADCAST_MAGIC => {
                let (idx, msg): (uint, SwarmMsg<Loc>) =
                                 SwarmSend::swarm_decode(pkt.slice_from(pkt_ptr));
                pkt_ptr += idx;
                assert_end_magic(pkt[pkt_ptr]);
                pkt_ptr += 1;

                (pkt_ptr, IronSwarmRPC::BROADCAST(msg))
            }
            _ => panic!("Unkown magic number: {}", magic)
        }
    }
}

#[cfg(test)]
mod test {
    use SwarmSend;
    use agent::{SwarmAgent};
    use artifact::SwarmArtifact;
    use swarm::SwarmMsg;
    use std::path::BytesContainer;
    use Location;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use swarm::network::IronSwarmRPC;
    use std::vec::Vec;


    fn swarm_send_rpc_tester(rpc: IronSwarmRPC<int>) {
        let mut vec = Vec::new();
        let orig_rpc = rpc.clone();
        SwarmSend::swarm_encode(rpc, &mut vec);
        let (_, dec_rpc): (uint, IronSwarmRPC<int>) =
                            SwarmSend::swarm_decode(vec.container_as_bytes());
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
    fn swarm_send_test() {
        let mut ack_vec = Vec::new();
        ack_vec.push(construct_agent());

        swarm_send_rpc_tester(IronSwarmRPC::HRTBT(construct_agent()));
        swarm_send_rpc_tester(IronSwarmRPC::HRTBTACK(ack_vec));
        swarm_send_rpc_tester(IronSwarmRPC::JOIN(construct_agent()));
        swarm_send_rpc_tester(IronSwarmRPC::INFO(10, construct_swarm_msg()));
        swarm_send_rpc_tester(IronSwarmRPC::BROADCAST(construct_swarm_msg()));
    }
}
