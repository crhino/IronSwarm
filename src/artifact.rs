// Artifact in the Swarm Ecosystem
//
// An Artifact is a resource that Swarm Agents can interact with. Artifacts are
// discovered by Agents locally and knowledge of Artifacts is dispersed throughout
// the Swarm using either the INFO or BROADCAST RPC, depending on the associated
// importance of a particular Artifact.
extern crate serialize;
use Location;
use byteid::ByteId;

#[deriving(Clone, Eq, PartialEq, Show, Decodable, Encodable)]
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

    pub fn id(&self) -> &ByteId {
        &self.id
    }

    pub fn update_location(&mut self, location: L) {
        self.location = location;
    }
}

#[cfg(test)]
mod test {
    extern crate bincode;
    use artifact::SwarmArtifact;
    use std::vec::Vec;
    use std::path::BytesContainer;

    #[test]
    fn bincode_test() {
        let loc = 9i;

        let art = SwarmArtifact::new(loc);
        let encoded = bincode::encode(&art).ok().unwrap();
        let dec_art: SwarmArtifact<int> =
            bincode::decode(encoded).ok().unwrap();

        assert_eq!(art.location(), dec_art.location());
        assert_eq!(art.id(), dec_art.id());
    }
}
