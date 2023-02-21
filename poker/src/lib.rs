//use hand::{royal_flush, two_pairs, Card, Hand, PokerHand};
use std::vec;

//mod hand;

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Card {
    pub value: u8,
    pub suit: char,
    pub position: u8,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum PokerHand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, Clone)]
pub struct Hand<'a> {
    pub hand: &'a str,
    cards: Vec<Card>,
    pub porker_hand: PokerHand,
    pub highest_card: Card,
    pub sorted_cards: Vec<Card>,
    pub poker_hand_points: u16,
    pub mapped_cards: HashMap<u8, u8>,
}

impl Hand<'_> {
    pub fn new(cards: &str) -> Hand {
        let values_chars: Vec<char> = vec![
            '0', '2', '3', '4', '5', '6', '7', '8', '9', '1', 'J', 'Q', 'K', 'A',
        ];

        let strutted_cards: Vec<Card> = cards
            .split(" ")
            .map(|card| {
                let mut chars = card.chars();
                let pre_value = chars.next().unwrap();
                //let mut value = pre_value as u8;

                let value = match pre_value {
                    '1' => {
                        let temp = values_chars
                            .iter()
                            .enumerate()
                            .filter(|(_, char)| *char == &'1')
                            .fold(0, |_, (index, _)| index as u8);
                        chars.next();
                        temp
                    }
                    _ => values_chars
                        .iter()
                        .enumerate()
                        .filter(|(_, char)| *char == &pre_value)
                        .fold(0, |_, (index, _)| index as u8),
                };

                let suit = chars.next().unwrap();

                Card {
                    value,
                    suit: suit,
                    position: 0,
                }
            })
            .collect();

        let mut strutted_cloned = strutted_cards.clone();
        strutted_cloned.sort_by(|a, b| a.value.cmp(&b.value));

        let highest_card = strutted_cloned.last().unwrap().clone();
        let poker_hand_points = strutted_cloned
            .iter()
            .map(|card| card.value)
            .fold(0, |accumulator, value| accumulator + value)
            as u16;

        println!("{}", poker_hand_points);

        let mapped_cards = strutted_cloned.clone().iter().fold(
            HashMap::new(),
            |mut mapped, card| -> HashMap<u8, u8> {
                mapped
                    .entry(card.value)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);

                mapped.into()
            },
        );

        let mut muttable_hand = Hand {
            highest_card,
            hand: cards,
            cards: strutted_cards,
            porker_hand: PokerHand::HighCard,
            sorted_cards: strutted_cloned,
            poker_hand_points,
            mapped_cards,
        };

        muttable_hand.one_pair();
        muttable_hand.two_pairs();
        muttable_hand.three_of_a_kind();
        muttable_hand.straight();
        muttable_hand.flush();
        muttable_hand.full_house();
        muttable_hand.four_of_a_kind();
        muttable_hand.straight_flush();
        muttable_hand.royal_flush();

        let hand = muttable_hand;

        hand
    }
}

impl Hand<'_> {
    fn one_pair(&mut self) {
        let values = &self.mapped_cards;

        for (key, value) in values.iter() {
            if value == &2 {
                self.porker_hand = PokerHand::OnePair;
            }
        }
    }

    fn two_pairs(&mut self) {
        let values = &self.mapped_cards;

        let mut pairs_count = 0;

        for (_, value) in values.iter() {
            if value == &2 {
                pairs_count += 1;
            }
        }

        if pairs_count == 2 {
            self.porker_hand = PokerHand::TwoPairs;
        }
    }

    fn three_of_a_kind(&mut self) {
        let values = &self.mapped_cards;

        for (key, value) in values.iter() {
            if value == &3 {
                self.porker_hand = PokerHand::ThreeOfAKind;
                self.poker_hand_points += *key as u16 * 2u16;
            }
        }
    }

    fn straight(&mut self) {
        /* let mut hand_copy = self.cards.to_vec();

        let hand_has_a_five = hand_copy.iter().find(|card| card.value == 4).is_some();
        let hand_has_an_ace = hand_copy.iter().find(|card| card.value == 13).is_some();

        let lowest_card;
        let lowest_card_index;

        if hand_has_a_five && hand_has_an_ace {
            hand_copy = hand_copy.iter().fold(Vec::new(), |mut accumulator, card| {
                if card.value == 13 {
                    let Card {
                        position,
                        suit,
                        value: _,
                    } = *card;
                    let new_card = Card {
                        position,
                        suit,
                        value: 0,
                    };
                    accumulator.push(new_card);
                    return accumulator;
                } else {
                    accumulator.push(*card);
                    return accumulator;
                }
            });

            lowest_card = hand_copy
                .iter()
                .find(|card| {
                    if hand_has_a_five && hand_has_an_ace {
                        card.value == 0
                    } else {
                        card.value == 4
                    }
                })
                .unwrap()
                .clone();
        } else {
            lowest_card = hand_copy
                .iter()
                .min_by(|previous_card, current_card| previous_card.value.cmp(&current_card.value))
                .unwrap()
                .clone();
        }

        lowest_card_index = hand_copy
            .iter()
            .position(|x| x.value == lowest_card.value)
            .unwrap() as usize;

        hand_copy.remove(lowest_card_index);

        hand_copy
            .sort_by(|previous_card, current_card| previous_card.value.cmp(&current_card.value));

        let mut values: Vec<Card> = vec![];
        values.push(lowest_card);

        for (index, card) in hand_copy.iter().enumerate() {
            let index = index as u8 + 1;
            if lowest_card.value == card.value {
                break;
            }

            if (lowest_card.value + index as u8) != card.value {
                break;
            }

            if (lowest_card.value + index as u8) == card.value {
                values.push(card.clone());
            }
        } */
        if straight::is_straight(&self) {
            self.porker_hand = PokerHand::Straight;
            self.poker_hand_points = self
                .cards
                .clone()
                .iter()
                .fold(0, |accumulator, card| accumulator + card.value as u16);
        }
    }

    fn flush(&mut self) {
        let mut flush_map: HashMap<char, u8> = HashMap::new();
        flush_map.insert('S', 0);
        flush_map.insert('H', 0);
        flush_map.insert('D', 0);
        flush_map.insert('C', 0);

        for card in &self.cards {
            flush_map.entry(card.suit).and_modify(|x| *x += 1);
        }

        for value in flush_map.values() {
            if *value == 5 {
                self.porker_hand = PokerHand::Flush;
                self.poker_hand_points = self
                    .cards
                    .clone()
                    .iter()
                    .fold(0, |accumulator, card| accumulator + card.value as u16);
            }
        }
    }

    fn full_house(&mut self) {
        let values = &self.mapped_cards;

        let mut third_of_a_kind_points = 0;
        let mut one_pair_points = 0;

        let some_third_of_a_kind = values.values().any(|x| *x == 3);
        if some_third_of_a_kind {
            let third_of_a_kind_value = values
                .iter()
                .filter(|kv| *kv.1 == 3)
                .fold(0 as u8, |_, kv| *kv.0);

            third_of_a_kind_points = *values.get(&third_of_a_kind_value).unwrap();
        }

        let some_one_pair = values.values().any(|x| *x == 2);
        if some_one_pair {
            let one_pair_value = values
                .iter()
                .filter(|kv| *kv.1 == 2)
                .fold(0 as u8, |_, kv| *kv.0);

            one_pair_points = *values.get(&one_pair_value).unwrap();
        }

        if some_third_of_a_kind && some_one_pair {
            self.porker_hand = PokerHand::FullHouse;
            self.poker_hand_points = self.cards.iter().fold(0 as u16, |accumulator, card| {
                accumulator + card.value as u16
            });
        }
    }

    fn four_of_a_kind(&mut self) {
        let values = &self.mapped_cards;

        let mut four_of_a_kind_points = 0;
        let some_four_of_kind = values.values().any(|x| *x == 4);

        if some_four_of_kind {
            let four_of_a_kind_value = values
                .iter()
                .filter(|kv| *kv.1 == 4)
                .fold(0 as u8, |_, kv| *kv.0);

            four_of_a_kind_points = *values.get(&four_of_a_kind_value).unwrap();
        }

        if some_four_of_kind {
            self.porker_hand = PokerHand::FourOfAKind;
            self.poker_hand_points = self.cards.iter().fold(0 as u16, |accumulator, card| {
                accumulator + card.value as u16
            });
        }
    }

    fn straight_flush(&mut self) {
        let first_suit = self.cards[0].suit;

        let are_all_suits_equal = self.cards.iter().all(|card| card.suit.eq(&first_suit));

        let is_straight = straight::is_straight(&self);

        if are_all_suits_equal && is_straight {
            self.porker_hand = PokerHand::StraightFlush;

            self.poker_hand_points = self.cards.iter().fold(0 as u16, |accumulator, card| {
                accumulator + card.value as u16
            });
        }
    }

    fn royal_flush(&mut self) {
        self.straight_flush();

        let is_straight_flush = self.porker_hand.eq(&PokerHand::StraightFlush);

        if !is_straight_flush {
            return;
        }

        self.straight();

        let is_straight = self.porker_hand.eq(&PokerHand::Straight);

        if is_straight {
            self.porker_hand = PokerHand::RoyalFlush;

            self.poker_hand_points = self.cards.iter().fold(0 as u16, |accumulator, card| {
                accumulator + card.value as u16
            });
        }
    }
}

// mods
pub mod two_pairs {
    use std::collections::HashMap;

    use super::Hand;

    pub fn hands_with_highest_pair<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
        let mut highest_card_value_for_a_pair = 0;
        let mut filtered_hands: Vec<Hand<'a>> = vec![];

        for hand in &hands {
            let mut accepted = false;
            for (key, value) in &hand.mapped_cards {
                if value == &2 && *key >= highest_card_value_for_a_pair {
                    highest_card_value_for_a_pair = key.clone();
                    accepted = true;
                }
            }
            if accepted {
                filtered_hands.push(hand.clone());
                accepted = false;
            }
        }

        filtered_hands
    }

    pub fn hands_with_highest_card<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
        let mut checking_hands: Vec<Hand<'a>> = vec![];

        let mut highest_card_value = 0 as u8;

        let highest_score = hands
            .iter()
            .map(|hand| hand.poker_hand_points)
            .max()
            .unwrap();

        checking_hands = hands
            .iter()
            .filter(|hand| hand.poker_hand_points.ge(&highest_score))
            .map(|hand| {
                let Hand {
                    cards,
                    hand,
                    highest_card,
                    mapped_cards,
                    poker_hand_points,
                    porker_hand,
                    sorted_cards,
                } = hand;
                Hand {
                    hand,
                    cards: cards.to_vec(),
                    porker_hand: *porker_hand,
                    highest_card: *highest_card,
                    sorted_cards: sorted_cards.to_vec(),
                    poker_hand_points: *poker_hand_points,
                    mapped_cards: mapped_cards.clone(),
                }
            })
            .collect();

        let clone_checking_hands = &checking_hands;

        for hand in clone_checking_hands.clone() {
            for (card, value) in hand.mapped_cards.iter() {
                let hand = hand.clone();
                let card: u8 = *card;
                let value: u8 = *value;
                if card > highest_card_value && value == 1 {
                    highest_card_value = card;
                    checking_hands.clear();
                    checking_hands.push(hand);
                }
            }
        }

        checking_hands
    }
}

mod straight {
    use super::Card;
    use super::Hand;

    pub fn is_straight(hand: &Hand) -> bool {
        let mut hand_copy = hand.cards.to_vec();

        let hand_has_a_five = hand_copy.iter().find(|card| card.value == 4).is_some();
        let hand_has_an_ace = hand_copy.iter().find(|card| card.value == 13).is_some();

        let lowest_card;
        let lowest_card_index;

        if hand_has_a_five && hand_has_an_ace {
            hand_copy = hand_copy.iter().fold(Vec::new(), |mut accumulator, card| {
                if card.value == 13 {
                    let Card {
                        position,
                        suit,
                        value: _,
                    } = *card;
                    let new_card = Card {
                        position,
                        suit,
                        value: 0,
                    };
                    accumulator.push(new_card);
                    return accumulator;
                } else {
                    accumulator.push(*card);
                    return accumulator;
                }
            });

            lowest_card = hand_copy
                .iter()
                .find(|card| {
                    if hand_has_a_five && hand_has_an_ace {
                        card.value == 0
                    } else {
                        card.value == 4
                    }
                })
                .unwrap()
                .clone();
        } else {
            lowest_card = hand_copy
                .iter()
                .min_by(|previous_card, current_card| previous_card.value.cmp(&current_card.value))
                .unwrap()
                .clone();
        }

        lowest_card_index = hand_copy
            .iter()
            .position(|x| x.value == lowest_card.value)
            .unwrap() as usize;

        hand_copy.remove(lowest_card_index);

        hand_copy
            .sort_by(|previous_card, current_card| previous_card.value.cmp(&current_card.value));

        let mut values: Vec<Card> = vec![];
        values.push(lowest_card);

        for (index, card) in hand_copy.iter().enumerate() {
            let index = index as u8 + 1;
            if lowest_card.value == card.value {
                break;
            }

            if (lowest_card.value + index as u8) != card.value {
                break;
            }

            if (lowest_card.value + index as u8) == card.value {
                values.push(card.clone());
            }
        }
        if values.len() == 5 {
            return true;
        }
        false
    }
}

pub mod royal_flush {
    use super::Hand;

    pub fn filter_by_highest_poker_points<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
        let highest_points = &hands
            .iter()
            .map(|hand| {
                let hand_has_a_five = hand.cards.iter().find(|card| card.value == 4).is_some();
                let hand_has_an_ace = hand.cards.iter().find(|card| card.value == 13).is_some();

                if hand_has_a_five && hand_has_an_ace {
                    return hand.poker_hand_points - 13;
                }

                hand.poker_hand_points
            })
            .max()
            .unwrap();

        hands
            .iter()
            .filter(|hand| {
                let hand = *hand;

                let mut true_points = hand.poker_hand_points;

                let hand_has_a_five = hand.cards.iter().find(|card| card.value == 4).is_some();
                let hand_has_an_ace = hand.cards.iter().find(|card| card.value == 13).is_some();

                if hand_has_a_five && hand_has_an_ace {
                    true_points = true_points - 13;
                }

                true_points.ge(highest_points)
            })
            .map(|hand| {
                let Hand {
                    hand,
                    cards,
                    porker_hand,
                    highest_card,
                    sorted_cards,
                    poker_hand_points,
                    mapped_cards,
                } = hand;

                Hand {
                    hand,
                    cards: cards.to_vec(),
                    porker_hand: *porker_hand,
                    highest_card: *highest_card,
                    sorted_cards: sorted_cards.to_vec(),
                    poker_hand_points: *poker_hand_points,
                    mapped_cards: mapped_cards.clone(),
                }
            })
            .collect()
    }
}

fn get_strutted_hands<'a>(hands: &[&'a str]) -> Vec<Hand<'a>> {
    hands.iter().map(|hand| Hand::new(*hand)).collect()
}

fn get_highest_poker(hands: &Vec<Hand>) -> PokerHand {
    hands
        .iter()
        .map(|hand| hand.porker_hand)
        .max()
        .or(Some(PokerHand::HighCard))
        .unwrap()
}

fn get_hands_with_highest_points<'a>(hands: Vec<Hand<'a>>) -> Vec<Hand<'a>> {
    let max_point = hands
        .iter()
        .map(|hand| hand.poker_hand_points)
        .max()
        .unwrap();
    hands
        .iter()
        .filter(|hand| hand.poker_hand_points.eq(&max_point))
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
        PokerHand::TwoPairs => {
            let a = two_pairs::hands_with_highest_pair(winner_hands);
            let b = get_hands_with_highest_points(a);

            b
        }
        PokerHand::Straight | PokerHand::StraightFlush | PokerHand::RoyalFlush => {
            royal_flush::filter_by_highest_poker_points(winner_hands)
        }
        _ => get_hands_with_highest_points(winner_hands),
    };

    get_string_hands(winner_hands)
}
