pub fn square(s: u32) -> u64 {

    if (s < 1) || (s > 64)  {
      panic!("Square must be between 1 and 64"); //"Square must be between 1 and 64"
    }

    if s == 1 {
      return s as u64;
    }

    let response = 2u128.pow(s) / 2;

    print!("{}", response);

    response as u64
}

pub fn total() -> u64 {
    u64::MAX
}
