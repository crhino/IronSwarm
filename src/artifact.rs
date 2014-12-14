// Artifact in the Swarm Ecosystem
//
// An Artifact is a resource that Swarm Agents can interact with. Artifacts are
// discovered by Agents locally and knowledge of Artifacts is dispersed throughout
// the Swarm using either the INFO or BROADCAST RPC, depending on the associated
// importance of a particular Artifact.
use Location;
use byteid::ByteId;
use SwarmSend;

#[deriving(Clone, Eq, PartialEq, Show)]
pub struct SwarmArtifact<L> {
    id: ByteId,
    location: L
}

impl<L: SwarmSend + Clone> SwarmSend for SwarmArtifact<L> {
    fn swarm_encode(art: SwarmArtifact<L>, pkt: &mut Vec<u8>) {
        SwarmSend::swarm_encode(art.id().clone(), pkt);
        SwarmSend::swarm_encode(art.location().clone(), pkt);
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, SwarmArtifact<L>) {
        let mut pkt_ptr = 0;
        let (loc_idx, id) = SwarmSend::swarm_decode(pkt);
        pkt_ptr += loc_idx;
        let (end, loc) = SwarmSend::swarm_decode(pkt.slice_from(loc_idx));
        pkt_ptr += end;
        (pkt_ptr, SwarmArtifact { id: id, location: loc })
    }
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
    use SwarmSend;
    use artifact::SwarmArtifact;
    use std::vec::Vec;
    use std::path::BytesContainer;

    #[test]
    fn swarm_send_test() {
        let loc = 9i;

        let art = SwarmArtifact::new(loc);
        let mut vec = Vec::new();
        let encoded = SwarmSend::swarm_encode(art, &mut vec);
        let (_, dec_art): (uint, SwarmArtifact<int>) =
                            SwarmSend::swarm_decode(vec.container_as_bytes());

        assert_eq!(art.location(), dec_art.location());
        assert_eq!(art.id(), dec_art.id());
    }
}
