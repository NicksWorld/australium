use std::convert::TryInto;
use std::net::{IpAddr, SocketAddr};

mod message;
mod socket;

use message::{SourceMessage, SourceMessageBuilder, SourceMessageReader};
use socket::SourceSocket;

#[cfg(target_os = "macos")]
static CURRENT_OS: u8 = 'm' as u8;

#[cfg(target_os = "linux")]
static CURRENT_OS: u8 = 'l' as u8;

#[cfg(target_os = "windows")]
static CURRENT_OS: u8 = 'w' as u8;

static CHALLENGE_MAGIC: i32 = 0x3111D7E9;

fn generate_challenge(addr: SocketAddr) -> i32 {
	CHALLENGE_MAGIC ^ match addr.ip() {
		IpAddr::V4(a) => i32::from_le_bytes(a.octets()),
		IpAddr::V6(a) => i32::from_le_bytes(a.octets()[12..16].try_into().unwrap())
	}
}

fn handle_connection(socket: &SourceSocket, mut reader: SourceMessageReader, addr: SocketAddr) {
	let _protocol = reader.long();
	let _authentication = reader.long();
	let _challenge = reader.long();
	let retry = reader.long();
	let message = SourceMessageBuilder::new()
		.long(-1) // Single Packet Response
		.byte('9' as u8) // S2C_CONNREJECT
		.long(retry) // The client's challenge
		.string("Inbound player connections are not supported by Australium at this time. ~Bread\n")
		.build();
	socket.send_to(&message, addr).unwrap();
}

fn handle_getchallenge(socket: &SourceSocket, mut reader: SourceMessageReader, addr: SocketAddr) {
	let message = SourceMessageBuilder::new()
		.long(-1) // Single Packet Resposne
		.byte('A' as u8) // S2C_CHALLENGE
		.long(0x5A4F4933) // Magic Version
		.long(generate_challenge(addr)) // Challenge
		.long(reader.long()) // Client Challenge
		.long(0x03) // Authentication Protocol
		.short(0) // Steam2 Encryption Key(Deprecated)
		.byte(0) // VAC Secure?
		.string("000000") // Padding
		.build();
	socket.send_to(&message, addr).unwrap();
}

fn handle_info(socket: &SourceSocket, addr: SocketAddr) {
	let message = SourceMessageBuilder::new()
		.long(-1) // Single Packet Response
		.byte('I' as u8) // S2A_INFO_SRC
		.byte(0x11) // Protocol Version
		.string("Australium Project") // Server Name
		.string("testing") // Map Name
		.string("tf") // Game Folder
		.string("Australium") // Game Name
		.short(440) // Steam AppID
		.byte(0) // Number of Players
		.byte(24) // Maximum Players
		.byte(0) // Number of Bots
		.byte('d' as u8) // Server Type
		.byte(CURRENT_OS) // Operating System
		.byte(0) // Password-Protected
		.byte(0) // VAC Secured
		.byte(0xA1) // Extra Data Flags
		.short(27015) // Server Port
		.string("australium,testing") // Server Tags
		.long_long(440) // 64-bit Steam AppID
		.build();
	socket.send_to(&message, addr).unwrap();
}

fn handle_player(socket: &SourceSocket, mut reader: SourceMessageReader, addr: SocketAddr) {
	let challenge = reader.long();
	if challenge == -1 {
		let message = SourceMessageBuilder::new()
			.long(-1) // Single Packet Response
			.byte('A' as u8) // S2C_CHALLENGE
			.long(generate_challenge(addr)) // Challenge
			.build();
		socket.send_to(&message, addr).unwrap();
	} else if generate_challenge(addr) == challenge {
		let message = SourceMessageBuilder::new()
			.long(-1) // Single Packet Response
			.byte('D' as u8) // S2C_PLAYER
			.byte(0) // Number of Players
			.build();
		socket.send_to(&message, addr).unwrap();
	}
}

fn handle_rules(socket: &SourceSocket, mut reader: SourceMessageReader, addr: SocketAddr) {
	let challenge = reader.long();
	if challenge == -1 {
		let message = SourceMessageBuilder::new()
			.long(-1) // Single Packet Response
			.byte('A' as u8) // S2C_CHALLENGE
			.long(generate_challenge(addr)) // Challenge
			.build();
		socket.send_to(&message, addr).unwrap();
	} else if generate_challenge(addr) == challenge {
		let message = SourceMessageBuilder::new()
			.long(-1) // Single Packet Response
			.byte('E' as u8) // S2C_RULES
			.short(0) // Number of Players
			.build();
		socket.send_to(&message, addr).unwrap();
	}
}

fn handle_connectionless(socket: &SourceSocket, mut reader: SourceMessageReader, addr: SocketAddr) {
	let t = reader.byte();
	match t {
		0x54 => handle_info(socket, addr),
		0x55 => handle_player(socket, reader, addr),
		0x56 => handle_rules(socket, reader, addr),
		0x6B => handle_connection(socket, reader, addr),
		0x71 => handle_getchallenge(socket, reader, addr),
		_ => println!("handle_connectionless: 0x{:0x}", t)
	}
}

fn main() {
	let mut socket = SourceSocket::bind("0.0.0.0:27015").unwrap();
	loop {
		if let Ok((msg, addr)) = socket.recv_from() {
			let mut reader = SourceMessageReader::new(msg);
			let signature = reader.long();
			match signature {
				-1 => handle_connectionless(&mut socket, reader, addr),
				_ => println!("main: 0x{:0x}", signature)
			}
		}
	}
}
