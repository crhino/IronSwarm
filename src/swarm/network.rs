use rustc_serialize::{Decodable, Encodable};
use std::vec::Vec;
use swarm::socket::{SwarmResult, SwarmSocket};
use swarm::SwarmMsg;
use agent::{SwarmAddr, SwarmAgent};
use Location;
use std::io::net::ip::{SocketAddr, ToSocketAddr};
use bincode::DecoderReader;
use bincode::EncoderWriter;
use std::io::{BufReader};
use std::io::MemWriter;
use std::fmt::Show;

#[derive(Clone, Eq, PartialEq, Show, RustcDecodable, RustcEncodable)]
pub enum IronSwarmRPC<Loc> {
    HRTBT(SwarmAgent<Loc>),
    HRTBTACK(Vec<SwarmAgent<Loc>>),
    JOIN(SwarmAgent<Loc>),
    INFO(Loc, SwarmMsg<Loc>),
    BROADCAST(SwarmMsg<Loc>),
}

pub struct SwarmNetwork<Loc> {
    socket: SwarmSocket,
    local_agent: SwarmAgent<Loc>,
    neighbors: Vec<SwarmAgent<Loc>>
}

impl<Loc> SwarmNetwork<Loc> {
    fn new<A: ToSocketAddr>(loc: Loc, address: A) -> SwarmNetwork<Loc> {
        let mut socket = SwarmSocket::new(address);
        let addr = socket.socket_name();
        let agent = SwarmAgent::new(loc, addr);

        SwarmNetwork {
            socket: socket,
            local_agent: agent,
            neighbors: Vec::new()
        }
    }

    fn address(&mut self) -> SocketAddr {
        self.socket.socket_name()
    }

    fn update_location(&mut self, location: Loc) {
        self.local_agent.update_location(location);
    }
}

// Send RPC's.
impl<Loc: Encodable + Clone> SwarmNetwork<Loc> {
    fn send_heartbeat<A: ToSocketAddr>(&mut self,
                                       hrtbt: SwarmAgent<Loc>,
                                       dest: A) -> SwarmResult<()> {
        let rpc = IronSwarmRPC::HRTBT(hrtbt);
        self.socket.send_packet(rpc, dest)
    }

    fn send_heartbeat_ack<A: ToSocketAddr>(&mut self,
                                           neighbors: Vec<SwarmAgent<Loc>>,
                                           dest: A) -> SwarmResult<()> {
        let rpc = IronSwarmRPC::HRTBTACK(neighbors);
        self.socket.send_packet(rpc, dest)
    }

    fn send_info<A: ToSocketAddr>(&mut self,
                                  loc: Loc,
                                  msg: SwarmMsg<Loc>,
                                  dest: A) -> SwarmResult<()> {
        let rpc = IronSwarmRPC::INFO(loc, msg);
        self.socket.send_packet(rpc, dest)
    }

    fn send_broadcast<A: ToSocketAddr>(&mut self,
                                       msg: SwarmMsg<Loc>,
                                       dest: A) -> SwarmResult<()> {
        let rpc = IronSwarmRPC::BROADCAST(msg);
        self.socket.send_packet(rpc, dest)
    }

    fn send_join<A: ToSocketAddr>(&mut self,
                                  agn: SwarmAgent<Loc>,
                                  dest: A) -> SwarmResult<()> {
        let rpc = IronSwarmRPC::JOIN(agn);
        self.socket.send_packet(rpc, dest)
    }

    fn join<A: ToSocketAddr>(&mut self, address: A) {
        let agn = self.local_agent.clone();
        self.send_join(agn, address);
    }

    fn heartbeat(&mut self) -> SwarmResult<()> {
        let addresses: Vec<SwarmAddr> = self.neighbors.iter().
            map(|n| n.address().clone()).collect();

        let agn = self.local_agent.clone();
        for dest in addresses.iter() {
            try!(self.send_heartbeat(agn.clone(), dest));
        }
        Ok(())
    }
}


// React to received RPC's.
impl<Loc: Show + Location + Encodable + Decodable + PartialEq + Clone> SwarmNetwork<Loc> {
    fn next_msg(&mut self) -> SwarmResult<IronSwarmRPC<Loc>> {
        self.socket.recv_msg()
    }

    fn dispatch_rpc(&mut self) {
        let res = self.next_msg();
        let rpc = match res {
            Ok(r) => r,
            Err(e) => panic!("Could not get next_msg: {:?}", e)
        };

        match rpc {
            IronSwarmRPC::HRTBT(agn) => {
                self.respond_to_heartbeat(agn);
            }
            IronSwarmRPC::HRTBTACK(ack_vec) => {
                let new_neighbors: Vec<SwarmAgent<Loc>> = {
                    let niter = self.neighbors.iter();
                    ack_vec.
                        into_iter().
                        filter(|n| {
                            *n != self.local_agent || niter.all(|old| *old != *n)
                        }).collect()
                };
                self.neighbors.push_all(new_neighbors.as_slice());
            }
            IronSwarmRPC::JOIN(join_agn) => {
                self.route_join_request(join_agn)
            }
            IronSwarmRPC::INFO(loc, msg) => {}
            IronSwarmRPC::BROADCAST(msg) => {}
        }
    }

    fn find_closer_agent(&self, loc: &Loc) -> Option<SwarmAgent<Loc>> {
        self.neighbors.
            iter().
            filter(|&a| a.location().distance(loc) <
                   self.local_agent.location().distance(loc)).
            cloned().
            min_by(|a| a.location().distance(loc))
    }

    fn respond_to_heartbeat(&mut self, agn: SwarmAgent<Loc>) {
        {
            let dest = agn.address();
            let neighbors = self.neighbors.clone();

            self.send_heartbeat_ack(neighbors, dest);
        }

        if self.neighbors.iter().all(|n| *n != agn) {
            self.neighbors.push(agn);
        }
    }

    fn route_join_request(&mut self, join_agn: SwarmAgent<Loc>) {
        let closest_agent = self.find_closer_agent(join_agn.location());

        match closest_agent {
            Some(send_agn) => {
                self.send_join(join_agn, send_agn.address());
            }
            // TODO: Already have too many neighbors
            None => {
                self.neighbors.push(join_agn);
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate bincode;
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::test::next_test_port;
    use agent::{SwarmAgent};
    use artifact::SwarmArtifact;
    use swarm::socket::SwarmResult;
    use swarm::SwarmMsg;
    use Location;
    use swarm::network::IronSwarmRPC;
    use super::SwarmNetwork;
    use bincode::{decode, encode};

    fn construct_artifact() -> SwarmArtifact<isize> {
        let loc = 9is;

        SwarmArtifact::new(loc)
    }

    fn construct_agent() -> SwarmAgent<isize> {
        SwarmAgent::new(9, local_socket())
    }

    fn construct_swarm_msg() -> SwarmMsg<isize> {
        SwarmMsg::new_artifact_msg(construct_agent(), construct_artifact())
    }

    fn local_socket() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: next_test_port() }
    }

    fn bincode_rpc_tester(rpc: IronSwarmRPC<isize>) {
        let orig_rpc = rpc.clone();
        let limit = bincode::SizeLimit::Infinite;
        let encoded = bincode::encode(&rpc, limit).ok().unwrap();
        let dec_rpc: IronSwarmRPC<isize> =
            bincode::decode(encoded.as_slice()).ok().unwrap();
        assert_eq!(orig_rpc, dec_rpc);
    }

    fn send_broadcast_tester(from_nework: &mut SwarmNetwork<isize>,
                           to_network: &mut SwarmNetwork<isize>) -> SwarmResult<()> {
        let msg = construct_swarm_msg();
        let exp_rpc = IronSwarmRPC::BROADCAST(msg.clone());

        try!(from_nework.send_broadcast(msg, to_network.address()));
        let recv_rpc = try!(to_network.next_msg());

        assert_eq!(exp_rpc, recv_rpc);
        Ok(())
    }

    fn next_msg_rpc_tester(from_network: &mut SwarmNetwork<isize>,
                           to_network: &mut SwarmNetwork<isize>,
                           rpc: IronSwarmRPC<isize>) -> SwarmResult<()> {
        let orig_rpc = rpc.clone();
        let net_sock = to_network.address();
        try!(from_network.socket.send_packet(rpc, net_sock));

        let dec_rpc = try!(to_network.next_msg());
        assert_eq!(orig_rpc, dec_rpc);
        Ok(())
    }

    #[test]
    fn next_msg_test() {
        let mut ack_vec = Vec::new();
        ack_vec.push(construct_agent());
        let mut from_network = SwarmNetwork::new(0is, local_socket());
        let mut to_network = SwarmNetwork::new(1is, local_socket());

        let res = next_msg_rpc_tester(&mut from_network, &mut to_network,
                                      IronSwarmRPC::HRTBT(construct_agent()));
        assert!(res.is_ok());
        let res = next_msg_rpc_tester(&mut from_network, &mut to_network,
                                      IronSwarmRPC::HRTBTACK(ack_vec));
        assert!(res.is_ok());
        let res = next_msg_rpc_tester(&mut from_network, &mut to_network,
                                      IronSwarmRPC::JOIN(construct_agent()));
        assert!(res.is_ok());
        let res = next_msg_rpc_tester(&mut from_network, &mut to_network,
                                      IronSwarmRPC::INFO(10, construct_swarm_msg()));
        assert!(res.is_ok());
        let res = next_msg_rpc_tester(&mut from_network, &mut to_network,
                                      IronSwarmRPC::BROADCAST(construct_swarm_msg()));
        assert!(res.is_ok());
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
    fn send_rpc_test() {
        let mut network_to = SwarmNetwork::new(0is, local_socket());
        let mut network_from = SwarmNetwork::new(10is, local_socket());

        let res = send_broadcast_tester(&mut network_from, &mut network_to);
        assert!(res.is_ok());
    }

    #[test]
    fn join_self_closest_test() {
        let mut network1 = SwarmNetwork::new(0is, local_socket());
        let mut network2 = SwarmNetwork::new(10is, local_socket());
        let mut joining = SwarmNetwork::new(2is, local_socket());
        network1.neighbors.push(network2.local_agent.clone());

        joining.join(network1.address());
        network1.dispatch_rpc();

        assert_eq!(network1.neighbors.len(), 2);
    }

    #[test]
    fn join_other_closest_test() {
        let mut network1 = SwarmNetwork::new(0is, local_socket());
        let mut network2 = SwarmNetwork::new(10is, local_socket());
        let mut joining = SwarmNetwork::new(9is, local_socket());
        network1.neighbors.push(network2.local_agent.clone());

        joining.join(network1.address());
        network1.dispatch_rpc();
        network2.dispatch_rpc();

        assert_eq!(network1.neighbors.len(), 1);
        assert_eq!(network2.neighbors.len(), 1);
    }

    #[test]
    fn htbt_and_ack_add_neighbor_test() {
        let mut network1 = SwarmNetwork::new(0is, local_socket());
        let mut network2 = SwarmNetwork::new(10is, local_socket());
        network1.neighbors.push(network2.local_agent.clone());

        // Neighbor of ACK'd swarm agent
        let mut network3 = SwarmNetwork::new(9is, local_socket());
        network2.neighbors.push(network3.local_agent.clone());

        // New agent that is not in neighbor list.
        let mut network4 = SwarmNetwork::new(11is, local_socket());
        network4.neighbors.push(network1.local_agent.clone());

        network1.heartbeat();
        network2.dispatch_rpc();
        network1.dispatch_rpc();

        network4.heartbeat();
        network1.dispatch_rpc();

        assert_eq!(network1.neighbors.len(), 3);
    }
}
