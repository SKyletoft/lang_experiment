use crate::*;

pub fn split(s: &'_ str) -> Result<Vec<&'_ str>, CustomErr> {
	let keep_closure = |slice: &str| slice.chars().any(|c| !c.is_whitespace());
	let mut vec = Vec::new();
	let mut parentheses = 0;
	let mut brackets = 0;
	let mut start = 0;
	let mut quotes = 0;
	let mut escape = false;
	for (i, c) in s.char_indices() {
		match (brackets, parentheses, quotes, c) {
			(0, 0, 0, '[') => {
				let slice = &s[start..i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i;
				brackets += 1;
			}
			(_, 0, 0, '[') => {
				brackets += 1;
			}
			(1, 0, 0, ']') => {
				let slice = &s[start..=i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i + 1;
				brackets -= 1;
			}
			(_, 0, 0, ']') => {
				brackets -= 1;
			}

			(0, 0, 0, '(') => {
				let slice = &s[start..i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i;
				parentheses += 1;
			}
			(0, _, 0, '(') => {
				parentheses += 1;
			}
			(0, 1, 0, ')') => {
				let slice = &s[start..=i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i + 1;
				parentheses -= 1;
			}
			(0, _, 0, ')') => {
				parentheses -= 1;
			}

			(0, 0, 0, '"') if !escape => {
				let slice = &s[start..i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i;
				quotes += 1;
			}
			(0, 0, 1, '"') if !escape => {
				let slice = &s[start..=i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i + 1;
				quotes -= 1;
			}

			(0, 0, 0, _) if c.is_whitespace() => {
				let slice = &s[start..i];
				if keep_closure(slice) {
					vec.push(slice);
				}
				start = i + 1;
			}

			(0, 0, 1, '\\') => {
				escape = true;
				continue;
			}
			_ => {}
		}
		escape = false;
	}
	let slice = &s[start..];
	if keep_closure(slice) {
		vec.push(slice);
	}
	if parentheses == 0 && brackets == 0 && quotes == 0 {
		Ok(vec)
	} else {
		perr!()
	}
}

pub fn remove_parentheses(s: &'_ str) -> &'_ str {
	if is_list(s) || has_parentheses(s) || is_string(s) {
		let l = s.len();
		&s[1..l - 1]
	} else {
		s
	}
}

pub fn is_list(s: &str) -> bool {
	let b = s.as_bytes();
	let l = s.len();
	let last = l.wrapping_sub(1);
	b.get(0) == Some(&b'[') && b.get(last) == Some(&b']')
}

pub fn has_parentheses(s: &str) -> bool {
	let b = s.as_bytes();
	let l = s.len();
	let last = l.wrapping_sub(1);
	b.get(0) == Some(&b'(') && b.get(last) == Some(&b')')
}

pub fn is_string(s: &str) -> bool {
	let b = s.as_bytes();
	let l = s.len();
	let last = l.wrapping_sub(1);
	b.get(0) == Some(&b'"') && b.get(last) == Some(&b'"')
}
