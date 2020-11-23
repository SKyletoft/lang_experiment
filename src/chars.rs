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

pub fn parse_or_get(s: &str, variables: &Variables, ascii: bool) -> Result<Variable, CustomErr> {
	let val = if helper::has_parentheses(s) {
		variable::evaluate_statement(
			&helper::split(helper::remove_parens(s), ascii)?,
			variables,
			ascii,
		)?
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

pub fn char_op(words: &[&str], variables: &Variables, ascii: bool) -> Result<Variable, CustomErr> {
	match words {
		["n", statement] => {
			let c = variable::un_char(&parse_or_get(statement, variables, ascii)?)?;
			Ok(Number(c as u8 as f64))
		}
		["dig", statement] => {
			let c = variable::un_number(&floats::parse_or_get(statement, variables, ascii)?)?;
			if c < 0. || 9. < c {
				Err(serr(line!(), file!()))
			} else {
				Ok(Char((c as u8 + b'0') as char))
			}
		}
		["num", statement] => {
			let c = variable::un_number(&floats::parse_or_get(statement, variables, ascii)?)?;
			Ok(List(CharT, format!("{}", c).chars().map(Char).collect()))
		}
		["c", statement] => {
			let n = variable::un_number(&floats::parse_or_get(statement, variables, ascii)?)?;
			Ok(Char(n as u8 as char))
		}
		[lhs, op, rhs] => {
			let f = match *op {
				"==" => |l, r| l == r,
				"<=" => |l, r| l <= r,
				">=" => |l, r| l >= r,
				"<" => |l, r| l < r,
				">" => |l, r| l > r,
				_ => return Err(perr(line!(), file!())),
			};
			let l = variable::un_char(&parse_or_get(lhs, variables, ascii)?)?;
			let r = variable::un_char(&parse_or_get(rhs, variables, ascii)?)?;
			Ok(Boolean(f(l, r)))
		}
		[statement] => parse_or_get(statement, variables, ascii),
		_ => Err(perr(line!(), file!())),
	}
}
