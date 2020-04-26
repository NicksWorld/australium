use std::convert::TryInto;

pub struct SourceMessage {
	raw: Vec<u8>
}

impl SourceMessage {
	pub fn as_slice(&self) -> &[u8] {
		self.raw.as_slice()
	}

	pub fn size(&self) -> usize {
		self.raw.len()
	}
}

impl From<&[u8]> for SourceMessage {
	fn from(vec: &[u8]) -> Self {
		SourceMessage {
			raw: vec.to_vec()
		}
	}
}

pub struct SourceMessageBuilder {
	raw: Vec<u8>
}

impl SourceMessageBuilder {
	pub fn build(self) -> SourceMessage {
		SourceMessage {
			raw: self.raw
		}
	}

	pub fn byte(mut self, val: u8) -> Self {
		self.raw.push(val);
		self
	}

	pub fn float(mut self, val: f32) -> Self {
		for b in &val.to_le_bytes() {
			self.raw.push(*b);
		}
		self
	}

	pub fn long(mut self, val: i32) -> Self {
		for b in &val.to_le_bytes() {
			self.raw.push(*b);
		}
		self
	}

	pub fn long_long(mut self, val: u64) -> Self {
		for b in &val.to_le_bytes() {
			self.raw.push(*b);
		}
		self
	}

	pub fn new() -> Self {
		Self {
			raw: Vec::new()
		}
	}

	pub fn short(mut self, val: i16) -> Self {
		for b in &val.to_le_bytes() {
			self.raw.push(*b);
		}
		self
	}

	pub fn string(mut self, val: &str) -> Self {
		for b in val.as_bytes() {
			self.raw.push(*b);
		}
		self.raw.push(0);
		self
	}
}

pub struct SourceMessageReader {
	message: SourceMessage,
	position: usize
}

impl SourceMessageReader {
	pub fn byte(&mut self) -> u8 {
		self.position += 1;
		self.message.raw[self.position - 1]
	}

	pub fn float(&mut self) -> f32 {
		self.position += 4;
		f32::from_le_bytes(self.message.raw[self.position - 4..self.position].try_into().unwrap())
	}

	pub fn long(&mut self) -> i32 {
		self.position += 4;
		i32::from_le_bytes(self.message.raw[self.position - 4..self.position].try_into().unwrap())
	}

	pub fn long_long(&mut self) -> u64 {
		self.position += 8;
		u64::from_le_bytes(self.message.raw[self.position - 8..self.position].try_into().unwrap())
	}

	pub fn new(msg: SourceMessage) -> Self {
		Self {
			message: msg,
			position: 0
		}
	}

	pub fn position(&self) -> usize {
		self.position
	}

	pub fn seek(&mut self, pos: usize) {
		self.position = pos;
	}

	pub fn short(&mut self) -> i16 {
		self.position += 2;
		i16::from_le_bytes(self.message.raw[self.position - 2..self.position].try_into().unwrap())
	}

	pub fn size(&self) -> usize {
		self.message.size()
	}

	pub fn string(&mut self) -> String {
		let mut data = Vec::new();
		let mut lcv = true;
		while lcv {
			let b = self.byte();
			if b == 0 {
				lcv = false;
			} else {
				data.push(b);
			}
		}
		String::from_utf8_lossy(&data).to_string()
	}
}
