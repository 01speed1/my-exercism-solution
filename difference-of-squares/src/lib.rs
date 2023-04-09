pub fn square_of_sum(n: u32) -> u32 {
    // unimplemented!("square of sum of 1...{n}")
    let sum: u32 = (1..=n).into_iter().sum();
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{n}")
    let value = (1..=n).into_iter().map(|number| number * number).sum();
    value
}

pub fn difference(n: u32) -> u32 {
    // unimplemented!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
