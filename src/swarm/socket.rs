use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::io::net::ip::{SocketAddr, ToSocketAddr};
use std::io::net::udp::UdpSocket;
use std::io::IoErrorKind;
use std::io::MemWriter;
use std::io::{IoResult, IoError, BufReader};
use bincode::{decode_from, encode};
use bincode::DecoderReader;
use bincode::EncoderWriter;
use std::mem;

const MAX_PACKET_SIZE: uint = 1024;

#[deriving(RustcDecodable, RustcEncodable, Show)]
struct Packet<B> {
    body: B
}

impl<B> Packet<B> {
    pub fn inner(self) -> B {
        self.body
    }
}

pub struct SwarmSocket {
    recv_buf: [u8, ..MAX_PACKET_SIZE],
    socket: UdpSocket
}

impl SwarmSocket {
    pub fn new<A: ToSocketAddr>(address: A) -> SwarmSocket {
        let mut socket = match UdpSocket::bind(address) {
            Ok(s) => s,
            Err(e) => panic!("Could not bind socket: {}", e)
        };
        SwarmSocket {
            recv_buf: [0u8, ..MAX_PACKET_SIZE],
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
    pub fn recv_msg<'a, B>(&mut self) -> IoResult<B>
    where B: Decodable<DecoderReader<'a,BufReader<'a>>, IoError> {
        match self.socket.recv_from(&mut self.recv_buf) {
            Ok((amt, _)) => {
                //XXX: transmuting the buffer in order to get appropriate lifetime.
                let transmuted_buf = unsafe {
                    mem::transmute(self.recv_buf.slice_to(amt))
                };
                let pkt: Packet<B> = try!(decode_from(&mut BufReader::new(transmuted_buf)));
                Ok(pkt.inner())
            }
            Err(e) => Err(e)
        }
    }
}

// Implement sending of IronSwarmRPC through the UDP socket.
impl<'a, B:Encodable<EncoderWriter<'a,MemWriter>, IoError>, A: ToSocketAddr> SwarmSocket {
    pub fn send_packet(&mut self, body: B, dest: A) -> IoResult<()> {
        let packet = Packet { body: body };
        let encoded = try!(encode(&packet));
        if encoded.len() > MAX_PACKET_SIZE {
            Err(IoError {
                kind: IoErrorKind::OtherIoError,
                desc: "Packet exceed maximum size",
                detail: Some(format!("Maximum size is {} bytes", MAX_PACKET_SIZE)),
            })
        } else {
            self.socket.send_to(encoded.as_slice(), dest)
        }
    }

}

#[cfg(test)]
mod test {
    use std::io::net::ip::{SocketAddr, Ipv4Addr};
    use std::io::net::udp::UdpSocket;
    use std::io::test::next_test_port;
    use std::io::IoResult;
    use std::vec::Vec;
    use super::SwarmSocket;

    fn local_socket() -> SocketAddr {
        SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: next_test_port() }
    }

    fn construct_swarm_socket_with_local_socket() -> SwarmSocket {
        SwarmSocket::new(local_socket())
    }

    #[deriving(Show, RustcDecodable, RustcEncodable)]
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
        let res: IoResult<Test<u8>> = to_socket.recv_msg();
        assert!(res.is_ok());
    }
}
