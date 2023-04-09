pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {n}")
    if n == 1 {
        return vec![];
    }
    if n == 2 {
        return vec![2];
    }
    let mut current_factors = vec![];
    current_factors.push(2);

    let mut factorized_number = n;

    while factorized_number != 1 {
        for factor in &current_factors {
            if factorized_number % factor == 0 {
                factorized_number = factorized_number / factor
            }
        }
        let next_value_after_last_factor = current_factors.last().unwrap() + 1;

        if !current_factors.contains(&next_value_after_last_factor) {
            let last = current_factors.pop().unwrap();
            current_factors.push(last + 1)
        }
        current_factors.clone().into_iter();
    }

    current_factors
}
