use std::collections::HashMap;
use std::{io, io::Write};

pub mod bools;
pub mod errors;
pub mod file;
pub mod floats;
pub mod helper;
pub mod list;
pub mod variable;
use errors::*;
use variable::{
	CallStack, Functions, Labels, Variable, Variable::*, VariableT, VariableT::*, Variables,
};

const KEYWORDS: [&str; 15] = [
	"let", "if", "endif", "print", "clear", "label", "jump", "jump_rel", "type", "end", "fn",
	"last", "len", "exit", "return"
];

fn create_variable(words: &[&str], variables: &mut Variables) -> Result<Variable, CustomErr> {
	if words.is_empty() || words.len() == 2 || words.get(0) == Some(&"last") {
		return Err(perr(line!(), file!()));
	}
	if !variable::is_ok(words[0]) {
		return Err(serr(line!(), file!()));
	}
	let new = if words.len() == 1 {
		variables.get("last").ok_or_else(|| serr(line!(), file!()))?.clone()
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
		return Err(perr(line!(), file!()));
	};
	let name = words[0].to_string();
	variables.insert(name, new.clone());
	Ok(new)
}

fn if_statement(
	words: &[&str],
	variables: &Variables,
	skipping_if: &mut isize,
) -> Result<Variable, CustomErr> {
	if let Boolean(b) = variable::evaluate_statement(words, variables)? {
		if !b {
			*skipping_if += 1;
		}
		Ok(Boolean(b))
	} else {
		Err(terr(line!(), file!()))
	}
}

fn print(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	let stdout = io::stdout();
	let mut lock = stdout.lock();
	write!(lock, "> ")?;
	for &word in words {
		let result = variables.get(word).ok_or_else(|| perr(line!(), file!()))?;
		write!(lock, "{} ", result)?;
	}
	writeln!(lock)?;
	Ok(Boolean(true))
}

fn print_type(var: Variable) -> Result<Variable, CustomErr> {
	println!("> {}", variable::to_type(&var));
	Ok(var)
}

fn clear() -> Result<Variable, CustomErr> {
	Ok(Boolean(true))
}

fn create_labels(words: &[&str], labels: &mut Labels, index: usize) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(perr(line!(), file!()));
	}
	labels.insert(words[0].to_string(), index);
	Ok(Boolean(true))
}

fn jump(
	words: &[&str],
	labels: &Labels,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(perr(line!(), file!()));
	}
	let &target = labels.get(words[0]).ok_or_else(|| perr(line!(), file!()))?;
	*jump_next = Some(target);
	Ok(Boolean(true))
}

fn jump_rel(
	words: &[&str],
	variables: &Variables,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(perr(line!(), file!()));
	}
	if let Number(n) = floats::evaluate_floats(words, variables)? {
		*jump_next = Some((index as isize).saturating_add(n as isize) as usize);
		Ok(Number(n))
	} else {
		Err(serr(line!(), file!()))
	}
}

fn create_function(
	words: &[&str],
	functions: &mut Functions,
	index: usize,
	creating_function: &mut isize,
) -> Result<Variable, CustomErr> {
	if words.len() % 2 == 0 {
		return Err(serr(line!(), file!()));
	}
	if !variable::is_ok(words[0]) {
		return Err(serr(line!(), file!()));
	}
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
	let name = words[0].to_owned();
	functions.insert(name, (args, index));
	*creating_function += 1;
	Ok(Boolean(true))
}

fn exit_function(
	variables: &mut Variables,
	call_stack: &mut CallStack,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	if call_stack.is_empty() {
		return Err(serr(line!(), file!()));
	}
	let return_value = variables.remove("last").ok_or_else(|| serr(line!(), file!()))?;
	let (revert_stack, return_adr) = call_stack.remove(call_stack.len() - 1);
	*jump_next = Some(return_adr);
	*variables = revert_stack;
	Ok(return_value)
}

fn function_call(
	words: &[&str],
	variables: &mut Variables,
	functions: &Functions,
	call_stack: &mut CallStack,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	let (args_req, pointer) = functions.get(words[0]).ok_or_else(|| perr(line!(), file!()))?;
	let args = helper::split(&words[1][1..words[1].len() - 1])?;
	let mut new_vars = HashMap::new();
	new_vars.insert("last".to_string(), Boolean(false));
	for ((name, typ), &arg) in args_req.iter().zip(args.iter()) {
		let split = helper::split(arg)?;
		let parsed = match typ {
			NumberT => floats::evaluate_floats(&split, variables)?,
			BooleanT => bools::evaluate_bools(&split, variables)?,
			//CharT => floats::evaluate_floats(arg, variables)?,
			ListT(_) => list::evaluate_list(words, variables)?,
			_ => return Err(perr(line!(), file!())),
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
	variables: &mut Variables,
	functions: &Functions,
	call_stack: &mut CallStack,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	let call = function_call(words, variables, functions, call_stack, index, jump_next);
	if call.is_ok() {
		call
	} else {
		variable::evaluate_statement(words, variables)
	}
}

fn main() -> Result<(), CustomErr> {
	let mut variables: Variables = HashMap::new();
	let mut labels: Labels = HashMap::new();
	let mut functions: Functions = HashMap::new();
	let mut call_stack: CallStack = Vec::new();
	let mut jump_next: Option<usize> = None;
	let mut code = file::Code::new();
	let mut creating_function: isize = 0;
	let mut skipping_if: isize = 0;

	variables.insert("last".to_owned(), Boolean(false));

	loop {
		let index = code.index + 1;
		let (input_line, interactive) = code.next_line()?;
		let words = {
			let words = helper::split(input_line.trim());
			if words.is_err() {
				println!("{:?}", words);
				continue;
			}
			words.unwrap()
		};

		if words.is_empty() {
			continue;
		}
		if creating_function >= 1 {
			if words[0].trim() == "end" {
				creating_function -= 1;
			}
			continue;
		}
		if skipping_if > 0 {
			match words[0].trim() {
				"endif" if skipping_if == 1 => skipping_if = 0,
				"endif" => skipping_if -= 1,
				"if" => skipping_if += 1,
				_ => {}
			}
			continue;
		}

		let rest = &words[1..];
		let result = match words[0] {
			"exit" => {
				return Ok(());
			}
			"let" => create_variable(rest, &mut variables),
			"if" => if_statement(rest, &variables, &mut skipping_if),
			"print" => print(rest, &variables),
			"clear" => clear(),
			"label" => create_labels(rest, &mut labels, index),
			"jump" => jump(rest, &labels, &mut jump_next),
			"jump_rel" => jump_rel(rest, &variables, index, &mut jump_next),
			"type" => print_type(variable::evaluate_statement(rest, &variables)?),
			"end" => exit_function(&mut variables, &mut call_stack, &mut jump_next),
			"return" => exit_function(&mut variables, &mut call_stack, &mut jump_next),
			"fn" => create_function(rest, &mut functions, index, &mut creating_function),
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
			*variables.get_mut("last").ok_or_else(|| serr(line!(), file!()))? = last;
		} else {
			println!("{:?}", result);
			*variables.get_mut("last").ok_or_else(|| serr(line!(), file!()))? = Boolean(false);
		}
		if let Some(target) = jump_next {
			code.index = target;
			jump_next = None;
		}
	}
}
