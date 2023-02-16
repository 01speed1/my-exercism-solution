use hand::{two_pairs, Card, Hand, PokerHand};
use std::vec;

mod hand;

fn get_strutted_hands<'a>(hands: &[&'a str]) -> Vec<Hand<'a>> {
    hands.iter().map(|hand| Hand::new(*hand)).collect()
}

fn get_highest_poker(hands: &Vec<Hand>) -> PokerHand {
    hands
        .iter()
        .map(|hand| hand.porker_hand.clone())
        .max()
        .or(Some(PokerHand::HighCard))
        .unwrap()
}

fn get_hands_with_highest_points<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
    let max_point = hands
        .iter()
        .map(|hand| hand.porker_hand_points)
        .max()
        .unwrap();
    hands
        .iter()
        .filter(|hand| hand.porker_hand_points.eq(&max_point))
        .map(|hand| hand.clone())
        .collect::<Vec<Hand>>()
}

fn get_highest_hands_by_poker_hand<'a>(
    hands: Vec<Hand<'a>>,
    highest_poker_hand: &PokerHand,
) -> Vec<Hand<'a>> {
    hands
        .iter()
        .filter(|hand| hand.porker_hand.eq(highest_poker_hand))
        .map(|hand| hand.clone())
        .collect()
}

fn find_winners_by_highest_card_hand<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
    let mut winner_hands: Vec<Hand<'a>> = vec![];

    for index_card in (0..=4).rev().collect::<Vec<usize>>() {
        let mut all_hand = hands.clone();
        let mut highest_cards_list: Vec<Card> = vec![];

        if winner_hands.is_empty() {
            winner_hands.append(&mut all_hand);
        }

        for hand in &winner_hands {
            highest_cards_list.push(hand.sorted_cards[index_card]);
        }

        let highest_value_in_cards = highest_cards_list.iter().map(|x| x.value).max().unwrap();

        winner_hands = winner_hands
            .iter()
            .filter(|hand| hand.sorted_cards[index_card].value == highest_value_in_cards)
            .map(|hand| hand.clone())
            .collect::<Vec<Hand>>();
    }

    winner_hands
}

fn get_string_hands<'a>(hands: Vec<Hand<'a>>) -> Vec<&'a str> {
    hands
        .into_iter()
        .map(|hand| hand.hand)
        .collect::<Vec<&'a str>>()
}

// main function lib
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let strutted_hands = get_strutted_hands(hands);
    let highest_poker = get_highest_poker(&strutted_hands);
    let mut winner_hands = get_highest_hands_by_poker_hand(strutted_hands, &highest_poker);

    winner_hands = match &highest_poker {
        PokerHand::HighCard => find_winners_by_highest_card_hand(winner_hands),
        PokerHand::TwoPairs | PokerHand::ThreeOfAKind => {
            two_pairs::hands_with_highest_card(winner_hands)
        }
        _ => get_hands_with_highest_points(winner_hands),
    };

    get_string_hands(winner_hands)
}
