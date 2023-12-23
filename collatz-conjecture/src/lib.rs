pub fn collatz(n: u64) -> Option<u64> {
    let mut counter = 0;
    let mut number = n;

    match number {
        110243094271..=u64::MAX => return None,
        0 => return None,
        _ => (),
    }

    while number != 1 {
        if number % 2 == 0 {
            number = number / 2;
            counter += 1;
            continue;
        } else {
            number = (number * 3) + 1;
            counter += 1;
        }
    }

    Some(counter)
}
