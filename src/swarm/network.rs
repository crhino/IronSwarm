extern crate serialize;

use serialize::{Decodable, Encodable};
use std::vec::Vec;
use swarm::socket::SwarmSocket;
use swarm::SwarmMsg;
use agent::SwarmAgent;
use std::io::{IoResult};
use std::io::net::ip::{SocketAddr, ToSocketAddr};

#[deriving(Clone, Eq, PartialEq, Show, Decodable, Encodable)]
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

