use crate::*;

fn create_variable(words: &[&str], variables: &mut Variables) -> Result<Variable, CustomErr> {
	let res = match &words[1..] {
		[] => variables
			.get("last")
			.ok_or_else(|| serr(line!(), file!()))?
			.clone(),
		["=", ..] => variable::evaluate_statement(&words[2..], variables)?,
		[typ, "=", ..] => {
			let rest = &words[3..];
			match typ.parse::<VariableT>()? {
				NumberT => floats::evaluate_floats(rest, variables)?,
				BooleanT => bools::evaluate_bools(rest, variables)?,
				CharT => chars::char_op(rest, variables)?,
				ListT(typ) => {
					let parsed = list::list_op(rest, variables)?;
					if variable::to_type(&parsed) != *typ {
						return Err(terr(line!(), file!()));
					}
					parsed
				}
			}
		}
		_ => return Err(perr(line!(), file!())),
	};
	let name = variable::owned_name(words.get(0))?;
	variables.insert(name, res.clone());
	Ok(res)
}

fn if_statement(
	words: &[&str],
	variables: &Variables,
	skipping_if: &mut isize,
) -> Result<Variable, CustomErr> {
	let parsed = variable::evaluate_statement(words, variables)?;
	let b = variable::un_bool(&parsed)?;
	if !b {
		*skipping_if += 1;
	}
	Ok(parsed)
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

fn print_string(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	let (typ, vec) = variable::un_list(variable::evaluate_statement(words, variables)?)?;
	if typ != CharT {
		return Err(terr(line!(), file!()));
	}
	let stdout = io::stdout();
	let mut lock = stdout.lock();
	for letter in vec.iter() {
		write!(lock, "{}", letter)?;
	}
	writeln!(lock)?;
	Ok(Boolean(true))
}

fn print_type(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	let var = variable::evaluate_statement(words, variables)?;
	println!("> {}", variable::to_type(&var));
	Ok(var)
}

fn clear() -> Result<Variable, CustomErr> {
	unimplemented!()
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
	let &word = words.get(0).ok_or_else(|| perr(line!(), file!()))?;
	let &target = labels.get(word).ok_or_else(|| perr(line!(), file!()))?;
	*jump_next = Some(target);
	Ok(Boolean(true))
}

fn jump_rel(
	words: &[&str],
	variables: &Variables,
	index: usize,
	jump_next: &mut Option<usize>,
) -> Result<Variable, CustomErr> {
	let n = variable::un_number(&variable::evaluate_statement(words, variables)?)?;
	*jump_next = Some((index as isize).saturating_add(n as isize) as usize);
	Ok(Number(n))
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
	let args = {
		let mut vec = Vec::with_capacity(words.len() / 2);
		for (name, typ) in words
			.iter()
			.skip(1)
			.step_by(2)
			.zip(words.iter().step_by(2).skip(1))
		{
			let name = variable::owned_name(Some(name))?;
			let typ = typ.parse()?;
			vec.push((name, typ));
		}
		vec
	};
	let name = variable::owned_name(words.get(0))?;
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
	let return_value = variables
		.remove("last")
		.ok_or_else(|| serr(line!(), file!()))?;
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
	if words.len() != 2 {
		return Err(perr(line!(), file!()));
	}
	if !helper::has_parentheses(words[1]) {
		return Err(perr(line!(), file!()));
	}
	let (args_req, pointer) = functions
		.get(words[0])
		.ok_or_else(|| perr(line!(), file!()))?;
	let args = helper::split(helper::remove_parens(words[1]))?;
	let mut new_vars = HashMap::new();
	new_vars.insert("last".to_string(), Boolean(false));
	for ((name, typ), &arg) in args_req.iter().zip(args.iter()) {
		let split = helper::split(helper::remove_parens(arg))?;
		let parsed = variable::evaluate_statement(&split, variables)?;
		if variable::to_type(&parsed) != *typ {
			return Err(terr(line!(), file!()));
		}
		new_vars.insert(name.clone(), parsed);
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

pub fn run(mut code: Code) -> Result<(), CustomErr> {
	eprint!(
		"Source at:\thttps://github.com/SKyletoft/lang_experiment\nCompiled at:\t{}",
		include_str!("../target/date.txt")
	);
	let mut variables: Variables = HashMap::new();
	let mut labels: Labels = HashMap::new();
	let mut functions: Functions = HashMap::new();
	let mut call_stack: CallStack = Vec::new();
	let mut jump_next: Option<usize> = None;
	let mut creating_function: isize = 0;
	let mut skipping_if: isize = 0;

	variables.insert("last".to_owned(), Boolean(false));

	loop {
		let index = code.index + 1;
		let (input_line, interactive) = code.next_line()?;
		let words = {
			let words = helper::split(input_line.trim());
			if words.is_err() {
				eprintln!("{:3}: {:?}", index, words);
				continue;
			}
			words.expect("This can't happen")
		};

		if words.is_empty() {
			continue;
		}
		if creating_function >= 1 {
			if words[0] == "end" {
				creating_function -= 1;
			}
			continue;
		}
		if skipping_if >= 1 {
			match words[0] {
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
			"endif" => Ok(Boolean(true)),
			"print" => print(rest, &variables),
			"clear" => clear(),
			"label" => create_labels(rest, &mut labels, index),
			"jump" => jump(rest, &labels, &mut jump_next),
			"jump_rel" => jump_rel(rest, &variables, index, &mut jump_next),
			"type" => print_type(rest, &variables),
			"end" => exit_function(&mut variables, &mut call_stack, &mut jump_next),
			"return" => exit_function(&mut variables, &mut call_stack, &mut jump_next),
			"fn" => create_function(rest, &mut functions, index, &mut creating_function),
			"puts" => print_string(rest, &variables),
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
			*variables
				.get_mut("last")
				.ok_or_else(|| serr(line!(), file!()))? = last;
		} else {
			eprintln!("{:3}: {:?}", index, result);
			*variables
				.get_mut("last")
				.ok_or_else(|| serr(line!(), file!()))? = Boolean(false);
		}
		if let Some(target) = jump_next {
			code.index = target;
			jump_next = None;
		}
	}
}
