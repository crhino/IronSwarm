use SwarmSend;
use std::fmt;
use std::rand::{task_rng, Rng};

const BYTE_ID_LEN: uint = 20;

#[deriving(Eq, PartialEq, Clone)]
pub struct ByteId([u8, ..BYTE_ID_LEN]);

impl ByteId {
    pub fn random_id() -> ByteId {
        let mut data = [0u8, ..BYTE_ID_LEN];
        task_rng().fill_bytes(&mut data);
        ByteId(data)
    }

    pub fn set_byte(&mut self, index: uint, val: u8) {
        let &ByteId(ref mut data) = self;
        data[index % BYTE_ID_LEN] = val;
    }

    pub fn byte(&self, index: uint) -> u8 {
        let &ByteId(ref data) = self;
        data[index % BYTE_ID_LEN]
    }
}

impl SwarmSend for ByteId {
    fn swarm_encode(id: ByteId, pkt: &mut Vec<u8>) {
        let ByteId(ref data) = id;
        pkt.push_all(data);
    }

    fn swarm_decode(pkt: &[u8]) -> (uint, ByteId) {
        let mut data = [0u8, ..BYTE_ID_LEN];
        let slice = pkt.slice_to(BYTE_ID_LEN);
        let mut i = 0;
        for &b in slice.iter() {
            println!("i: {}, b: {}", i, b);
            data[i] = b;
            i+=1;
        }

        let id = ByteId(data);
        (BYTE_ID_LEN, id)
    }
}

impl BitAnd<ByteId, ByteId> for ByteId {
    #[inline]
    fn bitand(&self, other: &ByteId) -> ByteId {
        let mut ret = [0u8, ..BYTE_ID_LEN];
        let &ByteId(ref me) = self;
        let &ByteId(ref you) = other;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = me[i] & you[i];
        }

        ByteId(ret)
    }
}

impl BitOr<ByteId, ByteId> for ByteId {
    #[inline]
    fn bitor(&self, other: &ByteId) -> ByteId {
        let mut ret = [0u8, ..BYTE_ID_LEN];
        let &ByteId(ref me) = self;
        let &ByteId(ref you) = other;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = me[i] | you[i];
        }

        ByteId(ret)
    }
}

impl BitXor<ByteId, ByteId> for ByteId {
    #[inline]
    fn bitxor(&self, other: &ByteId) -> ByteId {
        let mut ret = [0u8, ..BYTE_ID_LEN];
        let &ByteId(ref me) = self;
        let &ByteId(ref you) = other;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = me[i] ^ you[i];
        }

        ByteId(ret)
    }
}

impl Not<ByteId> for ByteId {
    #[inline]
    fn not(&self) -> ByteId {
        let mut ret = [0u8, ..BYTE_ID_LEN];
        let &ByteId(ref me) = self;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = !me[i];
        }

        ByteId(ret)
    }
}

impl fmt::Show for ByteId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let &ByteId(ref data) = self;
        write!(fmt, "{}", data)
    }
}


#[cfg(test)]
mod tests {
    use super::{ByteId, BYTE_ID_LEN};

    #[test]
    fn byte_test() {
        let a = [0u8, ..BYTE_ID_LEN];
        let mut b = [0u8, ..BYTE_ID_LEN];
        b[BYTE_ID_LEN/2] = 9;
        let id_a = ByteId(a);
        let id_b = ByteId(b);

        assert_eq!(id_a.byte(0), 0u8);
        assert_eq!(id_a.byte(BYTE_ID_LEN), 0u8);
        assert_eq!(id_b.byte(BYTE_ID_LEN/2), 9u8);
    }

    #[test]
    fn set_byte_test() {
        let a = [0u8, ..BYTE_ID_LEN];
        let mut id_a = ByteId(a);

        id_a.set_byte(0, 9);
        assert_eq!(id_a.byte(0), 9u8);

        id_a.set_byte(BYTE_ID_LEN, 9);
        assert_eq!(id_a.byte(BYTE_ID_LEN), 9u8);
    }

    #[test]
    fn bitand_test() {
        let data = [0u8, ..BYTE_ID_LEN];
        let mut id_a = ByteId(data.clone());
        let mut id_b = ByteId(data);

        id_a.set_byte(3, 0b1010_1010);
        id_b.set_byte(3, 0b0101_0111);
        let id_c = id_a & id_b;

        assert_eq!(id_c.byte(3), 0b0000_0010);
    }

    #[test]
    fn bitor_test() {
        let data = [0u8, ..BYTE_ID_LEN];
        let mut id_a = ByteId(data.clone());
        let mut id_b = ByteId(data);

        id_a.set_byte(3, 0b1010_1010);
        id_b.set_byte(3, 0b0101_0111);
        let id_c = id_a | id_b;

        assert_eq!(id_c.byte(3), 0b1111_1111);
    }

    #[test]
    fn bitxor_test() {
        let data = [0u8, ..BYTE_ID_LEN];
        let mut id_a = ByteId(data.clone());
        let mut id_b = ByteId(data);

        id_a.set_byte(3, 0b1010_1010);
        id_b.set_byte(3, 0b0101_0111);
        let id_c = id_a ^ id_b;

        assert_eq!(id_c.byte(3), 0b1111_1101);
    }

    #[test]
    fn not_test() {
        let data = [0u8, ..BYTE_ID_LEN];
        let mut id_b = ByteId(data);

        id_b.set_byte(3, 0b0000_1111);
        let id_c = !id_b;

        assert_eq!(id_c.byte(3), 0b1111_0000);
    }

    #[test]
    fn equal_test() {
        let id_a = ByteId::random_id();
        let id_b = ByteId::random_id();
        let id_c = id_a.clone();

        assert!(id_c == id_a);
        assert!(!(id_b == id_a));
    }

    #[test]
    fn show_test() {
        println!("{}", ByteId::random_id());
    }
}
