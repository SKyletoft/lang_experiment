use crate::variable::Variable::*;
use crate::*;

use std::collections::HashMap;
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

pub fn evaluate_statement(
	words: &[&str],
	variables: &HashMap<String, Variable>,
) -> Result<Variable, CustomErr> {
	let float = floats::evaluate_floats(words, &variables);
	if float.is_ok() {
		return float;
	}
	let b = bools::evaluate_bools(words, &variables);
	if b.is_ok() {
		return b;
	}
	Err(perr())
}
