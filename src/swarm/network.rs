extern crate serialize;

use serialize::{Decodable, Decoder, Encodable, Encoder};
use std::vec::Vec;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use std::io::net::ip::{ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::{IoResult, IoError, BufReader};
use bincode::{decode_from, encode_into};
use bincode::DecoderReader;
use bincode::EncoderWriter;

#[deriving(Clone, Eq, PartialEq, Show, Decodable, Encodable)]
enum IronSwarmRPC<Loc> {
    HRTBT(SwarmAgent<Loc>),
    HRTBTACK(Vec<SwarmAgent<Loc>>),
    JOIN(SwarmAgent<Loc>),
    INFO(Loc, SwarmMsg<Loc>),
    BROADCAST(SwarmMsg<Loc>),
}

pub struct SwarmNetwork<Loc> {
    recv_buf: [u8, ..1024],
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
            recv_buf: [0u8, ..1024],
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
impl<'a,Loc:Decodable<DecoderReader<'a,BufReader<'a>>, IoError>> SwarmNetwork<Loc> {
    pub fn recv_msg(&'a mut self) -> IoResult<IronSwarmRPC<Loc>> {
        match self.socket.recv_from(&mut self.recv_buf) {
            Ok((amt, _)) => {
                let rpc = try!(decode_from(&mut BufReader::new(&self.recv_buf)));
                Ok(rpc)
            }
            Err(e) => panic!("Could not recv a packet: {}", e)
        }
    }

    fn send_heartbeat(&self, agn: SwarmAgent<Loc>) -> IoResult<()> {
        panic!("not implemented")
    }
}

#[cfg(test)]
mod test {
    extern crate bincode;

    use agent::{SwarmAgent};
    use artifact::SwarmArtifact;
    use swarm::SwarmMsg;
    use Location;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::net::udp::UdpSocket;
    use std::io::IoResult;
    use swarm::network::IronSwarmRPC;
    use std::path::BytesContainer;
    use std::vec::Vec;
    use super::SwarmNetwork;


    fn bincode_rpc_tester(rpc: IronSwarmRPC<int>) {
        let orig_rpc = rpc.clone();
        let encoded = bincode::encode(&rpc).ok().unwrap();
        let dec_rpc: IronSwarmRPC<int> =
            bincode::decode(encoded).ok().unwrap();
        assert_eq!(orig_rpc, dec_rpc);
    }

    fn recv_msg_rpc_tester(network: &mut SwarmNetwork<int>,
                           rpc: IronSwarmRPC<int>) -> IoResult<()> {
        let mut socket = try!(UdpSocket::bind(local_socket_send()));
        let orig_rpc = rpc.clone();
        let encoded = bincode::encode(&rpc).ok().unwrap();
        try!(socket.send_to(encoded.container_as_bytes(), local_socket_swarm()));

        let dec_rpc = try!(network.recv_msg());
        assert_eq!(orig_rpc, dec_rpc);
        Ok(())
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

    fn local_socket_swarm() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 54321 }
    }

    fn local_socket_send() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 44444 }
    }

    fn construct_swarm_network_with_local_socket() -> SwarmNetwork<int> {
        SwarmNetwork::new(27, local_socket_swarm())
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

    #[test]
    fn recv_msg_test() {
        let mut ack_vec = Vec::new();
        ack_vec.push(construct_agent());
        let mut network = construct_swarm_network_with_local_socket();

        let res = recv_msg_rpc_tester(&mut network, IronSwarmRPC::HRTBT(construct_agent()));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut network, IronSwarmRPC::HRTBTACK(ack_vec));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut network, IronSwarmRPC::JOIN(construct_agent()));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut network, IronSwarmRPC::INFO(10, construct_swarm_msg()));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut network, IronSwarmRPC::BROADCAST(construct_swarm_msg()));
        assert!(res.is_ok());
    }
}
