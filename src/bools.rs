use crate::*;

#[derive(Clone, Debug, PartialEq)]
enum Op<'a> {
	And(Box<Op<'a>>, Box<Op<'a>>),
	Or(Box<Op<'a>>, Box<Op<'a>>),
	Xor(Box<Op<'a>>, Box<Op<'a>>),
	Eq(Box<Op<'a>>, Box<Op<'a>>),
	Not(Box<Op<'a>>),
	Val(Variable),
	Unparsed(&'a str),
}
use Op::*;

fn evaluate_bool(b: &str) -> Result<Variable, CustomErr> {
	match b {
		"true" => Ok(Boolean(true)),
		"false" => Ok(Boolean(false)),
		_ => Err(perr(line!(), file!())),
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

fn float_eq<'a>(
	idx: &mut usize,
	words: &mut Vec<Op<'a>>,
	variables: &Variables,
) -> Result<Op<'a>, CustomErr> {
	if *idx == 0 {
		return Err(perr(line!(), file!()));
	}
	let left = match words.remove(*idx - 1) {
		Unparsed(s) => floats::parse_or_get(s, variables)?,
		_ => return Err(terr(line!(), file!())),
	};
	*idx -= 1;
	let right = match words.remove(*idx + 1) {
		Unparsed(s) => floats::parse_or_get(s, variables)?,
		_ => return Err(terr(line!(), file!())),
	};
	Ok(Val(Boolean(left == right)))
}

fn get_right<'a>(
	idx: &usize,
	words: &mut Vec<Op<'a>>,
	variables: &Variables,
) -> Result<Op<'a>, CustomErr> {
	if *idx >= words.len() - 1 {
		return Err(perr(line!(), file!()));
	}
	let right = match words.remove(*idx + 1) {
		Unparsed(s) => Val(parse_or_get(s, variables)?),
		x => x,
	};
	Ok(right)
}

fn parse_or_get(s: &str, variables: &Variables) -> Result<Variable, CustomErr> {
	let val = if helper::has_parentheses(s) {
		variable::evaluate_statement(&helper::split(helper::remove_parens(s))?, variables)?
	} else if let Some(n) = variables.get(s) {
		n.clone()
	} else if let Ok(n) = evaluate_bool(s) {
		n
	} else {
		return Err(perr(line!(), file!()));
	};
	if variable::to_type(&val) == BooleanT {
		Ok(val)
	} else {
		Err(terr(line!(), file!()))
	}
}

fn eval_op(op: Op<'_>, variables: &Variables) -> Result<bool, CustomErr> {
	Ok(match op {
		And(l, r) => eval_op(*l, variables)? && eval_op(*r, variables)?,
		Or(l, r) => eval_op(*l, variables)? || eval_op(*r, variables)?,
		Xor(l, r) => eval_op(*l, variables)? ^ eval_op(*r, variables)?,
		Eq(l, r) => eval_op(*l, variables)? == eval_op(*r, variables)?,
		Not(l) => !eval_op(*l, variables)?,
		Val(Boolean(x)) => x,
		Unparsed(s) => {
			if let Boolean(n) = parse_or_get(s, variables)? {
				n
			} else {
				return Err(terr(line!(), file!()));
			}
		}
		_ => return Err(perr(line!(), file!())),
	})
}

pub fn evaluate_bools<'a>(words: &[&'a str], variables: &Variables) -> Result<Variable, CustomErr> {
	let mut words: Vec<Op<'a>> = words.iter().map(|x| Unparsed(x)).collect();

	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("&")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = And(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("|")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Or(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("^")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Xor(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("==")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Eq(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("===")) {
		words[idx] = float_eq(&mut idx, &mut words, variables)?;
	}
	while let Some(idx) = words.iter().position(|x| *x == Unparsed("!")) {
		let right = get_right(&idx, &mut words, variables)?;
		words[idx] = Not(Box::new(right));
	}

	if words.len() != 1 {
		return Err(perr(line!(), file!()));
	}

	Ok(Boolean(eval_op(words.remove(0), variables)?))
}
