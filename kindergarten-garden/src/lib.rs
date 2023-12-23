fn get_plant_name(letter: char) -> String {
    match letter {
        'V' => "violets".to_owned(),
        'R' => "radishes".to_owned(),
        'C' => "clover".to_owned(),
        'G' => "grass".to_owned(),
        _ => panic!("Invalid plant letter"),
    }
}

fn get_position_by_student_name(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 2,
        "Charlie" => 4,
        "David" => 6,
        "Eve" => 8,
        "Fred" => 10,
        "Ginny" => 12,
        "Harriet" => 14,
        "Ileana" => 16,
        "Joseph" => 18,
        "Kincaid" => 20,
        "Larry" => 22,
        _ => panic!("Invalid student name"),
    }
}

fn split_diagram(diagram: &str) -> (String, String) {
    let spliced = diagram.split("\n").collect::<Vec<_>>();
    (spliced[0].to_owned(), spliced[1].to_owned())
}

fn get_letters_by_position(row: &str, position: usize) -> (char, char) {
    let position_1 = row.chars().nth(position).unwrap();
    let position_2 = row.chars().nth(position + 1).unwrap();

    (position_1, position_2)
}

pub fn plants(_diagram: &str, _student: &str) -> Vec<String> {
    let mut result = vec![];

    let (row1, row2) = split_diagram(_diagram);
    let position = get_position_by_student_name(_student);
    let (letter1_row1, letter2_row1) = get_letters_by_position(&row1, position);
    let (letter1_row2, letter2_row2) = get_letters_by_position(&row2, position);

    let letters = format!(
        "{}{}{}{}",
        letter1_row1, letter2_row1, letter1_row2, letter2_row2
    );

    for letter in letters.chars() {
        let found_letter = get_plant_name(letter);
        result.push(found_letter.clone())
    }

    result
}
