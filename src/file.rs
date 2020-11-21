use crate::*;

pub struct Code {
	pub code: Vec<String>,
	pub index: usize,
}

impl Code {
	pub fn from_file(_files: &[&str]) -> Code {
		unimplemented!()
	}

	pub fn new() -> Self {
		Code {
			code: vec![String::new()],
			index: 0,
		}
	}

	pub fn next(&'_ mut self) -> Result<(&'_ str, bool), CustomErr> {
		self.index += 1;
		let mut interactive = false;
		while self.index >= self.code.len() {
			let mut input_line = String::new();
			io::stdin().read_line(&mut input_line)?;
			self.code.push(input_line);
			interactive = true;
		}
		Ok((&self.code[self.index], interactive))
	}
}

impl Default for Code {
	fn default() -> Self {
		Code::new()
	}
}
