use crate::{
	variable::{Variable::*, *},
	*,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Op<'a> {
	Add(Box<Op<'a>>, Box<Op<'a>>),
	Sub(Box<Op<'a>>, Box<Op<'a>>),
	Mul(Box<Op<'a>>, Box<Op<'a>>),
	Div(Box<Op<'a>>, Box<Op<'a>>),
	Val(Variable),
	Unparsed(&'a str),
}
use Op::*;

fn evaluate_float(num: &str) -> Result<Variable, CustomErr> {
	if !num.chars().all(|x| ('0' <= x && x <= '9') || x == '.') {
		return Err(perr());
	}
	let mut splits = num.split('.');
	let first = trusted_parse_int(splits.next().unwrap()) as f64;
	let res = match num.chars().filter(|&x| x == '.').count() {
		0 => Number(first),
		1 => {
			let word = splits.next().unwrap();
			let second = trusted_parse_int(word) as f64;
			Number(first + second / 10f64.powi(word.len() as i32))
		}
		_ => return Err(perr()),
	};
	Ok(res)
}

fn trusted_parse_int(string: &str) -> u64 {
	string
		.bytes()
		.fold(0, |acc, curr| acc * 10 + (curr - b'0') as u64)
}
fn get_left_and_right<'a>(
	idx: &mut usize,
	words: &mut Vec<Op<'a>>,
	variables: &HashMap<String, Variable>,
) -> Result<(Op<'a>, Op<'a>), CustomErr> {
	if *idx == 0 {
		return Err(perr());
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
fn parse_or_get(s: &str, variables: &HashMap<String, Variable>) -> Result<Variable, CustomErr> {
	if let Ok(n) = evaluate_float(s) {
		Ok(n)
	} else if let Some(n) = variables.get(s) {
		Ok(n.clone())
	} else {
		Err(perr())
	}
}
fn eval_op(op: Op<'_>, variables: &HashMap<String, Variable>) -> Result<f64, CustomErr> {
	//dbg!(&op);
	Ok(match op {
		Add(l, r) => eval_op(*l, variables)? + eval_op(*r, variables)?,
		Sub(l, r) => eval_op(*l, variables)? - eval_op(*r, variables)?,
		Mul(l, r) => eval_op(*l, variables)? * eval_op(*r, variables)?,
		Div(l, r) => eval_op(*l, variables)? / eval_op(*r, variables)?,
		Val(Number(x)) => x,
		Unparsed(s) => {
			if let Number(n) = parse_or_get(s, variables)? {
				n
			} else {
				return Err(terr());
			}
		}
		_ => return Err(perr()),
	})
}
pub fn evaluate_floats<'a>(
	words: &[&'a str],
	variables: &HashMap<String, Variable>,
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
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("+")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Add(Box::new(left), Box::new(right));
	}
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed("-")) {
		let (left, right) = get_left_and_right(&mut idx, &mut words, variables)?;
		words[idx] = Sub(Box::new(left), Box::new(right));
	}

	if words.len() != 1 {
		return Err(perr());
	}

	Ok(Number(eval_op(words.remove(0), variables)?))
}
