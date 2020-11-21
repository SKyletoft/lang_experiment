use crate::*;

pub struct Code {
	pub code: Vec<String>,
	pub index: usize,
}

impl Code {
	pub fn from_file(file: &str) -> Result<Code, CustomErr> {
		let mut code = Code::new();
		code.import(file)?;
		Ok(code)
	}

	pub fn new() -> Self {
		Code {
			code: vec![String::new()],
			index: 0,
		}
	}

	pub fn import(&mut self, file: &str) -> Result<(), CustomErr> {
		let mut file_content = fs::read_to_string(file)?.into_bytes();
		for byte in file_content.iter_mut() {
			*byte = match *byte {
				b'\n' => b' ',
				b';' => b'\n',
				c => c,
			}
		}
		let file_content = String::from_utf8(file_content)?;
		for line in file_content.lines() {
			self.code.push(line.to_owned());
		}
		Ok(())
	}

	pub fn next_line(&'_ mut self) -> Result<(&'_ str, bool), CustomErr> {
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
