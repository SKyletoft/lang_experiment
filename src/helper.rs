pub fn split(s: &'_ str) -> Vec<&'_ str> {
	let mut vec = Vec::new();
	let mut brackets = 0;
	let mut start = 0;
	for (i, c) in s.char_indices() {
		if c == '(' {
			if brackets == 0 {
				vec.push(&s[start..i]);
				start = i;
			}
			brackets += 1;
			continue;
		}
		if c == ')' {
			if brackets == 1 {
				vec.push(&s[start..=i]);
				start = i + 1;
			}
			brackets -= 1;
		}
		if c.is_whitespace() && brackets == 0 {
			vec.push(&s[start..i]);
			start = i + 1;
		}
	}
	vec.push(&s[start..]);
	vec.retain(|slice| slice.chars().any(|c| !c.is_whitespace()));
	vec
}