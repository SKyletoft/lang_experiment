use crate::variable::Variable::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
	Boolean(bool),
	Number(f64),
	Char(char),
	List(Vec<Variable>),
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Boolean(b) => write!(f, "{}", b),
			Number(n) => write!(f, "{}", n),
			Char(c) => write!(f, "{}", c),
			List(l) => write!(f, "{:?}", l),
		}
	}
}
