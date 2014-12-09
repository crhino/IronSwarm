### Iron Swarm

A peer-to-peer, agent-based complex system framework.

This framework takes in an actor that implements `ReactToSwarm` in order to react to `SwarmMsg`s. The user will also implement a type that implements `Location`, which corresponds with the agent's location. The location is updated by the actor over time. The `swarm` module contains the Swarm overlay network and is how the Swarm passes messages around.

## Uses

Potential uses include:
- Modeling animal/human interactions.
- A group of simple robotic agents performing a given task.
- A cluster of servers/VMs cooperatively performing a task.
