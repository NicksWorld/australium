use crate::message::SourceMessage;
use std::io::Result;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

pub struct SourceSocket {
	socket: UdpSocket
}

impl SourceSocket {
	pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<SourceSocket> {
		Ok(Self {
			socket: UdpSocket::bind(addr)?
		})
	}

	pub fn recv_from(&self) -> Result<(SourceMessage, SocketAddr)> {
		let mut buffer = vec![0; 1400];
		let (size, addr) = self.socket.recv_from(&mut buffer)?;
		Ok((SourceMessage::from(&buffer[..size]), addr))
	}

	pub fn send_to<A: ToSocketAddrs>(&self, msg: &SourceMessage, addr: A) -> Result<usize> {
		self.socket.send_to(msg.as_slice(), addr)
	}
}
