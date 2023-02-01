fn check_value_in_card(card_code: &str) -> u32 {
    let card_rank = card_code.chars().nth(0).unwrap();
    match card_rank.to_string().as_str() {
        "2" => 1,
        "3" => 2,
        "4" => 3,
        "5" => 4,
        "6" => 5,
        "7" => 6,
        "8" => 7,
        "9" => 8,
        "T" => 9,
        "J" => 10,
        "Q" => 11,
        "K" => 12,
        "A" => 13,
        _ => 0,
    }
}

fn convert_in_hand_values(hand: Vec<&str>) -> Vec<u32> {
    hand.iter().map(|card| check_value_in_card(card)).collect()
}

fn sort_by_high_card(mut hand: Vec<u32>) -> Vec<u32> {
    hand.sort();
    hand
}

fn main() {
    let ve = vec!["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"];
    //ve.sort_by(|a, b| a.split_whitespace().map(|card| check_value_in_card(card)))
    let mut a = convert_in_hand_values(ve);
    a = sort_by_high_card(a);
    println!("{:?}", a);
}
