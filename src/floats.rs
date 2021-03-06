use crate::*;

#[derive(Clone, Debug, PartialEq)]
enum Op<'a> {
	Add(Box<Op<'a>>, Box<Op<'a>>),
	Sub(Box<Op<'a>>, Box<Op<'a>>),
	Mul(Box<Op<'a>>, Box<Op<'a>>),
	Div(Box<Op<'a>>, Box<Op<'a>>),
	Mod(Box<Op<'a>>, Box<Op<'a>>),
	Pow(Box<Op<'a>>, Box<Op<'a>>),
	Val(Variable),
	Unparsed(&'a str),
}
use Op::*;
type OpFnPtr<'a> = fn(Box<Op<'a>>, Box<Op<'a>>) -> Op<'a>;

fn evaluate_float(num: &str) -> Result<Variable, CustomErr> {
	if !num.bytes().all(|x| x.is_ascii_digit() || x == b'.') {
		return perr!();
	}
	let mut splits = num.split('.');
	match (splits.next(), splits.next(), splits.next()) {
		(Some(_), None, None) => Ok(Number(parse_int(num) as f64)),
		(Some(_), Some(_), None) => {
			let number = parse_int(num) as f64;
			let dot_index = num.bytes().rev().position(|c| c == b'.').unwrap_or(0) as i32;
			Ok(Number(number / 10f64.powi(dot_index)))
		}
		_ => perr!(),
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
		return perr!();
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
		variable::evaluate_statement(&helper::split(helper::remove_parentheses(s))?, variables)?
	} else if let Some(n) = variables.get(s) {
		n.clone()
	} else if let Ok(n) = evaluate_float(s) {
		n
	} else {
		return perr!();
	};
	variable::assert_type_of(&val, &NumberT)?;
	Ok(val)
}

fn eval_op(op: Op, variables: &Variables) -> Result<f64, CustomErr> {
	Ok(match op {
		Add(l, r) => eval_op(*l, variables)? + eval_op(*r, variables)?,
		Sub(l, r) => eval_op(*l, variables)? - eval_op(*r, variables)?,
		Mul(l, r) => eval_op(*l, variables)? * eval_op(*r, variables)?,
		Div(l, r) => eval_op(*l, variables)? / eval_op(*r, variables)?,
		Mod(l, r) => eval_op(*l, variables)? % eval_op(*r, variables)?,
		Pow(l, r) => eval_op(*l, variables)?.powf(eval_op(*r, variables)?),
		Val(Number(x)) => x,
		Unparsed(s) => variable::un_number(&parse_or_get(s, variables)?)?,
		_ => return perr!(),
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

fn order_of_operations_parse(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	let mut words: Vec<Op> = words.iter().map(|x| Unparsed(x)).collect();

	let operator_fn_pair: [(&str, OpFnPtr); 6] = [
		("^", |lhs, rhs| Pow(lhs, rhs)),
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
		return perr!();
	}

	Ok(Number(eval_op(words.remove(0), variables)?))
}

fn logic_parse(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() != 3 {
		return perr!();
	}
	let op = words[1];
	let f: fn(f64, f64) -> bool = match op {
		"==" => |l, r| (l - r).abs() < f64::EPSILON,
		"<=" => |l, r| l <= r,
		">=" => |l, r| l >= r,
		"<" => |l, r| l < r,
		">" => |l, r| l > r,
		_ => return perr!(),
	};
	let lhs = variable::un_number(&variable::evaluate_statement(&words[0..1], variables)?)?;
	let rhs = variable::un_number(&variable::evaluate_statement(&words[2..3], variables)?)?;

	Ok(Boolean(f(lhs, rhs)))
}

fn round_parse(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() != 2 {
		return perr!();
	}
	let f: fn(f64) -> f64 = match words[0] {
		"floor" => |x| x.floor(),
		"ceil" => |x| x.ceil(),
		"round" => |x| x.round(),
		"sqrt" => |x| x.sqrt(),
		_ => return perr!(),
	};
	let num = variable::un_number(&variable::evaluate_statement(&words[1..2], variables)?)?;
	Ok(Number(f(num)))
}

pub fn evaluate_floats(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	logic_parse(words, variables).or_else(|_| round_parse(words, variables)).or_else(|_| order_of_operations_parse(words, variables))
}
