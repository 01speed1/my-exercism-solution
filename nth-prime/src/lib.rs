fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    true
}

pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {n}th prime number?")
    if n == 0 {
        return 2;
    }

    if n == 1 {
        return 3;
    }

    let mut primes = vec![];
    primes.push(2);
    primes.push(3);
    let mut current_count = 4;
    while primes.len() <= (n as usize) {
        if is_prime(current_count) {
            primes.push(current_count);
        }
        current_count = current_count + 1;
    }

    primes.last().unwrap().to_owned() as u32
}
