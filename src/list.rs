use crate::*;

pub fn evaluate_list_or_string(
	words: &[&str],
	variables: &Variables,
) -> Result<Variable, CustomErr> {
	let list = evaluate_list(words, variables);
	if list.is_ok() {
		return list;
	}
	let string = evaluate_string(words);
	if string.is_ok() {
		return string;
	}
	Err(perr(line!(), file!()))
}

pub fn evaluate_list(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() != 1 {
		return Err(perr(line!(), file!()));
	}
	let word = words[0];
	if !helper::is_list(word) {
		return Err(perr(line!(), file!()));
	}

	let mut vec = Vec::new();
	let typ;
	let split = helper::split(helper::remove_parens(word))?;
	let mut iter = split.iter();
	if let Some(&var) = iter.next() {
		let parsed = variable::evaluate_statement(&helper::split(var)?, variables)?;
		typ = variable::to_type(&parsed);
		vec.push(parsed);
	} else {
		return Err(perr(line!(), file!()));
	}
	for &token in iter {
		let parsed = variable::evaluate_statement(&helper::split(token)?, variables)?;
		if variable::to_type(&parsed) != typ {
			return Err(perr(line!(), file!()));
		}
		vec.push(parsed);
	}
	Ok(List(typ, vec))
}

pub fn evaluate_string(words: &[&str]) -> Result<Variable, CustomErr> {
	if words.len() != 1 {
		return Err(perr(line!(), file!()));
	}
	let word = words[0];
	if helper::is_string(word) {
		Ok(List(
			CharT,
			helper::remove_parens(word).chars().map(Char).collect(),
		))
	} else {
		Err(perr(line!(), file!()))
	}
}

pub fn parse_list_and_index(
	list: Variable,
	index: Variable,
) -> Result<(VariableT, Vec<Variable>, usize), CustomErr> {
	if let (List(t, vec), Number(i)) = (list, index) {
		let index = i as usize;
		if vec.len() >= index {
			Ok((t, vec, index))
		} else {
			eprintln!("Out of bounds");
			Err(serr(line!(), file!()))
		}
	} else {
		Err(terr(line!(), file!()))
	}
}

pub fn remove_from_list(list: Variable, index: Variable) -> Result<Variable, CustomErr> {
	let (t, mut vec, index) = parse_list_and_index(list, index)?;
	vec.remove(index);
	Ok(List(t, vec))
}

pub fn add_to_list(list: Variable, index: Variable, item: Variable) -> Result<Variable, CustomErr> {
	let (t, mut vec, index) = parse_list_and_index(list, index)?;
	let typ = variable::to_type(&item);
	if t != typ {
		return Err(terr(line!(), file!()));
	}
	vec.insert(index, item);
	Ok(List(t, vec))
}

pub fn list_len(list: &Variable) -> Result<usize, CustomErr> {
	if let List(_, l) = list {
		Ok(l.len())
	} else {
		Err(terr(line!(), file!()))
	}
}

pub fn join_lists(lhs: Variable, rhs: Variable) -> Result<Variable, CustomErr> {
	if let (List(typ1, mut list), List(typ2, mut other_list)) = (lhs, rhs) {
		if typ1 != typ2 {
			return Err(terr(line!(), file!()));
		}
		list.append(&mut other_list);
		Ok(List(typ1, list))
	} else {
		Err(terr(line!(), file!()))
	}
}

pub fn get_item(list: Variable, index: Variable) -> Result<Variable, CustomErr> {
	let (_, mut vec, index) = parse_list_and_index(list, index)?;
	Ok(vec.remove(index))
}

pub fn list_op(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(serr(line!(), file!()));
	}
	let list = if words
		.get(0)
		.map(|s| !helper::is_list(s) && !helper::is_string(s))
		== Some(true)
	{
		if words.len() == 1 {
			return Err(perr(line!(), file!()));
		}
		variable::evaluate_statement(&words[..1], variables)?
	} else {
		evaluate_list_or_string(&words[..1], variables)?
	};
	if !variable::to_type(&list).is_list_t() {
		return Err(terr(line!(), file!()));
	}

	let len = Number(list_len(&list)? as f64);
	let val = match words {
		[] => list,
		[_, "len"] => len,
		[_, "+", item] => {
			add_to_list(list, len, variable::evaluate_statement(&[item], variables)?)?
		}
		[_, "+", index, item] => add_to_list(
			list,
			variable::evaluate_statement(&[index], variables)?,
			variable::evaluate_statement(&[item], variables)?,
		)?,
		[_, "-", index] => {
			remove_from_list(list, variable::evaluate_statement(&[index], variables)?)?
		}
		[_, "++", rhs] => join_lists(list, variable::evaluate_statement(&[rhs], variables)?)?,
		[_, "@", index] => get_item(list, variable::evaluate_statement(&[index], variables)?)?,
		_ => return Err(perr(line!(), file!())),
	};
	Ok(val)
}
