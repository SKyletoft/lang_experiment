#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;
use Variable::*;

type CustomErr = Box<dyn std::error::Error>;

#[derive(Copy, Clone, Debug)]
struct ParseError {}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "parse error!")
	}
}

fn err() -> CustomErr {
	Box::new(ParseError {})
}

enum Variable {
	Integer(i64),
	Floating(f64),
	Char(char),
	List(Vec<Variable>),
}

fn evaluate(words: &[&str]) -> Result<Variable, CustomErr> {
	if words[0].as_bytes()[0] == b'[' {}
	Ok(Integer(0))
}

fn evaluate_float(num: &str) -> Result<Variable, CustomErr> {
	if !num.chars().all(|x| ('0' <= x && x <= '9') || x == '.')
	{
		return Err(err());
	}
	let mut splits = num.split('.');
	let first = trusted_parse_int(splits.next().unwrap()) as f64;
	let res = match num.chars().filter(|&x| x == '.').count() {
		0 => Floating(first),
		1 => {
			let word = splits.next().unwrap();
			let second = trusted_parse_int(word) as f64;
			Floating(first + second / 10f64.powi(word.len() as i32))
		}
		_ => return Err(err()),
	};
	Ok(res)
}

fn trusted_parse_int(string: &str) -> u64 {
	string.bytes().fold(0, |acc, curr| acc * 10 + (curr - b'0') as u64)
}

fn evaluate_floats(words: &[&str]) -> Result<Variable, CustomErr> {
	enum Op {
		Add, Sub, Mul, Div, Val(Variable)
	}
	enum Tree {
		Branch(Op, Box<Tree>),
		Leaf(Variable)
	}
	use Op::*;
	let mut parsed = Vec::new();
	for &x in words.iter() {
		parsed.push(match x {
			"+" => Add,
			"-" => Sub,
			"*" => Mul,
			"/" => Div,
			_ => {
				let val = evaluate_float(&x)?;
				Val(val)
			}
		});
	}
	
	Ok(Integer(0))
}

fn create_variable(
	variables: &mut HashMap<String, Variable>,
	words: &[&str],
) -> Result<(), CustomErr> {
	let name = words[0].to_string();
	if words[2] != "=" {
		return Err(err());
	}
	match words[1] {
		"List" => unimplemented!(),
		"Float" | "f64" => evaluate_floats(&words[4]),
	}
	Ok(())
}
fn if_statement() -> Result<(), CustomErr> {
	Ok(())
}
fn print() -> Result<(), CustomErr> {
	Ok(())
}
fn clear() -> Result<(), CustomErr> {
	Ok(())
}

fn main() -> Result<(), CustomErr> {
	let mut variables: HashMap<String, Variable> = HashMap::new();
	let mut input_line = String::new();
	//let mut input_line_words = Vec::new();
	let stdin = std::io::stdin();
	loop {
		input_line.clear();
		stdin.read_line(&mut input_line)?;
		let words = input_line
			.trim()
			.split_whitespace()
			//.map(|x| x.split(','))
			//.flatten()
			.collect::<Vec<&str>>();
		if words.is_empty() {
			continue;
		}
		dbg!(&words);
		let result = match words[0] {
			"let" => create_variable(&mut variables, &words[1..]),
			"if" => if_statement(),
			"print" => print(),
			"clear" => clear(),
			_ => unimplemented!(),
		};
		if result.is_err() {
			println!("Parse error");
		}
	}
	Ok(())
}
