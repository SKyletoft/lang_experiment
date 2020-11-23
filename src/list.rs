use crate::*;

pub fn evaluate_list(
	words: &[&str],
	variables: &Variables,
	ascii: bool,
) -> Result<Variable, CustomErr> {
	if words.len() != 1 {
		return Err(perr(line!(), file!()));
	}
	let word = words[0];
	if !helper::is_list(word) {
		return Err(perr(line!(), file!()));
	}

	let mut vec = Vec::new();
	let typ;
	let split = helper::split(helper::remove_parens(word), ascii)?;
	let mut iter = split.iter();
	if let Some(&var) = iter.next() {
		let parsed = variable::evaluate_statement(&helper::split(var, ascii)?, variables, ascii)?;
		typ = variable::to_type(&parsed);
		vec.push(parsed);
	} else {
		return Err(perr(line!(), file!()));
	}
	for &token in iter {
		let parsed = variable::evaluate_statement(&helper::split(token, ascii)?, variables, ascii)?;
		if variable::to_type(&parsed) != typ {
			return Err(perr(line!(), file!()));
		}
		vec.push(parsed);
	}
	Ok(List(typ, vec))
}

pub fn evaluate_string(word: &str) -> Result<Variable, CustomErr> {
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
	let (typ, vec) = variable::un_list(list)?;
	let i = variable::un_number(&index)? as usize;
	if vec.len() >= i {
		Ok((typ, vec, i))
	} else {
		eprintln!("Out of bounds");
		Err(serr(line!(), file!()))
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
	let (typ_l, mut list_l) = variable::un_list(lhs)?;
	let (typ_r, mut list_r) = variable::un_list(rhs)?;
	if typ_l != typ_r {
		return Err(terr(line!(), file!()));
	}
	list_l.append(&mut list_r);
	Ok(List(typ_l, list_l))
}

pub fn get_item(list: Variable, index: Variable) -> Result<Variable, CustomErr> {
	let (_, mut vec, index) = parse_list_and_index(list, index)?;
	Ok(vec.remove(index))
}

pub fn list_op(words: &[&str], variables: &Variables, ascii: bool) -> Result<Variable, CustomErr> {
	if words.is_empty() {
		return Err(serr(line!(), file!()));
	}
	let first = words[0];
	let words = &words[1..];
	let list = if helper::is_list(first) {
		evaluate_list(&[first], variables, ascii)?
	} else if helper::is_string(first) {
		evaluate_string(first)?
	} else {
		if words.is_empty() {
			return Err(perr(line!(), file!()));
		}
		variable::evaluate_statement(&[first], variables, ascii)?
	};
	if !variable::to_type(&list).is_list_t() {
		return Err(terr(line!(), file!()));
	}

	let len = Number(list_len(&list)? as f64);
	let val = match words {
		[] => list,
		["len"] => len,
		["+", item] => add_to_list(
			list,
			len,
			variable::evaluate_statement(&[item], variables, ascii)?,
		)?,
		["+", index, item] => add_to_list(
			list,
			variable::evaluate_statement(&[index], variables, ascii)?,
			variable::evaluate_statement(&[item], variables, ascii)?,
		)?,
		["-", index] => remove_from_list(
			list,
			variable::evaluate_statement(&[index], variables, ascii)?,
		)?,
		["++", rhs] => join_lists(
			list,
			variable::evaluate_statement(&[rhs], variables, ascii)?,
		)?,
		["@", index] => get_item(
			list,
			variable::evaluate_statement(&[index], variables, ascii)?,
		)?,
		_ => return Err(perr(line!(), file!())),
	};
	Ok(val)
}
