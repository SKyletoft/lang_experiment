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
type OpFnPtr<'a> = fn(Box<Op<'a>>, Box<Op<'a>>) -> Op<'a>;

fn evaluate_float(num: &str) -> Result<Variable, CustomErr> {
	if !num.bytes().all(|x| x.is_ascii_digit() || x == b'.') {
		return Err(perr(line!(), file!()));
	}
	let mut splits = num.split('.');
	match (splits.next(), splits.next(), splits.next()) {
		(Some(_), None, None) => Ok(Number(parse_int(num) as f64)),
		(Some(_), Some(_), None) => {
			let number = parse_int(num) as f64;
			let dot_index = num.bytes().rev().position(|c| c == b'.').unwrap_or(0) as i32;
			Ok(Number(number / 10f64.powi(dot_index)))
		}
		_ => Err(perr(line!(), file!())),
	}
}

fn parse_int(string: &str) -> u64 {
	string
		.bytes()
		.filter(u8::is_ascii_digit)
		.fold(0, |acc, curr| acc * 10 + (curr - b'0') as u64)
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

fn eval_op(op: Op, variables: &Variables) -> Result<f64, CustomErr> {
	Ok(match op {
		Add(l, r) => eval_op(*l, variables)? + eval_op(*r, variables)?,
		Sub(l, r) => eval_op(*l, variables)? - eval_op(*r, variables)?,
		Mul(l, r) => eval_op(*l, variables)? * eval_op(*r, variables)?,
		Div(l, r) => eval_op(*l, variables)? / eval_op(*r, variables)?,
		Mod(l, r) => eval_op(*l, variables)? % eval_op(*r, variables)?,
		Val(Number(x)) => x,
		Unparsed(s) => variable::un_number(&parse_or_get(s, variables)?)?,
		_ => return Err(perr(line!(), file!())),
	})
}

fn perform_all_of_operation<'a>(
	words: &mut Vec<Op<'a>>,
	variables: &Variables,
	operator: &str,
	operation_function: OpFnPtr<'a>,
) -> Result<(), CustomErr> {
	while let Some(mut idx) = words.iter().position(|x| *x == Unparsed(operator)) {
		let (left, right) = get_left_and_right(&mut idx, words, variables)?;
		words[idx] = operation_function(Box::new(left), Box::new(right));
	}
	Ok(())
}

fn order_of_operations_parse(
	words: &[&str],
	variables: &Variables,
) -> Result<Variable, CustomErr> {
	let mut words: Vec<Op> = words.iter().map(|x| Unparsed(x)).collect();

	let operator_fn_pair: [(&str, OpFnPtr); 5] = [
		("*", |lhs, rhs| Mul(lhs, rhs)),
		("/", |lhs, rhs| Div(lhs, rhs)),
		("%", |lhs, rhs| Mod(lhs, rhs)),
		("+", |lhs, rhs| Add(lhs, rhs)),
		("-", |lhs, rhs| Sub(lhs, rhs)),
	];
	for (operator, node_type) in operator_fn_pair.iter() {
		perform_all_of_operation(&mut words, variables, operator, *node_type)?;
	}

	if words.len() != 1 {
		return Err(perr(line!(), file!()));
	}

	Ok(Number(eval_op(words.remove(0), variables)?))
}

fn logic_parse(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() != 3 {
		return Err(perr(line!(), file!()));
	}
	let lhs = variable::evaluate_statement(&words[0..1], variables)?;
	let rhs = variable::evaluate_statement(&words[2..3], variables)?;
	let lhs_n = variable::un_number(&lhs)?;
	let rhs_n = variable::un_number(&rhs)?;
	let op = words[1];

	let res = match op {
		"==" => lhs == rhs,
		"<" => lhs_n < rhs_n,
		"<=" => lhs_n <= rhs_n,
		">" => lhs_n > rhs_n,
		">=" => lhs_n >= rhs_n,
		_ => return Err(perr(line!(), file!())),
	};

	Ok(Boolean(res))
}

pub fn evaluate_floats(
	words: &[&str],
	variables: &Variables,
) -> Result<Variable, CustomErr> {
	let ooop = order_of_operations_parse(words, variables);
	if ooop.is_ok() {
		return ooop;
	}
	logic_parse(words, variables)
}
