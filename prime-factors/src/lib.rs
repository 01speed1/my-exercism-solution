pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {n}")
    if n == 1 {
        return vec![];
    }
    if n == 2 {
        return vec![2];
    }
    let mut current_factors = vec![];

    let mut factorized_number = n;
    let mut factor_increment = 2;
    let mut was_factorized = false;

    while factorized_number != 1 {
        if factorized_number % factor_increment == 0 {
            factorized_number = factorized_number / factor_increment;
            was_factorized = true;
        }
        if !was_factorized {
            factor_increment = factor_increment + 1
        } else {
            current_factors.push(factor_increment);
            was_factorized = false
        }
        println!("{} {:?}", factor_increment, current_factors);
    }

    current_factors
}
