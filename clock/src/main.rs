
fn wrapper(num: i32, wrap: i32) -> i32 {
  (num + ((-num / wrap) * wrap) + wrap) % wrap
}

pub fn main() {
  println!("{}", wrapper(-20, 60))
}
