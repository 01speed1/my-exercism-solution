use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum WinningHands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Ord for WinningHands {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use WinningHands::*;
        match (self, other) {
            (RoyalFlush, _) => std::cmp::Ordering::Greater,
            (_, RoyalFlush) => std::cmp::Ordering::Less,
            (StraightFlush, _) => std::cmp::Ordering::Greater,
            (_, StraightFlush) => std::cmp::Ordering::Less,
            (FourOfAKind, _) => std::cmp::Ordering::Greater,
            (_, FourOfAKind) => std::cmp::Ordering::Less,
            (FullHouse, _) => std::cmp::Ordering::Greater,
            (_, FullHouse) => std::cmp::Ordering::Less,
            (Flush, _) => std::cmp::Ordering::Greater,
            (_, Flush) => std::cmp::Ordering::Less,
            (Straight, _) => std::cmp::Ordering::Greater,
            (_, Straight) => std::cmp::Ordering::Less,
            (ThreeOfAKind, _) => std::cmp::Ordering::Greater,
            (_, ThreeOfAKind) => std::cmp::Ordering::Less,
            (TwoPairs, _) => std::cmp::Ordering::Greater,
            (_, TwoPairs) => std::cmp::Ordering::Less,
            (Pair, _) => std::cmp::Ordering::Greater,
            (_, Pair) => std::cmp::Ordering::Less,
            (HighCard, _) => std::cmp::Ordering::Greater,
            (_, HighCard) => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for WinningHands {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Hand<'a> {
    cards: &'a str,
    high_rank: Option,
}

impl Hand<'_> {
    fn new(cards: &str) -> Hand {
        Hand { cards }
    }

    fn high_card(&self) -> Option<HighCard> {
      for card in self.cards.split(" ") {
        let rank = card.chars().nth(0).unwrap()
      }
    }

    fn 
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {}
