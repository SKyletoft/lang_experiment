pub fn split(s: &'_ str) -> Vec<&'_ str> {
	let mut vec = Vec::new();
	let mut parentheses = 0;
	let mut brackets = 0;
	let mut start = 0;
	for (i, c) in s.char_indices() {
		if brackets == 0 {
			if c == '(' {
				if parentheses == 0 {
					vec.push(&s[start..i]);
					start = i;
				}
				parentheses += 1;
				continue;
			}
			if c == ')' {
				if parentheses == 1 {
					vec.push(&s[start..=i]);
					start = i + 1;
				}
				parentheses -= 1;
			}
		}
		if parentheses == 0 {
			if c == '[' {
				if brackets == 0 {
					vec.push(&s[start..i]);
					start = i;
				}
				brackets += 1;
				continue;
			}
			if c == ']' {
				if brackets == 1 {
					vec.push(&s[start..=i]);
					start = i + 1;
				}
				brackets -= 1;
			}
		}
		if c.is_whitespace() && parentheses == 0 && brackets == 0 {
			vec.push(&s[start..i]);
			start = i + 1;
		}
	}
	vec.push(&s[start..]);
	vec.retain(|slice| slice.chars().any(|c| !c.is_whitespace()));
	vec
}

pub fn remove_parens(s: &'_ str) -> &'_ str {
	let b = s.as_bytes();
	let l = s.len();
	if l >= 1
		&& ((b.get(0) == Some(&b'(') && b.get(l - 1) == Some(&b')'))
			|| (b.get(0) == Some(&b'[') && b.get(l - 1) == Some(&b']')))
	{
		&s[1..l - 1]
	} else {
		s
	}
}
