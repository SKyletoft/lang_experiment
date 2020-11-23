use crate::*;

pub struct Code {
	code_internal: String,
	pub code: Vec<(usize, usize)>,
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
			code_internal: String::new(),
			code: Vec::new(),
			index: usize::MAX,
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
			self.push_line(line);
		}
		Ok(())
	}

	fn push_line(&mut self, line: &str) {
		let line_start = self.code_internal.len();
		self.code_internal.push_str(line);
		self.code.push((line_start, self.code_internal.len()));
	}

	fn get_line(&'_ self, index: usize) -> Result<&'_ str, CustomErr> {
		let (start, end) = self.code.get(index).ok_or_else(|| serr(line!(), file!()))?;
		self.code_internal.get(*start..*end).ok_or_else(|| serr(line!(), file!()))
	}

	pub fn next_line(&'_ mut self) -> Result<(&'_ str, bool), CustomErr> {
		self.index += 1;
		let mut interactive = false;
		while self.index >= self.code.len() {
			let mut input_line = String::new();
			io::stdin().read_line(&mut input_line)?;
			self.push_line(&input_line);
			interactive = true;
		}
		Ok((self.get_line(self.index)?, interactive))
	}
}

impl Default for Code {
	fn default() -> Self {
		Code::new()
	}
}
