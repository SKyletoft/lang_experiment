use std::{error, fmt};

pub type CustomErr = Box<dyn error::Error>;

#[derive(Copy, Clone, Debug)]
pub struct ParseError {line: u32, file: &'static str}
impl error::Error for ParseError {}
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "parse error!")
	}
}
pub fn perr(line: u32, file: &'static str) -> CustomErr {
	Box::new(ParseError {line, file})
}

#[derive(Copy, Clone, Debug)]
pub struct SyntaxError {line: u32, file: &'static str}
impl error::Error for SyntaxError {}
impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "syntax error!")
	}
}
pub fn serr(line: u32, file: &'static str) -> CustomErr {
	Box::new(SyntaxError {line, file})
}

#[derive(Copy, Clone, Debug)]
pub struct TypeError {line: u32, file: &'static str}
impl error::Error for TypeError {}
impl fmt::Display for TypeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "type error!")
	}
}
pub fn terr(line: u32, file: &'static str) -> CustomErr {
	Box::new(TypeError {line, file})
}
