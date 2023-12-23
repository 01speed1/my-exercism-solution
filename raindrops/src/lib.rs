use std::vec;

pub fn get_note(n: u32) -> String {
    match n {
        3 => "Pling".to_string(),
        5 => "Plang".to_string(),
        7 => "Plong".to_string(),
        _ => n.to_string(),
    }
}

pub fn raindrops(n: u32) -> String {
    let factors = [3, 5, 7];
    let mut notes = vec![];

    for factor in factors.iter() {
        let note = get_note(*factor);

        if n % factor == 0 {
            notes.push(note);
        }
    }

    if notes.is_empty() {
        return n.to_string();
    }

    notes.join("")
}
