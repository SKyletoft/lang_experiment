use crate::*;

#[derive(Clone, Debug, PartialEq)]
enum Op<'a> {
	Add(Box<Op<'a>>, Box<Op<'a>>),
	Sub(Box<Op<'a>>, Box<Op<'a>>),
	Mul(Box<Op<'a>>, Box<Op<'a>>),
	Div(Box<Op<'a>>, Box<Op<'a>>),
	Mod(Box<Op<'a>>, Box<Op<'a>>),
	Val(Variable),
	Unparsed(&'a str),
}
use Op::*;

fn evaluate_char(chr: &str) -> Result<Variable, CustomErr> {
	let mut chars = chr.chars();
	let first = chars.next();
	let second = chars.next();
	let third = chars.next();
	let fourth = chars.next();
	match (first, second, third, fourth) {
		(Some('\''), Some(c), Some('\''), None) => Ok(Char(c)),
		_ => Err(perr(line!(), file!()))
	}
}

fn get_left_and_right<'a>(
	idx: &mut usize,
	words: &mut Vec<Op<'a>>,
	variables: &Variables,
) -> Result<(Op<'a>, Op<'a>), CustomErr> {
	if *idx == 0 {
		return Err(perr(line!(), file!()));
	}
	let left = match words.remove(*idx - 1) {
		Unparsed(s) => Val(parse_or_get(s, variables)?),
		x => x,
	};
	*idx -= 1;
	let right = match words.remove(*idx + 1) {
		Unparsed(s) => Val(parse_or_get(s, variables)?),
		x => x,
	};
	Ok((left, right))
}

pub fn parse_or_get(s: &str, variables: &Variables) -> Result<Variable, CustomErr> {
	let val = if helper::has_parentheses(s) {
		variable::evaluate_statement(&helper::split(helper::remove_parens(s))?, variables)?
	} else if let Some(n) = variables.get(s) {
		n.clone()
	} else if let Ok(n) = evaluate_float(s) {
		n
	} else {
		return Err(perr(line!(), file!()));
	};
	if variable::to_type(&val) == NumberT {
		Ok(val)
	} else {
		Err(terr(line!(), file!()))
	}
}

fn eval_op(op: Op<'_>, variables: &Variables) -> Result<f64, CustomErr> {
	Ok(match op {
		Add(l, r) => eval_op(*l, variables)? + eval_op(*r, variables)?,
		Sub(l, r) => eval_op(*l, variables)? - eval_op(*r, variables)?,
		Mul(l, r) => eval_op(*l, variables)? * eval_op(*r, variables)?,
		Div(l, r) => eval_op(*l, variables)? / eval_op(*r, variables)?,
		Mod(l, r) => eval_op(*l, variables)? % eval_op(*r, variables)?,
		Val(Number(x)) => x,
		Unparsed(s) => {
			if let Number(n) = parse_or_get(s, variables)? {
				n
			} else {
				return Err(terr(line!(), file!()));
			}
		}
		_ => return Err(perr(line!(), file!())),
	})
}

pub fn evaluate_floats<'a>(
	words: &[&'a str],
	variables: &Variables,
) -> Result<Variable, CustomErr> {
	let mut words: Vec<Op<'a>> = words.iter().map(|x| Unparsed(x)).collect();
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("*")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Mul(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("/")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Div(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("%")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Mod(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("+")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Add(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("-")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Sub(Box::new(left), Box::new(right));
	}

	if words.len() != 1 {
		return Err(perr(line!(), file!()));
	}

	Ok(Number(eval_op(words.remove(0), variables)?))
}
