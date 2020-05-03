use std::convert::TryInto;
use std::io::{Cursor, Read, Result};

pub struct ByteReader {
	cursor: Cursor<Vec<u8>>
}

impl ByteReader {
	pub fn byte(&mut self) -> Result<u8> {
		let mut buffer = [0; 1];
		self.cursor.read_exact(&mut buffer)?;
		Ok(buffer[0])
	}

	pub fn float(&mut self) -> Result<f32> {
		let mut buffer = [0; 4];
		self.cursor.read_exact(&mut buffer)?;
		Ok(f32::from_le_bytes(buffer[..].try_into().unwrap()))
	}

	pub fn long(&mut self) -> Result<i32> {
		let mut buffer = [0; 4];
		self.cursor.read_exact(&mut buffer)?;
		Ok(i32::from_le_bytes(buffer[..].try_into().unwrap()))
	}

	pub fn long_long(&mut self) -> Result<u64> {
		let mut buffer = [0; 8];
		self.cursor.read_exact(&mut buffer)?;
		Ok(u64::from_le_bytes(buffer[..].try_into().unwrap()))
	}

	pub fn new(data: Vec<u8>) -> Self {
		Self {
			cursor: Cursor::new(data)
		}
	}

	pub fn short(&mut self) -> Result<i16> {
		let mut buffer = [0; 2];
		self.cursor.read_exact(&mut buffer)?;
		Ok(i16::from_le_bytes(buffer[..].try_into().unwrap()))
	}

	pub fn string(&mut self) -> Result<String> {
		let mut string = Vec::new();
		let mut temp = self.byte()?;
		while temp != 0 {
			string.push(temp);
			temp = self.byte()?;
		}
		Ok(String::from_utf8(string).unwrap())
	}
}
