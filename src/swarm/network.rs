extern crate serialize;

use serialize::{Decodable, Decoder, Encodable, Encoder};
use std::vec::Vec;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use std::io::net::ip::{SocketAddr, ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::MemWriter;
use std::io::{IoResult, IoError, BufReader};
use bincode::{decode_from, encode};
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

    pub fn socket_name(&mut self) -> IoResult<SocketAddr> {
        self.socket.socket_name()
    }

    pub fn update_location(&mut self, location: Loc) {
        self.local_agent.update_location(location);
    }
}

// Implement receiving of IronSwarmRPC through the UDP socket.
impl<'a,Loc:Decodable<DecoderReader<'a,BufReader<'a>>, IoError>> SwarmNetwork<Loc> {
    pub fn recv_msg(&'a mut self) -> IoResult<IronSwarmRPC<Loc>> {
        match self.socket.recv_from(&mut self.recv_buf) {
            Ok((amt, _)) => {
                let rpc = try!(decode_from(&mut BufReader::new(self.recv_buf.slice_to(amt))));
                Ok(rpc)
            }
            Err(e) => panic!("Could not recv a packet: {}", e)
        }
    }
}

// Implement sending of IronSwarmRPC through the UDP socket.
impl<'a,Loc:Encodable<EncoderWriter<'a,MemWriter>, IoError>+Clone> SwarmNetwork<Loc> {
    fn send_rpc(&'a mut self, rpc: IronSwarmRPC<Loc>, to_agn: SwarmAgent<Loc>) -> IoResult<()> {
        let encoded = try!(encode(&rpc));
        try!(self.socket.send_to(encoded.as_slice(), to_agn.address()));
        Ok(())
    }

    fn send_heartbeat(&'a mut self, to_agn: SwarmAgent<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::HRTBT(self.local_agent.clone());
        self.send_rpc(rpc, to_agn)
    }

    fn send_heartbeat_ack(&'a mut self, to_agn: SwarmAgent<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::HRTBTACK(self.neighbors.clone());
        self.send_rpc(rpc, to_agn)
    }

    fn send_join(&'a mut self, to_agn: SwarmAgent<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::JOIN(self.local_agent.clone());
        self.send_rpc(rpc, to_agn)
    }

    fn send_info(&'a mut self, to_agn: SwarmAgent<Loc>, loc: Loc, msg: SwarmMsg<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::INFO(loc, msg);
        self.send_rpc(rpc, to_agn)
    }

    fn send_broadcast(&'a mut self, to_agn: SwarmAgent<Loc>, msg: SwarmMsg<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::BROADCAST(msg);
        self.send_rpc(rpc, to_agn)
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
    use std::io::test::next_test_port;
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
        let mut socket = try!(UdpSocket::bind(local_socket()));
        let orig_rpc = rpc.clone();
        let encoded = bincode::encode(&rpc).ok().unwrap();
        let net_sock = try!(network.socket_name());
        try!(socket.send_to(encoded.container_as_bytes(), net_sock));

        let dec_rpc = try!(network.recv_msg());
        assert_eq!(orig_rpc, dec_rpc);
        Ok(())
    }

    fn send_hrtbt_tester(from_network: &mut SwarmNetwork<int>,
                           to_network: &mut SwarmNetwork<int>) -> IoResult<()> {
        let exp_rpc = IronSwarmRPC::HRTBT(from_network.local_agent.clone());

        try!(from_network.send_heartbeat(to_network.local_agent.clone()));
        let recv_rpc = try!(to_network.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_hrtbtack_tester(from_network: &mut SwarmNetwork<int>,
                           to_network: &mut SwarmNetwork<int>) -> IoResult<()> {
        let exp_rpc = IronSwarmRPC::HRTBTACK(from_network.neighbors.clone());

        try!(from_network.send_heartbeat_ack(to_network.local_agent.clone()));
        let recv_rpc = try!(to_network.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_join_tester(from_network: &mut SwarmNetwork<int>,
                           to_network: &mut SwarmNetwork<int>) -> IoResult<()> {
        let exp_rpc = IronSwarmRPC::JOIN(from_network.local_agent.clone());

        try!(from_network.send_join(to_network.local_agent.clone()));
        let recv_rpc = try!(to_network.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_info_tester(from_network: &mut SwarmNetwork<int>,
                           to_network: &mut SwarmNetwork<int>) -> IoResult<()> {
        let msg = construct_swarm_msg();
        let exp_rpc = IronSwarmRPC::INFO(27, msg.clone());

        try!(from_network.send_info(to_network.local_agent.clone(), 27, msg));
        let recv_rpc = try!(to_network.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_broadcast_tester(from_network: &mut SwarmNetwork<int>,
                           to_network: &mut SwarmNetwork<int>) -> IoResult<()> {
        let msg = construct_swarm_msg();
        let exp_rpc = IronSwarmRPC::BROADCAST(msg.clone());

        try!(from_network.send_broadcast(to_network.local_agent.clone(), msg));
        let recv_rpc = try!(to_network.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn construct_artifact() -> SwarmArtifact<int> {
        let loc = 9i;

        SwarmArtifact::new(loc)
    }

    fn construct_agent() -> SwarmAgent<int> {
        SwarmAgent::new(9, local_socket())
    }

    fn construct_swarm_msg() -> SwarmMsg<int> {
        SwarmMsg::new_artifact_msg(construct_agent(), construct_artifact())
    }

    fn local_socket() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: next_test_port() }
    }

    fn construct_swarm_network_with_local_socket() -> SwarmNetwork<int> {
        SwarmNetwork::new(27, local_socket())
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

    #[test]
    fn send_hrtbt_test() {
        let mut from_network = construct_swarm_network_with_local_socket();
        let mut to_network = construct_swarm_network_with_local_socket();

        let res = send_hrtbt_tester(&mut from_network, &mut to_network);
        assert!(res.is_ok());
    }

    #[test]
    fn send_hrtbtack_test() {
        let mut from_network = construct_swarm_network_with_local_socket();
        let mut to_network = construct_swarm_network_with_local_socket();

        let res = send_hrtbtack_tester(&mut from_network, &mut to_network);
        assert!(res.is_ok());
    }

    #[test]
    fn send_join_test() {
        let mut from_network = construct_swarm_network_with_local_socket();
        let mut to_network = construct_swarm_network_with_local_socket();

        let res = send_join_tester(&mut from_network, &mut to_network);
        assert!(res.is_ok());
    }

    #[test]
    fn send_info_test() {
        let mut from_network = construct_swarm_network_with_local_socket();
        let mut to_network = construct_swarm_network_with_local_socket();

        let res = send_info_tester(&mut from_network, &mut to_network);
        assert!(res.is_ok());
    }

    #[test]
    fn send_broadcast_test() {
        let mut from_network = construct_swarm_network_with_local_socket();
        let mut to_network = construct_swarm_network_with_local_socket();

        let res = send_broadcast_tester(&mut from_network, &mut to_network);
        assert!(res.is_ok());
    }
}
