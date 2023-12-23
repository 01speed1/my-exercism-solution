pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digits_list = vec![];
    let mut index: usize = 0;

    while index + len <= digits.len() {
        digits_list.push(digits[index..index + len].to_string());
        index += 1;
    }

    digits_list
}
