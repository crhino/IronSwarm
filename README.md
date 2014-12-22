### Iron Swarm

A peer-to-peer, agent-based complex system framework.

This framework takes in an actor that implements `ReactToSwarm` in order to react to `SwarmMsg`s. The user will also implement a type that implements `Location`, which corresponds with the agent's location. The location is updated by the actor over time. The `swarm` module contains the Swarm overlay network and is how the Swarm passes messages around.

## RPC Mechanism

Currently, Iron Swarm uses an enum consisting of the available RPC types:

```rust
pub enum IronSwarmRPC<Loc> {
    HRTBT(SwarmAgent<Loc>),
    HRTBTACK(Vec<SwarmAgent<Loc>>),
    JOIN(SwarmAgent<Loc>),
    INFO(Loc, SwarmMsg<Loc>),
    BROADCAST(SwarmMsg<Loc>),
}
```

`HRTBT`, as the name suggests, is a periodic heartbeat of the Swarm agent to it's neighbors. This is used to keep neighbor lists up-to-date.

`HRTBTACK` is the acknowledgement of a HRTBT. The purpose of this is two-fold. First, a Swarm agent will send the HRTBTACK along with a list of it's neighbors, allowing the receiving agent to update it's own neighbor list as agents move. This should prove to be a fairly good way of keeping up-to-date neighbor lists, as agents are most likely going to move in small increments between each HRTBT. Second, the HRTBTACK is used in order to ensure that an agent has an upper bound on the number of incoming/outgoing Swarm connections at any one time. If an agent has already hit the threshold and receives another HRTBT, it can choose to not respond, invalidating it's place in the sending agent's list.

`JOIN` is the RPC used to join the Swarm network, and will route the agent into it's correct place in the overlay network.

`INFO` is the first of the RPCs that a user has involvement with. The `INFO` RPC is used to send a `SwarmMsg` to a specific location.

`BROADCAST` is similar to `INFO` except, as the name suggests, every agent in the network will receive and  react to the message.

## Uses

Potential uses include:
- Modeling animal/human interactions.
- A group of simple robotic agents performing a given task.
- A cluster of servers/VMs cooperatively performing a task.
