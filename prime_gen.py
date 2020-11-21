def prime_gen(max):
	primes = [2,3,5,7,11,13]
	candidate = 13
	while (True):
		length = len(primes)
		candidate += 2
		index = 0
		if (length == max):
			return primes
		while(True):
			if index == length:
				primes.append(candidate)
				break
			if (candidate % primes[index]) == 0:
				break
			index += 1

print(prime_gen(10000))