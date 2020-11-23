use std::{error, fmt};

pub type CustomErr = Box<dyn error::Error>;

#[derive(Copy, Clone, Debug)]
pub enum CodeError {
	Parse { line: u32, file: &'static str },
	Syntax { line: u32, file: &'static str },
	Type { line: u32, file: &'static str },
}

impl error::Error for CodeError {}
impl fmt::Display for CodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CodeError::Parse { line: _, file: _ } => write!(f, "parse error!"),
			CodeError::Syntax { line: _, file: _ } => write!(f, "syntax error!"),
			CodeError::Type { line: _, file: _ } => write!(f, "type error!"),
		}
	}
}
#[macro_export]
macro_rules! perr {
	() => {
		Err(Box::new(CodeError::Parse {
			line: line!(),
			file: file!(),
			}))
	};
}
#[macro_export]
macro_rules! perrE {
	() => {
		CodeError::Parse {
			line: line!(),
			file: file!(),
			}
	};
}
#[macro_export]
macro_rules! serr {
	() => {
		Err(Box::new(CodeError::Syntax {
			line: line!(),
			file: file!(),
			}))
	};
}
#[macro_export]
macro_rules! serrE {
	() => {
		CodeError::Syntax {
			line: line!(),
			file: file!(),
			}
	};
}

#[macro_export]
macro_rules! terr {
	() => {
		Err(Box::new(CodeError::Type {
			line: line!(),
			file: file!(),
			}))
	};
}
#[macro_export]
macro_rules! terrE {
	() => {
		CodeError::Type {
			line: line!(),
			file: file!(),
			}
	};
}
