#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

pub mod bools;
pub mod floats;
pub mod variable;
use variable::Variable;

type CustomErr = Box<dyn std::error::Error>;

#[derive(Copy, Clone, Debug)]
struct ParseError {}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "parse error!")
	}
}
fn perr() -> CustomErr {
	Box::new(ParseError {})
}

#[derive(Copy, Clone, Debug)]
struct TypeError {}
impl std::error::Error for TypeError {}
impl fmt::Display for TypeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "type error!")
	}
}
fn terr() -> CustomErr {
	Box::new(TypeError {})
}

fn create_variable(
	variables: &mut HashMap<String, Variable>,
	words: &[&str],
) -> Result<(), CustomErr> {
	let name = words[0].to_string();
	if words[2] != "=" {
		return Err(perr());
	}
	let new = match words[1] {
		"num" => floats::evaluate_floats(&words[3..], &variables)?,
		"bool" => bools::evaluate_bools(&words[3..], &variables)?,
		"list" => unimplemented!(),
		_ => {
			if let Ok(res) = floats::evaluate_floats(&words[2..], &variables) {
				res
			} else if let Ok(res) = bools::evaluate_bools(&words[2..], &variables) {
				res
			} else {
				return Err(perr());
			}
		}
	};
	variables.insert(name, new);
	Ok(())
}
fn if_statement() -> Result<(), CustomErr> {
	Ok(())
}
fn print(variables: &mut HashMap<String, Variable>, words: &[&str]) -> Result<(), CustomErr> {
	print!("> ");
	for &word in words {
		let result = variables.get(word).ok_or_else(perr)?;
		print!("{} ", result);
	}
	println!();
	Ok(())
}
fn clear() -> Result<(), CustomErr> {
	Ok(())
}

fn create_labels() -> Result<(), CustomErr> {
	Ok(())
}

fn main() -> Result<(), CustomErr> {
	let mut variables: HashMap<String, Variable> = HashMap::new();
	let mut line_hist = Vec::new();
	let mut _labels: HashMap<String, usize> = HashMap::new();
	//let mut input_line_words = Vec::new();
	let stdin = std::io::stdin();
	loop {
		let mut input_line = String::new();
		stdin.read_line(&mut input_line)?;
		let words = input_line.trim().split_whitespace().collect::<Vec<&str>>();
		if words.is_empty() {
			continue;
		}
		//dbg!(&words);
		let result = match words[0] {
			"let" => create_variable(&mut variables, &words[1..]),
			"if" => if_statement(),
			"print" => print(&mut variables, &words[1..]),
			"clear" => clear(),
			"label" => create_labels(),
			_ => unimplemented!(),
		};
		if result.is_err() {
			println!("Parse error");
		}
		line_hist.push(input_line);
	}
	//Ok(())
}
