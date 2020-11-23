use crate::*;

fn evaluate_list(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.len() != 1 {
		return perr!();
	}
	let word = words[0];
	if !helper::is_list(word) {
		return perr!();
	}

	let mut vec = Vec::new();
	let typ;
	let split = helper::split(helper::remove_parentheses(word))?;
	let mut iter = split.iter();
	if let Some(&var) = iter.next() {
		let parsed = variable::evaluate_statement(&helper::split(var)?, variables)?;
		typ = variable::to_type(&parsed);
		vec.push(parsed);
	} else {
		return perr!();
	}
	for &token in iter {
		let parsed = variable::evaluate_statement(&helper::split(token)?, variables)?;
		variable::assert_type_of(&parsed, &typ)?;
		vec.push(parsed);
	}
	Ok(List(typ, vec))
}

fn evaluate_string(word: &str) -> Result<Variable, CustomErr> {
	if helper::is_string(word) {
		Ok(List(
			CharT,
			helper::remove_parentheses(word).chars().map(Char).collect(),
		))
	} else {
		perr!()
	}
}

fn parse_list_and_index(
	list: Variable,
	index: Variable,
) -> Result<(VariableT, Vec<Variable>, usize), CustomErr> {
	let (typ, vec) = variable::un_list(list)?;
	let i = variable::un_number(&index)? as usize;
	if vec.len() >= i {
		Ok((typ, vec, i))
	} else {
		eprintln!("Out of bounds");
		serr!()
	}
}

fn remove_from_list(list: Variable, index: Variable) -> Result<Variable, CustomErr> {
	let (t, mut vec, index) = parse_list_and_index(list, index)?;
	vec.remove(index);
	Ok(List(t, vec))
}

fn add_to_list(list: Variable, index: Variable, item: Variable) -> Result<Variable, CustomErr> {
	let (t, mut vec, index) = parse_list_and_index(list, index)?;
	variable::assert_type_of(&item, &t)?;
	vec.insert(index, item);
	Ok(List(t, vec))
}

fn list_len(list: &Variable) -> Result<usize, CustomErr> {
	if let List(_, l) = list {
		Ok(l.len())
	} else {
		terr!()
	}
}

fn join_lists(lhs: Variable, rhs: Variable) -> Result<Variable, CustomErr> {
	let (typ_l, mut list_l) = variable::un_list(lhs)?;
	let (typ_r, mut list_r) = variable::un_list(rhs)?;
	variable::assert_type(&typ_l, &typ_r)?;
	list_l.append(&mut list_r);
	Ok(List(typ_l, list_l))
}

fn get_item(list: Variable, index: Variable) -> Result<Variable, CustomErr> {
	let (_, mut vec, index) = parse_list_and_index(list, index)?;
	Ok(vec.remove(index))
}

pub fn list_op(words: &[&str], variables: &Variables) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return serr!();
	}
	let first = words[0];
	let words = &words[1..];
	let list = if helper::is_list(first) {
		evaluate_list(&[first], variables)?
	} else if helper::is_string(first) {
		evaluate_string(first)?
	} else {
		if words.is_empty() {
			return perr!();
		}
		variable::evaluate_statement(&[first], variables)?
	};
	if !variable::to_type(&list).is_list_t() {
		return terr!();
	}

	let len = Number(list_len(&list)? as f64);
	let val = match words {
		[] => list,
		["len"] => len,
		["+", item] => add_to_list(list, len, variable::evaluate_statement(&[item], variables)?)?,
		["+", index, item] => add_to_list(
			list,
			variable::evaluate_statement(&[index], variables)?,
			variable::evaluate_statement(&[item], variables)?,
		)?,
		["-", index] => remove_from_list(list, variable::evaluate_statement(&[index], variables)?)?,
		["++", rhs] => join_lists(list, variable::evaluate_statement(&[rhs], variables)?)?,
		["@", index] => get_item(list, variable::evaluate_statement(&[index], variables)?)?,
		_ => return perr!(),
	};
	Ok(val)
}
