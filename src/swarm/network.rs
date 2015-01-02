use rustc_serialize::{Decodable, Encodable};
use std::vec::Vec;
use swarm::socket::SwarmSocket;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use Location;
use std::io::net::ip::{SocketAddr, ToSocketAddr};
use bincode::DecoderReader;
use bincode::EncoderWriter;
use std::io::{IoResult, IoError, BufReader};
use std::io::MemWriter;

#[deriving(Clone, Eq, PartialEq, Show, RustcDecodable, RustcEncodable)]
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
impl<Loc: Clone> SwarmNetwork<Loc> {
    fn send_join<'a, A: ToSocketAddr>(&mut self, address: A, agn: SwarmAgent<Loc>)
        where Loc: Encodable<EncoderWriter<'a,MemWriter>, IoError> {
            match self.socket.send_join(address, agn) {
                Ok(_) => {},
                Err(e) => panic!("Could not send join RPC: {}", e)
            }
        }

    fn join<'a, A: ToSocketAddr>(&mut self, address: A)
        where Loc: Encodable<EncoderWriter<'a,MemWriter>, IoError> {
            let agn = self.local_agent.clone();
            self.send_join(address, agn);
        }
}


// React to received RPC's.
impl<'e,'d,Loc:Location +
Encodable<EncoderWriter<'e,MemWriter>, IoError> +
Decodable<DecoderReader<'d,BufReader<'d>>,IoError> +
Clone> SwarmNetwork<Loc> {
    fn next_msg(&mut self) -> IronSwarmRPC<Loc> {
        match self.socket.recv_msg() {
            Ok(r) => r,
            Err(e) => panic!("Could not recv RPC: {}", e)
        }
    }

    fn dispatch_rpc(&mut self) {
        let rpc = self.next_msg();
        match rpc {
            IronSwarmRPC::HRTBT(agn) => {}
            IronSwarmRPC::HRTBTACK(ack_vec) => {}
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

    fn route_join_request(&mut self, join_agn: SwarmAgent<Loc>) {
        let closest_agent = self.find_closer_agent(join_agn.location());

        match closest_agent {
            // Send JOIN request to closest known agent.
            Some(send_agn) => {
                self.send_join(send_agn.address(), join_agn);
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
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::test::next_test_port;
    use agent::{SwarmAgent};
    use artifact::SwarmArtifact;
    use swarm::SwarmMsg;
    use Location;
    use super::SwarmNetwork;

    fn local_socket() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: next_test_port() }
    }

    #[test]
    fn join_self_closest_test() {
        let mut network1 = SwarmNetwork::new(0i, local_socket());
        let mut network2 = SwarmNetwork::new(10i, local_socket());
        let mut joining = SwarmNetwork::new(2i, local_socket());
        network1.neighbors.push(network2.local_agent.clone());

        joining.join(network1.address());
        network1.dispatch_rpc();

        assert_eq!(network1.neighbors.len(), 2);
    }

    #[test]
    fn join_other_closest_test() {
        let mut network1 = SwarmNetwork::new(0i, local_socket());
        let mut network2 = SwarmNetwork::new(10i, local_socket());
        let mut joining = SwarmNetwork::new(9i, local_socket());
        network1.neighbors.push(network2.local_agent.clone());

        joining.join(network1.address());
        network1.dispatch_rpc();
        network2.dispatch_rpc();

        assert_eq!(network1.neighbors.len(), 1);
        assert_eq!(network2.neighbors.len(), 1);
    }
}
