pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {list:?}")
    if list.len() == 0 {
        return String::from("");
    }
    let last_index = list.len() - 1;
    let mut sentences = vec![];

    for (index, _) in list.iter().enumerate() {
        if index != last_index {
            sentences.push(build_first_line(list[index], list[index + 1]))
        }
    }

    sentences.push(build_last_line(list[0]));
    sentences.join("\n")
}

fn build_first_line(word_one: &str, word_two: &str) -> String {
    format!(
        "For want of a {} the {} was lost.",
        word_one.clone(),
        word_two.clone()
    )
    .to_owned()
}

fn build_last_line(word_one: &str) -> String {
    format!("And all for the want of a {}.", word_one.clone()).to_owned()
}
