use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};
use std::fmt;
use std::rand::{thread_rng, Rng};
use std::ops::{BitAnd, BitOr, BitXor, Not};

const BYTE_ID_LEN: uint = 20;

#[derive(Clone, Eq, PartialEq, Show)]
pub struct ByteId([u8; BYTE_ID_LEN]);

impl ByteId {
    pub fn random_id() -> ByteId {
        let mut data = [0u8; BYTE_ID_LEN];
        thread_rng().fill_bytes(&mut data);
        ByteId(data)
    }

    pub fn set_byte(&mut self, index: uint, val: u8) {
        let &mut ByteId(ref mut data) = self;
        data[index % BYTE_ID_LEN] = val;
    }

    pub fn byte(&self, index: uint) -> u8 {
        let &ByteId(ref data) = self;
        data[index % BYTE_ID_LEN]
    }
}

impl Encodable for ByteId {
    fn encode<S:Encoder>(&self, s: &mut S) -> Result<(),S::Error> {
        let &ByteId(ref data) = self;
        for &n in data.iter() {
            try!(s.emit_u8(n))
        }
        Ok(())
    }
}

impl Decodable for ByteId {
    fn decode<D:Decoder>(d: &mut D) -> Result<ByteId,D::Error> {
        let mut data = [0u8; BYTE_ID_LEN];
        for i in range(0u, BYTE_ID_LEN) {
            data[i] = try!(d.read_u8());
        }

        Ok(ByteId(data))
    }
}

impl BitAnd<ByteId> for ByteId {
    type Output = ByteId;

    #[inline]
    fn bitand(self, other: ByteId) -> ByteId {
        let mut ret = [0u8; BYTE_ID_LEN];
        let ByteId(ref me) = self;
        let ByteId(ref you) = other;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = me[i] & you[i];
        }

        ByteId(ret)
    }
}

impl BitOr<ByteId> for ByteId {
    type Output = ByteId;

    #[inline]
    fn bitor(self, other: ByteId) -> ByteId {
        let mut ret = [0u8; BYTE_ID_LEN];
        let ByteId(ref me) = self;
        let ByteId(ref you) = other;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = me[i] | you[i];
        }

        ByteId(ret)
    }
}

impl BitXor<ByteId> for ByteId {
    type Output = ByteId;

    #[inline]
    fn bitxor(self, other: ByteId) -> ByteId {
        let mut ret = [0u8; BYTE_ID_LEN];
        let ByteId(ref me) = self;
        let ByteId(ref you) = other;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = me[i] ^ you[i];
        }

        ByteId(ret)
    }
}

impl Not for ByteId {
    type Output = ByteId;

    #[inline]
    fn not(self) -> ByteId {
        let mut ret = [0u8; BYTE_ID_LEN];
        let ByteId(ref me) = self;

        for i in range(0u, BYTE_ID_LEN) {
            ret[i] = !me[i];
        }

        ByteId(ret)
    }
}

#[cfg(test)]
mod test {
    use super::{ByteId, BYTE_ID_LEN};

    #[test]
    fn byte_test() {
        let a = [0u8; BYTE_ID_LEN];
        let mut b = [0u8; BYTE_ID_LEN];
        b[BYTE_ID_LEN/2] = 9;
        let id_a = ByteId(a);
        let id_b = ByteId(b);

        assert_eq!(id_a.byte(0), 0u8);
        assert_eq!(id_a.byte(BYTE_ID_LEN), 0u8);
        assert_eq!(id_b.byte(BYTE_ID_LEN/2), 9u8);
    }

    #[test]
    fn set_byte_test() {
        let a = [0u8; BYTE_ID_LEN];
        let mut id_a = ByteId(a);

        id_a.set_byte(0, 9);
        assert_eq!(id_a.byte(0), 9u8);

        id_a.set_byte(BYTE_ID_LEN, 9);
        assert_eq!(id_a.byte(BYTE_ID_LEN), 9u8);
    }

    #[test]
    fn bitand_test() {
        let data = [0u8; BYTE_ID_LEN];
        let mut id_a = ByteId(data.clone());
        let mut id_b = ByteId(data);

        id_a.set_byte(3, 0b1010_1010);
        id_b.set_byte(3, 0b0101_0111);
        let id_c = id_a & id_b;

        assert_eq!(id_c.byte(3), 0b0000_0010);
    }

    #[test]
    fn bitor_test() {
        let data = [0u8; BYTE_ID_LEN];
        let mut id_a = ByteId(data.clone());
        let mut id_b = ByteId(data);

        id_a.set_byte(3, 0b1010_1010);
        id_b.set_byte(3, 0b0101_0111);
        let id_c = id_a | id_b;

        assert_eq!(id_c.byte(3), 0b1111_1111);
    }

    #[test]
    fn bitxor_test() {
        let data = [0u8; BYTE_ID_LEN];
        let mut id_a = ByteId(data.clone());
        let mut id_b = ByteId(data);

        id_a.set_byte(3, 0b1010_1010);
        id_b.set_byte(3, 0b0101_0111);
        let id_c = id_a ^ id_b;

        assert_eq!(id_c.byte(3), 0b1111_1101);
    }

    #[test]
    fn not_test() {
        let data = [0u8; BYTE_ID_LEN];
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
        println!("{:?}", ByteId::random_id());
    }
}
