// Artifact in the Swarm Ecosystem
//
// An Artifact is a resource that Swarm Agents can interact with. Artifacts are
// discovered by Agents locally and knowledge of Artifacts is dispersed throughout
// the Swarm using either the INFO or BROADCAST RPC, depending on the associated
// importance of a particular Artifact.
use Location;
use byteid::ByteId;

pub trait SwarmArtifact<L: Location> {
    fn new(loc: L) -> Self;
    fn location(&self) -> &L;
    fn update_location(&mut self, location: L);
}

pub struct IronSwarmArtifact<L> {
    id: ByteId,
    location: L
}

impl<L: Location> SwarmArtifact<L> for IronSwarmArtifact<L> {
    fn new(loc: L) -> IronSwarmArtifact<L> {
        IronSwarmArtifact {
            id: ByteId::random_id(), location: loc
        }
    }

    fn location(&self) -> &L {
        &self.location
    }

    fn update_location(&mut self, location: L) {
        self.location = location;
    }
}
