use crate::*;

pub fn evaluate_list(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() != 1 {
		return Err(perr());
	}
	let word = words[0];
	if word.as_bytes()[0] != b'[' || word.as_bytes()[word.len() - 1] != b']' {
		return Err(perr());
	}

	let mut vec = Vec::new();
	let typ;
	let split = helper::split(helper::remove_parens(word));
	let mut iter = split.iter();
	if let Some(&var) = iter.next() {
		let parsed = variable::evaluate_statement(&helper::split(var), variables)?;
		typ = variable::to_type(&parsed);
		vec.push(parsed);
	} else {
		return Err(perr());
	}
	for &token in iter {
		let parsed = variable::evaluate_statement(&helper::split(token), variables)?;
		if variable::to_type(&parsed) != typ {
			return Err(perr());
		}
	}
	Ok(List(typ, vec))
}

pub fn parse_list_and_index(
	list: Variable,
	index: Variable,
) -> Result<(VariableT, Vec<Variable>, usize), CustomErr> {
	if let (List(t, vec), Number(i)) = (list, index) {
		let index = i as usize;
		if vec.len() < index {
			Ok((t, vec, index))
		} else {
			Err(serr())
		}
	} else {
		Err(perr())
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
		return Err(terr());
	}
	vec.insert(index, item);
	Ok(List(t, vec))
}
