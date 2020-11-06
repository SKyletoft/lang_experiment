use crate::*;

pub struct Code {
	pub code: Vec<String>,
	pub index: usize,
}

impl Code {
	pub fn from_file(files: &[&str]) -> Code {
		unimplemented!()
	}

	pub fn new() -> Self {
		Code {
			code: vec![String::new()],
			index: 0,
		}
	}
	
	pub fn next(&'_ mut self) -> Result<&'_ str, CustomErr> {
		self.index += 1;
		while self.index >= self.code.len() {
			let mut input_line = String::new();
			io::stdin().read_line(&mut input_line)?;
			self.code.push(input_line);
		}
		Ok(&self.code[self.index])
	}
}

impl Default for Code {
	fn default() -> Self {
		Code::new()
	}
}