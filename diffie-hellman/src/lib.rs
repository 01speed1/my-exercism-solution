use rand::{thread_rng, Rng};

fn pow_mod(base: u128, exponent: u64, modulus: u64) -> u64 {
    let mut base = base;
    let mut exponent = exponent;
    let mut result = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus as u128;
        }
        base = (base * base) % modulus as u128;
        exponent /= 2;
    }

    result as u64
}

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _even if n % 2 == 0 => false,
        _ => {
            let sqrt_limit = (n as f64).sqrt() as u64;
            !(3..=sqrt_limit).step_by(2).any(|i| n % i == 0)
        }
    }
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();

    loop {
        let random_number = rng.gen_range(1..p);
        if is_prime(random_number) {
            return random_number;
        }
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_mod(b_pub as u128, a, p)
}
