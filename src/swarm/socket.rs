use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::io::net::ip::{SocketAddr, ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::IoErrorKind;
use std::io::MemWriter;
use std::io::{IoResult, IoError, BufReader};
use bincode::{self, decode_from, encode, DecodingError, EncodingError};
use bincode::DecoderReader;
use bincode::EncoderWriter;
use std::mem;
use std::error::FromError;

pub const MAX_PACKET_SIZE: usize = 1024;

#[derive(RustcDecodable, RustcEncodable, Show)]
struct Packet<B> {
    body: B
}

impl<B> Packet<B> {
    pub fn inner(self) -> B {
        self.body
    }
}

pub struct SwarmSocket {
    recv_buf: [u8; MAX_PACKET_SIZE],
    socket: UdpSocket
}

pub type SwarmResult<R> = Result<R, SwarmError>;

fn io_to_swarm_result<R>(res: IoResult<R>) -> SwarmResult<R> {
    match res {
        Ok(r) => Ok(r),
        Err(err) => Err(SwarmError::IoError(err))
    }
}

#[derive(Show)]
pub enum SwarmError {
    IoError(IoError),
}

impl FromError<DecodingError> for SwarmError {
    fn from_error(err: DecodingError) -> SwarmError {
        match err {
            DecodingError::IoError(err) => SwarmError::IoError(err),
            _ => panic!("{:?}", err)
        }
    }
}

impl FromError<EncodingError> for SwarmError {
    fn from_error(err: EncodingError) -> SwarmError {
        match err {
            EncodingError::IoError(err) => SwarmError::IoError(err),
            _ => panic!("{:?}", err)
        }
    }
}

impl FromError<IoError> for SwarmError {
    fn from_error(err: IoError) -> SwarmError {
        SwarmError::IoError(err)
    }
}

impl SwarmSocket {
    pub fn new<A: ToSocketAddr>(address: A) -> SwarmSocket {
        let mut socket = match UdpSocket::bind(address) {
            Ok(s) => s,
            Err(e) => panic!("Could not bind socket: {}", e)
        };
        SwarmSocket {
            recv_buf: [0u8; MAX_PACKET_SIZE],
            socket: socket,
        }
    }

    pub fn socket_name(&mut self) -> SocketAddr {
        match self.socket.socket_name() {
            Ok(a) => a,
            Err(e) => panic!("Could not get socket name: {}", e)
        }
    }
}

// Implement receiving of packets through the UDP socket.
impl SwarmSocket {
    pub fn recv_msg<'a, B>(&mut self) -> SwarmResult<B>
    where B: Decodable {
        match self.socket.recv_from(&mut self.recv_buf) {
            Ok((amt, _)) => {
                //XXX: transmuting the buffer in order to get appropriate lifetime.
                let transmuted_buf = unsafe {
                    mem::transmute(self.recv_buf.slice_to(amt))
                };
                let limit = bincode::SizeLimit::UpperBound(MAX_PACKET_SIZE as u64);
                let pkt: Packet<B> = try!(decode_from(
                        &mut BufReader::new(transmuted_buf),
                        limit));
                Ok(pkt.inner())
            }
            Err(e) => Err(SwarmError::IoError(e))
        }
    }
}

// Implement sending of IronSwarmRPC through the UDP socket.
impl SwarmSocket {
    pub fn send_packet<B:Encodable, A: ToSocketAddr>(&mut self, body: B, dest: A) -> SwarmResult<()> {
        let packet = Packet { body: body };
        let limit = bincode::SizeLimit::UpperBound(MAX_PACKET_SIZE as u64);
        let encoded = try!(encode(&packet, limit));
        if encoded.len() > MAX_PACKET_SIZE {
            Err(SwarmError::IoError(IoError {
                kind: IoErrorKind::OtherIoError,
                desc: "Packet exceed maximum size",
                detail: Some(format!("Maximum size is {} bytes", MAX_PACKET_SIZE)),
            }))
        } else {
            io_to_swarm_result(self.socket.send_to(encoded.as_slice(), dest))
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::net::udp::UdpSocket;
    use std::io::test::next_test_port;
    use std::vec::Vec;
    use super::{SwarmSocket, SwarmResult};

    fn local_socket() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: next_test_port() }
    }

    fn construct_swarm_socket_with_local_socket() -> SwarmSocket {
        SwarmSocket::new(local_socket())
    }

    #[derive(Show, RustcDecodable, RustcEncodable)]
    struct Test<T> {
        body: T
    }

    #[test]
    fn socket_test() {
        let mut from_socket = construct_swarm_socket_with_local_socket();
        let mut to_socket = construct_swarm_socket_with_local_socket();
        let socket_addr = to_socket.socket_name();
        let body = Test { body: 27u8 };

        let res = from_socket.send_packet(body, socket_addr);
        assert!(res.is_ok());
        let res: SwarmResult<Test<u8>> = to_socket.recv_msg();
        assert!(res.is_ok());
    }
}
