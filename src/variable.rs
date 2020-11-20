use crate::*;

use std::collections::HashMap;
use std::fmt;

pub type Variables = HashMap<String, Variable>;
pub type Labels = HashMap<String, usize>;
pub type Functions = HashMap<String, (Vec<(String, VariableT)>, usize)>;
pub type CallStack = Vec<(HashMap<String, Variable>, usize)>;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
	Boolean(bool),
	Number(f64),
	Char(char),
	List(VariableT, Vec<Variable>),
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Boolean(b) => write!(f, "{}", b),
			Number(n) => write!(f, "{}", n),
			Char(c) => write!(f, "{}", c),
			List(t, l) => {
				write!(f, "({})[ ", t)?;
				for element in l.iter() {
					write!(f, "{} ", element)?;
				}
				write!(f, "]")
			}
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum VariableT {
	BooleanT,
	NumberT,
	CharT,
	ListT(Box<VariableT>),
}

impl std::str::FromStr for VariableT {
	type Err = SyntaxError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let res = match s.trim() {
			"f64" => NumberT,
			"num" => NumberT,
			"bool" => BooleanT,
			"char" => CharT,
			//"list" => ListT()
			_ => return Err(SyntaxError {}),
		};
		Ok(res)
	}
}

impl fmt::Display for VariableT {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let ListT(t) = self {
			write!(f, "List of {}", t)
		} else {
			write!(
				f,
				"{}",
				match self {
					NumberT => "Number",
					CharT => "Char",
					BooleanT => "Boolean",
					ListT(_) => unreachable!(),
				}
			)
		}
	}
}

impl VariableT {
    pub fn is_list_t(&self) -> bool {
        match self {
            ListT(_) => true,
            _ => false,
        }
    }
}

pub fn to_type(var: &Variable) -> VariableT {
	match var {
		Number(_) => NumberT,
		Char(_) => CharT,
		Boolean(_) => BooleanT,
		List(t, _) => ListT(Box::new(t.clone())),
	}
}

pub fn evaluate_statement(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() == 1 {
		if let Some(n) = variables.get(words[0]) {
			return Ok(n.clone());
		}
	}
	let float = floats::evaluate_floats(words, variables);
	if float.is_ok() {
		return float;
	}
	let b = bools::evaluate_bools(words, variables);
	if b.is_ok() {
		return b;
	}
	let list_op = list::list_op(words, variables);
	if list_op.is_ok() {
		return list_op;
	}
	
	Err(perr())
}

pub fn is_ok(name: &str) -> bool {
	!KEYWORDS.contains(&name)
        && !name.is_empty()
        && name.as_bytes().get(0).map(|d| d.is_ascii_digit()) != Some(true)
        && !helper::is_list(name)
}
