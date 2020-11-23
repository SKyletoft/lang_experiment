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
				write!(f, "({}, {})[ ", t, l.len())?;
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
	type Err = CustomErr;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let res = if helper::has_parentheses(s) {
			let split = helper::split(helper::remove_parentheses(s))?;
			match split.as_slice() {
				["list", typ] => ListT(Box::new(typ.parse()?)),
				[_, _] => return terr!(),
				_ => return serr!(),
			}
		} else {
			match s {
				"f64" => NumberT,
				"num" => NumberT,
				"bool" => BooleanT,
				"char" => CharT,
				_ => return serr!(),
			}
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

impl From<Variable> for VariableT {
	fn from(rhs: Variable) -> Self {
		to_type(&rhs)
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

pub fn assert_type_of(var: &Variable, typ: &VariableT) -> Result<(), CustomErr> {
	assert_type(&to_type(var), typ)
}
pub fn assert_list_type_of(var: &Variable) -> Result<(), CustomErr> {
	assert_list_type(&to_type(var))
}

pub fn assert_type(t1: &VariableT, t2: &VariableT) -> Result<(), CustomErr> {
	if *t1 == *t2 {
		Ok(())
	} else {
		terr!()
	}
}

pub fn assert_list_type(t: &VariableT) -> Result<(), CustomErr> {
	if let ListT(_) = t {
		Ok(())
	} else {
		terr!()
	}
}

pub fn un_number(var: &Variable) -> Result<f64, CustomErr> {
	if let Number(n) = var {
		Ok(*n)
	} else {
		terr!()
	}
}
pub fn un_bool(var: &Variable) -> Result<bool, CustomErr> {
	if let Boolean(n) = var {
		Ok(*n)
	} else {
		terr!()
	}
}
pub fn un_char(var: &Variable) -> Result<char, CustomErr> {
	if let Char(n) = var {
		Ok(*n)
	} else {
		terr!()
	}
}
pub fn un_list(var: Variable) -> Result<(VariableT, Vec<Variable>), CustomErr> {
	if let List(t, v) = var {
		Ok((t, v))
	} else {
		terr!()
	}
}

pub fn evaluate_statement(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	match words {
		[] => perr!(),
		[s] if helper::has_parentheses(s) => {
			evaluate_statement(&helper::split(helper::remove_parentheses(words[0]))?, variables)
		}
		[s] if variables.contains_key(*s) => Ok(variables.get(*s).expect("Unreachable?").clone()),
		_ => floats::evaluate_floats(words, variables)
			.or_else(|_| bools::evaluate_bools(words, variables))
			.or_else(|_| chars::char_op(words, variables))
			.or_else(|_| list::list_op(words, variables))
			.map_err(|_| Box::new(perrE!()) as Box<dyn std::error::Error>),
	}
}

pub fn is_ok(name: &str) -> bool {
	!KEYWORDS.contains(&name)
		&& !name.is_empty()
		&& name.as_bytes().get(0).map(|d| d.is_ascii_digit()) != Some(true)
		&& !helper::is_list(name)
		&& !helper::is_string(name)
		&& !helper::has_parentheses(name)
}

pub fn owned_name(name: Option<&&str>) -> Result<String, CustomErr> {
	let name = name.ok_or(perrE!())?;
	if is_ok(name) {
		Ok(name.to_string())
	} else {
		serr!()
	}
}
