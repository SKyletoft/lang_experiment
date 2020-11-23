use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Code {
	code_internal: String,
	code: Vec<(usize, usize)>,
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
		let mut is_comment = false;
		for byte in file_content.iter_mut() {
			*byte = match *byte {
				b'\n' => {
					is_comment = false;
					b' '
				}
				b'#' => {
					is_comment = true;
					b' '
				}
				b';' => b'\n',
				_ if is_comment => b' ',
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
		let comment_start = line
			.chars()
			.position(|c| c == '#')
			.unwrap_or_else(|| line.len());
		let trimmed = (&line[..comment_start]).trim();
		if trimmed.is_empty() {
			return;
		}
		let line_start = self.code_internal.len();
		self.code_internal.push_str(trimmed);
		self.code.push((line_start, self.code_internal.len()));
	}

	fn get_line(&'_ self, index: usize) -> Result<&'_ str, CustomErr> {
		self.code
			.get(index)
			.map(|(s, e)| self.code_internal.get(*s..*e))
			.flatten()
			.ok_or_else(|| Box::new(perrE!()) as Box<dyn std::error::Error>)
	}

	pub fn next_line(&'_ mut self) -> Result<(&'_ str, bool), CustomErr> {
		self.index = self.index.wrapping_add(1);
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
