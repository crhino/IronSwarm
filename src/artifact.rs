// Artifact in the Swarm Ecosystem
//
// An Artifact is a resource that Swarm Agents can interact with. Artifacts are
// discovered by Agents locally and knowledge of Artifacts is dispersed throughout
// the Swarm using either the INFO or BROADCAST RPC, depending on the associated
// importance of a particular Artifact.
use Location;
use byteid::ByteId;

pub trait SwarmArtifact<L: Location> {
    fn location(&self) -> &L;
    fn update_location(&self, location: L);
}

struct IronSwarmArtifact<L> {
    id: ByteId,
    location: L
}
