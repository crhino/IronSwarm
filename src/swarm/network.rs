use std::vec::Vec;
use swarm::SwarmMsg;

enum IronSwarmMsg<Loc, Agn, Art> {
    HRTBT(Agn),
    HRTBTACK(Vec<Agn>),
    JOIN(Agn),
    INFO(Loc, SwarmMsg<Loc, Agn, Art>),
    BROADCAST(SwarmMsg<Loc, Agn, Art>)
}

pub struct SwarmNetwork<Loc, Agn, Art> {
    local_agent: Agn,
    neighbors: Vec<Agn>
}
