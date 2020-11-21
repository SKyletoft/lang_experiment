use std::{error, fmt};

pub type CustomErr = Box<dyn error::Error>;

#[derive(Copy, Clone, Debug)]
pub struct ParseError {}
impl error::Error for ParseError {}
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "parse error!")
	}
}
pub fn perr() -> CustomErr {
	Box::new(ParseError {})
}

#[derive(Copy, Clone, Debug)]
pub struct SyntaxError {}
impl error::Error for SyntaxError {}
impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "syntax error!")
	}
}
pub fn serr() -> CustomErr {
	Box::new(SyntaxError {})
}

#[derive(Copy, Clone, Debug)]
pub struct TypeError {}
impl error::Error for TypeError {}
impl fmt::Display for TypeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "type error!")
	}
}
pub fn terr() -> CustomErr {
	Box::new(TypeError {})
}
