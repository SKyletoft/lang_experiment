fn prime_gen(max: usize) -> Vec<usize> {
	let mut primes = vec![2,3,5,7,11,13];
	let mut candidate = 13;
	loop {
		let length = primes.len();
		candidate += 2;
		let mut index = 0;
		if length == max {
			return primes;
		}
		loop {
			if index == length {
				primes.push(candidate);
				break;
			}
			if (candidate % primes[index]) == 0 {
				break;
			}
			index += 1;
		}
	}
}

fn main() {
	println!("{:?}", prime_gen(112500));
}