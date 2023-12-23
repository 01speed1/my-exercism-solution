pub fn check_around_by_position(current_row: &str, previous_row: &str, next_row: &str) -> u8 {
    let mut count = 0;

    if previous_row.len() > 0 {
        count += previous_row.chars().filter(|&c| c == '*').count() as u8;
    }

    count += current_row.chars().filter(|&c| c == '*').count() as u8;

    if next_row.len() > 0 {
        count += next_row.chars().filter(|&c| c == '*').count() as u8;
    }

    count
}

pub fn check_around(row: &str, index: usize) -> String {
    let limit = row.len() as usize;
    let mut result = "".to_string();

    if limit == 1 {
        return result;
    }

    if index == limit - 1 {
        result = row[index - 1..index].to_owned();
        return result;
    }

    if index == 0 {
        result = row[1..2].to_owned();
        return result;
    } else {
        let left = row[index - 1..index].to_owned();
        let right = row[index + 1..index + 2].to_owned();

        return [left, right].concat();
    }
}

pub fn splice_by_index(row: &str, index: usize) -> &str {
    let limit = row.len() as usize;

    if limit == 1 {
        return row;
    }

    if index == limit - 1 {
        return &row[index - 1..=index];
    }

    if index == 0 {
        return &row[index..index + 2];
    } else {
        return &row[index - 1..index + 2];
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for (row_index, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();

        for (field_index, field) in row.as_bytes().iter().enumerate() {
            let previous_row = if row_index == 0 {
                ""
            } else {
                splice_by_index(minefield[row_index - 1], field_index)
            };

            let next_row = if row_index == minefield.len() - 1 {
                ""
            } else {
                splice_by_index(minefield[row_index + 1], field_index)
            };

            if field == &b'*' {
                new_row = new_row + "*";
                continue;
            }

            let count =
                check_around_by_position(&check_around(row, field_index), &previous_row, &next_row);

            if count == 0 {
                new_row = new_row + " ";
                continue;
            }

            new_row = new_row + &count.to_string();
        }

        result.push(new_row);
    }
    result
}
