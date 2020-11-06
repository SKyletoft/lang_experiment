use std::collections::HashMap;
use std::io;

pub mod bools;
pub mod errors;
pub mod file;
pub mod floats;
pub mod helper;
pub mod variable;
use errors::*;
use variable::{evaluate_statement, Variable, Variable::*, VariableT, VariableT::*};

fn create_variable(
	words: &[&str],
	variables: &mut HashMap<String, Variable>,
) -> Result<Variable, CustomErr> {
	if words.is_empty() || words.len() == 2 || words.get(0) == Some(&"last") {
		return Err(perr());
	}
	let new = if words.len() == 1 {
		variables.get("last").ok_or_else(serr)?.clone()
	} else if words.get(1) == Some(&"=") {
		variable::evaluate_statement(&words[2..], variables)?
	} else if words.get(2) == Some(&"=") {
		match words[1].parse::<VariableT>()? {
			NumberT => floats::evaluate_floats(&words[3..], &variables)?,
			BooleanT => bools::evaluate_bools(&words[3..], &variables)?,
			ListT(_) => unimplemented!(),
			CharT => unimplemented!(),
		}
	} else {
		return Err(perr());
	};
	let name = words[0].to_string();
	variables.insert(name, new.clone());
	Ok(new)
}

fn if_statement(
	words: &[&str],
	variables: &HashMap<String, Variable>,
	skipping_if: &mut isize,
) -> Result<Variable, CustomErr> {
	dbg!(words);
	if let Boolean(b) = bools::evaluate_bools(words, variables)? {
		if !b {
			*skipping_if += 1;
		}
		Ok(Boolean(b))
	} else {
		Err(serr())
	}
}

fn print(words: &[&str], variables: &mut HashMap<String, Variable>) -> Result<Variable, CustomErr> {
	print!("> ");
	for &word in words {
		let result = variables.get(word).ok_or_else(perr)?;
		print!("{} ", result);
	}
	println!();
	Ok(Boolean(true))
}

fn print_type(var: Variable) -> Result<Variable, CustomErr> {
	println!(
		"> {}",
		match var {
			Number(_) => "Number",
			List(_) => "List of _", //THIS IS TOTALLY SOLVABLE. BUT I MIGHT COMPLETELY CHANGE HOW LISTS WORK
			Char(_) => "Char",
			Boolean(_) => "Boolean",
		}
	);
	Ok(var)
}

fn clear() -> Result<Variable, CustomErr> {
	Ok(Boolean(true))
}

fn create_labels(
	words: &[&str],
	labels: &mut HashMap<String, usize>,
	index: usize,
) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(perr());
	}
	labels.insert(words[0].to_string(), index);
	Ok(Boolean(true))
}

fn jump(
	words: &[&str],
	labels: &HashMap<String, usize>,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(perr());
	}
	let &target = labels.get(words[0]).ok_or_else(perr)?;
	*jump_next = Some(target);
	Ok(Boolean(true))
}

fn jump_rel(
	words: &[&str],
	variables: &HashMap<String, Variable>,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(perr());
	}
	if let Number(n) = floats::evaluate_floats(words, variables)? {
		*jump_next = Some((index as isize).saturating_add(n as isize) as usize);
		Ok(Number(n))
	} else {
		Err(serr())
	}
}

fn create_function(
	words: &[&str],
	functions: &mut HashMap<String, (Vec<(String, VariableT)>, usize)>,
	index: usize,
	creating_function: &mut isize,
) -> Result<Variable, CustomErr> {
	if words.len() % 2 == 0 {
		return Err(serr());
	}
	let name = words[0].to_owned();
	let args = {
		let mut vec = Vec::with_capacity(words.len() / 2);
		for (name, typ) in words
			.iter()
			.skip(1)
			.step_by(2)
			.zip(words.iter().step_by(2).skip(1))
		{
			let name = name.to_string();
			let typ = typ.parse()?;
			vec.push((name, typ));
		}
		vec
	};
	functions.insert(name, (args, index));
	*creating_function += 1;
	Ok(Boolean(true))
}

fn exit_function(
	variables: &mut HashMap<String, Variable>,
	call_stack: &mut Vec<(HashMap<String, Variable>, usize)>,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	if call_stack.is_empty() {
		return Err(serr());
	}
	let return_value = variables.remove("last").ok_or_else(serr)?;
	let (revert_stack, return_adr) = call_stack.remove(call_stack.len() - 1);
	*jump_next = Some(return_adr);
	*variables = revert_stack;
	Ok(return_value)
}

fn function_call(
	words: &[&str],
	variables: &mut HashMap<String, Variable>,
	functions: &HashMap<String, (Vec<(String, VariableT)>, usize)>,
	call_stack: &mut Vec<(HashMap<String, Variable>, usize)>,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	let (args_req, pointer) = functions.get(words[0]).ok_or_else(perr)?;
	let args = helper::split(&words[1][1..words[1].len() - 1]);
	let mut new_vars = HashMap::new();
	new_vars.insert("last".to_string(), Boolean(false));
	for ((name, typ), &arg) in args_req.iter().zip(args.iter()) {
		let split = helper::split(arg);
		let parsed = match typ {
			NumberT => floats::evaluate_floats(&split, variables)?,
			BooleanT => bools::evaluate_bools(&split, variables)?,
			//CharT => floats::evaluate_floats(arg, variables)?,
			//ListT(_)
			_ => return Err(perr()),
		};
		new_vars.insert(name.to_string(), parsed);
	}
	call_stack.push((variables.clone(), index));
	*variables = new_vars;
	*jump_next = Some(*pointer);
	Ok(Boolean(false))
}

fn solve_function_or_variable(
	words: &[&str],
	variables: &mut HashMap<String, Variable>,
	functions: &HashMap<String, (Vec<(String, VariableT)>, usize)>,
	call_stack: &mut Vec<(HashMap<String, Variable>, usize)>,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	//dbg!(words);
	if words.len() == 1 {
		let res = variables.get(words[0]).ok_or_else(perr);
		if res.is_ok() {
			Ok(res?.clone())
		} else {
			variable::evaluate_statement(words, variables)
		}
	} else {
		let call = function_call(words, variables, functions, call_stack, index, jump_next);
		if call.is_ok() {
			call
		} else {
			variable::evaluate_statement(words, variables)
		}
	}
}

fn main() -> Result<(), CustomErr> {
	let mut variables: HashMap<String, Variable> = HashMap::new();
	let mut labels: HashMap<String, usize> = HashMap::new();
	let mut functions: HashMap<String, (Vec<(String, VariableT)>, usize)> = HashMap::new();
	let mut call_stack: Vec<(HashMap<String, Variable>, usize)> = Vec::new();
	let mut jump_next: Option<usize> = None;
	let mut code = file::Code::new();
	let mut creating_function: isize = 0;
	let mut skipping_if: isize = 0;

	variables.insert("last".to_owned(), Boolean(false));

	loop {
		let index = code.index + 1;
		let (input_line, interactive) = code.next()?;
		let words = helper::split(input_line.trim());

		if words.is_empty() {
			continue;
		}
		if creating_function >= 1 {
			if words[0].trim() == "end" {
				creating_function -= 1;
			}
			continue;
		}
		if skipping_if >= 0 {
			match words[0].trim() {
				"endif" if skipping_if == 1 => {
					skipping_if = 0;
					continue;
				}
				"endif" => skipping_if -= 1,
				"if" => skipping_if += 1,
				_ => {}
			}
		}

		let result = match words[0] {
			"let" => create_variable(&words[1..], &mut variables),
			"if" => if_statement(&words[1..], &variables, &mut skipping_if),
			"print" => print(&words[1..], &mut variables),
			"clear" => clear(),
			"label" => create_labels(&words[1..], &mut labels, index),
			"jump" => jump(&words[1..], &labels, &mut jump_next),
			"jump_rel" => jump_rel(&words[1..], &variables, index, &mut jump_next),
			"type" => print_type(evaluate_statement(&words[1..], &variables)?),
			"end" => exit_function(&mut variables, &mut call_stack, &mut jump_next),
			"fn" => create_function(&words[1..], &mut functions, index, &mut creating_function),
			_ => solve_function_or_variable(
				&words,
				&mut variables,
				&functions,
				&mut call_stack,
				index,
				&mut jump_next,
			),
		};
		if let Ok(last) = result {
			if interactive && creating_function == 0 && call_stack.is_empty() {
				println!("> {}", &last);
			}
			*variables.get_mut("last").ok_or_else(serr)? = last;
		} else {
			println!("{:?}", result);
		}
		if let Some(target) = jump_next {
			code.index = target;
			jump_next = None;
		}
	}
}
