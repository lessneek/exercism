use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};

use crate::Kind::*;
use crate::Suit::*;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    hands
        .iter()
        .map(|&s| Hand::new(s))
        .collect::<BinaryHeap<Hand>>()
        .into_iter()
        .scan(Option::<u32>::None, |high_hand_rank, hand| {
            if high_hand_rank.is_none() {
                *high_hand_rank = Some(hand.rank);
            }
            match *high_hand_rank {
                Some(x) if x == hand.rank => Some(hand.code),
                _ => None,
            }
        })
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
#[repr(u8)]
enum Suit {
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
    Spades = 4,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
#[repr(u8)]
enum Kind {
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Add for Kind {
    type Output = u8;

    fn add(self, rhs: Self) -> Self::Output {
        self as u8 + rhs as u8
    }
}

impl Sub for Kind {
    type Output = u8;

    fn sub(self, rhs: Self) -> Self::Output {
        self as u8 - rhs as u8
    }
}

#[derive(Eq, PartialEq)]
struct Card {
    suit: Suit,
    kind: Kind,
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.kind, self.suit)
    }
}

#[derive(Eq, Debug)]
struct Hand<'a> {
    code: &'a str,
    rank: u32,
}

impl<'a> Hand<'a> {
    pub fn new(code: &'a str) -> Hand {
        Hand {
            code,
            rank: poker_hand_code_to_rank(code),
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[test]
fn test_code_to_card() {
    assert_eq!(
        code_to_card("2H").unwrap(),
        Card {
            kind: _2,
            suit: Hearts
        }
    );
    assert_eq!(
        code_to_card("JS").unwrap(),
        Card {
            kind: Jack,
            suit: Spades
        }
    );
    assert_eq!(code_to_card("AB"), None);
}

fn code_to_card(code: &str) -> Option<Card> {
    if code.len() != 2 && code.len() != 3 {
        return None;
    }
    let mut chars = code.chars();
    let kind = match chars.next()? {
        '2' => _2,
        '3' => _3,
        '4' => _4,
        '5' => _5,
        '6' => _6,
        '7' => _7,
        '8' => _8,
        '9' => _9,
        '1' => {
            if chars.next()? == '0' {
                _10
            } else {
                return None;
            }
        }
        'J' => Jack,
        'Q' => Queen,
        'K' => King,
        'A' => Ace,
        _ => return None,
    };
    let suit = match chars.next()? {
        'H' => Hearts,
        'D' => Diamonds,
        'C' => Clubs,
        'S' => Spades,
        _ => return None,
    };
    Some(Card { suit, kind })
}

fn rank_cards_with_kind(cards: &[Card], kind: Kind) -> u32 {
    cards.iter().fold(0, |acc, card| {
        acc + (card.kind as u32) * if card.kind == kind { 14 } else { 1 }
    })
}

fn poker_hand_code_to_rank(code: &str) -> u32 {
    let mut cards: Vec<Card> = code
        .split_ascii_whitespace()
        .filter_map(code_to_card)
        .collect();
    if cards.len() != 5 {
        return 0;
    }
    cards.sort_by(|a, b| a.kind.cmp(&b.kind));

    let kinds = cards.iter().map(|x| x.kind).collect::<Vec<_>>();

    let mut kinds_counts = HashMap::<Kind, u8>::new();
    for card in cards.iter() {
        *kinds_counts.entry(card.kind).or_default() += 1;
    }

    let (straight, ace_low_straight) = match cards.iter().map(|x| x.kind).collect::<Vec<_>>()[..] {
        [_2, _3, _4, _5, Ace] => (true, true),
        _ => (
            cards
                .iter()
                .zip(cards.iter().skip(1))
                .fold(true, |acc, (a, b)| acc && (b.kind - a.kind == 1)),
            false,
        ),
    };

    let flush = cards[1..4].iter().all(|card| card.suit == cards[0].suit);

    let rank_base: Vec<u32> = (0u32..=9u32).map(|x| x * 10_000_000).collect();

    // [9] Straight flush.
    if straight && flush {
        return rank_base[9] + (kinds[4] as u32)
            - if ace_low_straight { Ace as u32 - 1 } else { 0 };
    }

    // [8] Four of a kind.
    if let Some((&kind, _)) = kinds_counts.iter().find(|(_, &count)| count == 4) {
        return rank_base[8] + rank_cards_with_kind(&cards, kind);
    }

    // [7] Full house.
    if let (Some((&kind_3, _)), Some(_)) = (
        kinds_counts.iter().find(|(_, &count)| count == 3),
        kinds_counts.iter().find(|(_, &count)| count == 2),
    ) {
        return rank_base[7] + rank_cards_with_kind(&cards, kind_3);
    }

    // [6] Flush.
    if flush {
        return rank_base[6] + kinds.iter().map(|x| *x as u32).sum::<u32>();
    }

    // [5] Straight.
    if straight {
        return rank_base[5] + kinds.iter().map(|x| *x as u32).sum::<u32>()
            - if ace_low_straight { Ace as u32 - 1 } else { 0 };
    }

    //[4] Three of a kind.
    if let Some((&kind, _)) = kinds_counts.iter().find(|(_, &count)| count == 3) {
        return rank_base[4] + rank_cards_with_kind(&cards, kind);
    }

    // [3] Two pair.
    let mut pairs_kinds = kinds_counts
        .iter()
        .filter(|(_, &count)| count == 2)
        .map(|(kind, _)| *kind)
        .collect::<Vec<_>>();
    if pairs_kinds.len() == 2 {
        pairs_kinds.sort_unstable();
        let (higher_pair_kind, lower_pair_kind) = (pairs_kinds[1], pairs_kinds[0]);
        return rank_base[3]
            + kinds.iter().fold(0, |acc, &kind| {
                acc + (kind as u32)
                    * match kind {
                        x if x == higher_pair_kind => 2744,
                        x if x == lower_pair_kind => 196,
                        _ => 1,
                    }
            });
    }

    // [2] One pair.
    if pairs_kinds.len() == 1 {
        return rank_base[2] + rank_cards_with_kind(&cards, pairs_kinds[0]);
    }

    // [1] High card.
    return rank_base[1]
        + cards.iter().enumerate().fold(0, |acc, (i, card)| {
            acc + (card.kind as u32 * 14u32.pow(i as u32))
        });
}
