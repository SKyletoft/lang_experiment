use crate::*;

pub fn split(s: &'_ str) -> Result<Vec<&'_ str>, CustomErr> {
	let mut vec = Vec::new();
	let mut parentheses = 0;
	let mut brackets = 0;
	let mut start = 0;
	let mut quotes = 0;
	let mut escape = false;
	for (i, c) in s.char_indices() {
		match (brackets, parentheses, quotes, c) {
			(0, 0, 0, '[') => {
				vec.push(&s[start..i]);
				start = i;
				brackets += 1;
			}
			(_, 0, 0, '[') => {
				brackets += 1;
			}
			(1, 0, 0, ']') => {
				vec.push(&s[start..=i]);
				start = i + 1;
				brackets -= 1;
			}
			(_, 0, 0, ']') => {
				brackets -= 1;
			}

			(0, 0, 0, '(') => {
				vec.push(&s[start..i]);
				start = i;
				parentheses += 1;
			}
			(0, _, 0, '(') => {
				parentheses += 1;
			}
			(0, 1, 0, ')') => {
				vec.push(&s[start..=i]);
				start = i + 1;
				parentheses -= 1;
			}
			(0, _, 0, ')') => {
				parentheses -= 1;
			}

			(0, 0, 0, '"') if !escape => {
				vec.push(&s[start..i]);
				start = i;
				quotes += 1;
			}
			(0, 0, 1, '"') if !escape => {
				vec.push(&s[start..=i]);
				start = i + 1;
				quotes -= 1;
			}

			(0, 0, 0, _) if c.is_whitespace() => {
				vec.push(&s[start..i]);
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
	vec.push(&s[start..]);
	vec.retain(|slice| slice.chars().any(|c| !c.is_whitespace()));
	if parentheses == 0 && brackets == 0 && quotes == 0 {
		Ok(vec)
	} else {
		dbg!(s);
		dbg!(vec);
		dbg!((brackets, parentheses, quotes));
		Err(perr(line!(), file!()))
	}
}

pub fn remove_parens(s: &'_ str) -> &'_ str {
	let l = s.len();
	if is_list(s) || has_parentheses(s) {
		&s[1..l - 1]
	} else {
		s
	}
}

pub fn is_list(s: &str) -> bool {
	let b = s.as_bytes();
	let l = s.len();
	b.get(0) == Some(&b'[') && b.get(l - 1) == Some(&b']')
}

pub fn has_parentheses(s: &str) -> bool {
	let b = s.as_bytes();
	let l = s.len();
	b.get(0) == Some(&b'(') && b.get(l - 1) == Some(&b')')
}