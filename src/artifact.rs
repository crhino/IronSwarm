// Artifact in the Swarm Ecosystem
//
// An Artifact is a resource that Swarm Agents can interact with. Artifacts are
// discovered by Agents locally and knowledge of Artifacts is dispersed throughout
// the Swarm using either the INFO or BROADCAST RPC, depending on the associated
// importance of a particular Artifact.
use Location;
use byteid::ByteId;

pub struct SwarmArtifact<L> {
    id: ByteId,
    location: L
}

impl<L> SwarmArtifact<L> {
    pub fn new(loc: L) -> SwarmArtifact<L> {
        SwarmArtifact {
            id: ByteId::random_id(), location: loc
        }
    }

    pub fn location(&self) -> &L {
        &self.location
    }

    pub fn update_location(&mut self, location: L) {
        self.location = location;
    }
}
