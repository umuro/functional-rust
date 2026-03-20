#![allow(clippy::all)]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourKind,
    StraightFlush,
}

impl HandType {
    pub fn name(self) -> &'static str {
        match self {
            HandType::StraightFlush => "Straight Flush",
            HandType::FourKind => "Four of a Kind",
            HandType::FullHouse => "Full House",
            HandType::Flush => "Flush",
            HandType::Straight => "Straight",
            HandType::ThreeKind => "Three of a Kind",
            HandType::TwoPair => "Two Pair",
            HandType::Pair => "Pair",
            HandType::HighCard => "High Card",
        }
    }
}

// Solution 1: Idiomatic Rust — HashMap for counting, iterator for classification
pub fn classify(ranks: &[u8], is_flush: bool) -> HandType {
    let counts: Vec<usize> = {
        let mut map = HashMap::new();
        for &r in ranks {
            *map.entry(r).or_insert(0usize) += 1;
        }
        let mut v: Vec<usize> = map.into_values().collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v
    };

    let mut sorted = ranks.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    let is_straight = sorted.len() == 5
        && counts.iter().all(|&c| c == 1)
        && (sorted[0] as i32 - sorted[4] as i32) == 4;

    match (is_flush, is_straight, counts.as_slice()) {
        (true, true, _) => HandType::StraightFlush,
        (_, _, [4, ..]) => HandType::FourKind,
        (_, _, [3, 2]) => HandType::FullHouse,
        (true, _, _) => HandType::Flush,
        (_, true, _) => HandType::Straight,
        (_, _, [3, ..]) => HandType::ThreeKind,
        (_, _, [2, 2, ..]) => HandType::TwoPair,
        (_, _, [2, ..]) => HandType::Pair,
        _ => HandType::HighCard,
    }
}

// Solution 2: Functional/recursive — mirrors OCaml's explicit recursion style
fn count_occurrences(rank: u8, ranks: &[u8]) -> usize {
    ranks.iter().filter(|&&r| r == rank).count()
}

fn unique_sorted(ranks: &[u8]) -> Vec<u8> {
    let mut seen: Vec<u8> = Vec::new();
    for &r in ranks {
        if !seen.contains(&r) {
            seen.push(r);
        }
    }
    seen.sort_unstable();
    seen
}

pub fn classify_functional(ranks: &[u8], is_flush: bool) -> HandType {
    let mut sorted = ranks.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    let uniq = unique_sorted(&sorted);

    let mut counts: Vec<usize> = uniq
        .iter()
        .map(|&r| count_occurrences(r, &sorted))
        .collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    let is_straight =
        sorted.len() == 5 && uniq.len() == 5 && (sorted[0] as i32 - sorted[4] as i32) == 4;

    match (is_flush, is_straight, counts.as_slice()) {
        (true, true, _) => HandType::StraightFlush,
        (_, _, [4, ..]) => HandType::FourKind,
        (_, _, [3, 2]) => HandType::FullHouse,
        (true, _, _) => HandType::Flush,
        (_, true, _) => HandType::Straight,
        (_, _, [3, ..]) => HandType::ThreeKind,
        (_, _, [2, 2, ..]) => HandType::TwoPair,
        (_, _, [2, ..]) => HandType::Pair,
        _ => HandType::HighCard,
    }
}

/* Output:
   Idiomatic  classify:
     Royal straight flush      → Straight Flush
     Low straight flush        → Straight Flush
     Four of a kind            → Four of a Kind
     Full house                → Full House
     Flush                     → Flush
     Straight                  → Straight
     Three of a kind           → Three of a Kind
     Two pair                  → Two Pair
     Pair                      → Pair
     High card                 → High Card

   Functional classify:
     Royal straight flush      → Straight Flush
     Low straight flush        → Straight Flush
     Four of a kind            → Four of a Kind
     Full house                → Full House
     Flush                     → Flush
     Straight                  → Straight
     Three of a kind           → Three of a Kind
     Two pair                  → Two Pair
     Pair                      → Pair
     High card                 → High Card
*/
