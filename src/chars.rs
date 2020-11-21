use crate::*;

fn evaluate_char(chr: &str) -> Result<Variable, CustomErr> {
	let mut chars = chr.chars();
	let first = chars.next();
	let second = chars.next();
	let third = chars.next();
	let fourth = chars.next();
	match (first, second, third, fourth) {
		(Some('\''), Some(c), Some('\''), None) => Ok(Char(c)),
		_ => Err(perr(line!(), file!())),
	}
}

pub fn parse_or_get(s: &str, variables: &Variables) -> Result<Variable, CustomErr> {
	let val = if helper::has_parentheses(s) {
		variable::evaluate_statement(&helper::split(helper::remove_parens(s))?, variables)?
	} else if let Some(n) = variables.get(s) {
		n.clone()
	} else if let Ok(n) = evaluate_char(s) {
		n
	} else {
		return Err(perr(line!(), file!()));
	};
	if variable::to_type(&val) == CharT {
		Ok(val)
	} else {
		Err(terr(line!(), file!()))
	}
}

pub fn char_op<'a>(words: &[&'a str], variables: &Variables) -> Result<Variable, CustomErr> {
	match words {
		["n", statement] => {
			if let Char(c) = parse_or_get(statement, variables)? {
				Ok(Number(c as u8 as f64))
			} else {
				Err(terr(line!(), file!()))
			}
		}
		["dig", statement] => {
			if let Number(c) = floats::parse_or_get(statement, variables)? {
				if c < 0. || 9. < c {
					Err(serr(line!(), file!()))
				} else {
					Ok(Char((c as u8 + b'0') as char))
				}
			} else {
				Err(terr(line!(), file!()))
			}
		}
		["num", statement] => {
			if let Number(c) = floats::parse_or_get(statement, variables)? {
				Ok(List(CharT, format!("{}", c).chars().map(Char).collect()))
			} else {
				Err(terr(line!(), file!()))
			}
		}
		["c", statement] => {
			if let Number(n) = floats::parse_or_get(statement, variables)? {
				Ok(Char(n as u8 as char))
			} else {
				Err(terr(line!(), file!()))
			}
		}
		[statement] => parse_or_get(statement, variables),
		_ => Err(perr(line!(), file!())),
	}
}
