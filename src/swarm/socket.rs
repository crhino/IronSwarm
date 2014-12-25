use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::io::net::ip::{SocketAddr, ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::MemWriter;
use std::io::{IoResult, IoError, BufReader};
use bincode::{decode_from, encode};
use bincode::DecoderReader;
use bincode::EncoderWriter;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use swarm::network::IronSwarmRPC;

pub struct SwarmSocket {
    recv_buf: [u8, ..1024],
    socket: UdpSocket
}

impl SwarmSocket {
    pub fn new<A: ToSocketAddr>(address: A) -> SwarmSocket {
        let mut socket = match UdpSocket::bind(address) {
            Ok(s) => s,
            Err(e) => panic!("Could not bind socket: {}", e)
        };
        SwarmSocket {
            recv_buf: [0u8, ..1024],
            socket: socket,
        }
    }

    pub fn socket_name(&mut self) -> SocketAddr {
        match self.socket.socket_name() {
            Ok(a) => a,
            Err(e) => panic!("Could not get socket name: {}", e)
        }
    }
}

// Implement receiving of IronSwarmRPC through the UDP socket.
impl<'a,Loc:Decodable<DecoderReader<'a,BufReader<'a>>, IoError>> SwarmSocket {
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
impl<'a,Loc:Encodable<EncoderWriter<'a,MemWriter>, IoError>+Clone,A: ToSocketAddr> SwarmSocket {
    fn send_rpc(&'a mut self, rpc: IronSwarmRPC<Loc>, dest: A) -> IoResult<()> {
        let encoded = try!(encode(&rpc));
        try!(self.socket.send_to(encoded.as_slice(), dest));
        Ok(())
    }

    pub fn send_heartbeat(&'a mut self, dest: A, hrtbt: SwarmAgent<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::HRTBT(hrtbt);
        self.send_rpc(rpc, dest)
    }

    pub fn send_heartbeat_ack(&'a mut self, dest: A,
                         neighbors: Vec<SwarmAgent<Loc>>) -> IoResult<()> {
        let rpc = IronSwarmRPC::HRTBTACK(neighbors);
        self.send_rpc(rpc, dest)
    }

    pub fn send_join(&'a mut self, dest: A, join_agn: SwarmAgent<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::JOIN(join_agn);
        self.send_rpc(rpc, dest)
    }

    pub fn send_info(&'a mut self, dest: A, loc: Loc, msg: SwarmMsg<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::INFO(loc, msg);
        self.send_rpc(rpc, dest)
    }

    pub fn send_broadcast(&'a mut self, dest: A, msg: SwarmMsg<Loc>) -> IoResult<()> {
        let rpc = IronSwarmRPC::BROADCAST(msg);
        self.send_rpc(rpc, dest)
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
    use super::SwarmSocket;

    fn bincode_rpc_tester(rpc: IronSwarmRPC<int>) {
        let orig_rpc = rpc.clone();
        let encoded = bincode::encode(&rpc).ok().unwrap();
        let dec_rpc: IronSwarmRPC<int> =
            bincode::decode(encoded).ok().unwrap();
        assert_eq!(orig_rpc, dec_rpc);
    }

    fn recv_msg_rpc_tester(swarm_socket: &mut SwarmSocket,
                           rpc: IronSwarmRPC<int>) -> IoResult<()> {
        let mut socket = try!(UdpSocket::bind(local_socket()));
        let orig_rpc = rpc.clone();
        let encoded = bincode::encode(&rpc).ok().unwrap();
        let net_sock = swarm_socket.socket_name();
        try!(socket.send_to(encoded.container_as_bytes(), net_sock));

        let dec_rpc = try!(swarm_socket.recv_msg());
        assert_eq!(orig_rpc, dec_rpc);
        Ok(())
    }

    fn send_hrtbt_tester(from_socket: &mut SwarmSocket,
                           to_socket: &mut SwarmSocket) -> IoResult<()> {
        let agn = construct_agent();
        let exp_rpc = IronSwarmRPC::HRTBT(agn.clone());

        try!(from_socket.send_heartbeat(to_socket.socket_name(), agn));
        let recv_rpc = try!(to_socket.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_hrtbtack_tester(from_socket: &mut SwarmSocket,
                           to_socket: &mut SwarmSocket) -> IoResult<()> {
        let mut ack_vec = Vec::new();
        ack_vec.push(construct_agent());
        let exp_rpc = IronSwarmRPC::HRTBTACK(ack_vec.clone());

        try!(from_socket.send_heartbeat_ack(to_socket.socket_name(), ack_vec));
        let recv_rpc = try!(to_socket.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_join_tester(from_socket: &mut SwarmSocket,
                           to_socket: &mut SwarmSocket) -> IoResult<()> {
        let agn = construct_agent();
        let exp_rpc = IronSwarmRPC::JOIN(agn.clone());

        try!(from_socket.send_join(to_socket.socket_name(), agn));
        let recv_rpc = try!(to_socket.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_info_tester(from_socket: &mut SwarmSocket,
                           to_socket: &mut SwarmSocket) -> IoResult<()> {
        let loc = 10;
        let msg = construct_swarm_msg();
        let exp_rpc = IronSwarmRPC::INFO(loc.clone(), msg.clone());

        try!(from_socket.send_info(to_socket.socket_name(), loc, msg));
        let recv_rpc = try!(to_socket.recv_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn send_broadcast_tester(from_socket: &mut SwarmSocket,
                           to_socket: &mut SwarmSocket) -> IoResult<()> {
        let msg = construct_swarm_msg();
        let exp_rpc = IronSwarmRPC::BROADCAST(msg.clone());

        try!(from_socket.send_broadcast(to_socket.socket_name(), msg));
        let recv_rpc = try!(to_socket.recv_msg());

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

    fn construct_swarm_socket_with_local_socket() -> SwarmSocket {
        SwarmSocket::new(local_socket())
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
        let mut socket = construct_swarm_socket_with_local_socket();

        let res = recv_msg_rpc_tester(&mut socket, IronSwarmRPC::HRTBT(construct_agent()));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut socket, IronSwarmRPC::HRTBTACK(ack_vec));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut socket, IronSwarmRPC::JOIN(construct_agent()));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut socket, IronSwarmRPC::INFO(10, construct_swarm_msg()));
        assert!(res.is_ok());
        let res = recv_msg_rpc_tester(&mut socket, IronSwarmRPC::BROADCAST(construct_swarm_msg()));
        assert!(res.is_ok());
    }

    #[test]
    fn send_rpc_test() {
        let mut from_socket = construct_swarm_socket_with_local_socket();
        let mut to_socket = construct_swarm_socket_with_local_socket();

        let res = send_hrtbt_tester(&mut from_socket, &mut to_socket);
        assert!(res.is_ok());
        let res = send_hrtbtack_tester(&mut from_socket, &mut to_socket);
        assert!(res.is_ok());
        let res = send_join_tester(&mut from_socket, &mut to_socket);
        assert!(res.is_ok());
        let res = send_info_tester(&mut from_socket, &mut to_socket);
        assert!(res.is_ok());
        let res = send_broadcast_tester(&mut from_socket, &mut to_socket);
        assert!(res.is_ok());
    }
}


