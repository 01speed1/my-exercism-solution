use std::ops::Add;

pub fn is_armstrong_number(num: u32) -> bool {
  let numbers = num.to_string();
  let numbers =numbers.chars();
  let numbers_clone_count = numbers.clone().count();

  let numbers_count = numbers_clone_count.try_into().unwrap();

  let armstrong_number = numbers
    .map(|number| number.to_digit(10).unwrap().pow(numbers_count) )
    .reduce(|a:u32, b:u32| a.add(b) )
    .unwrap();

  num == armstrong_number
}
