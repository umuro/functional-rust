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

    // Sort descending to check straight span
    let mut sorted = ranks.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    // Straight: 5 unique ranks spanning exactly 4 (highest - lowest == 4)
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

    // Count occurrences of each unique rank, then sort counts descending
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- classify (idiomatic) ---

    #[test]
    fn test_straight_flush() {
        assert_eq!(
            classify(&[10, 11, 12, 13, 14], true),
            HandType::StraightFlush
        );
        assert_eq!(classify(&[2, 3, 4, 5, 6], true), HandType::StraightFlush);
    }

    #[test]
    fn test_four_of_a_kind() {
        assert_eq!(classify(&[9, 9, 9, 9, 5], false), HandType::FourKind);
    }

    #[test]
    fn test_full_house() {
        assert_eq!(classify(&[3, 3, 3, 7, 7], false), HandType::FullHouse);
        assert_eq!(classify(&[10, 10, 10, 6, 6], false), HandType::FullHouse);
    }

    #[test]
    fn test_flush() {
        assert_eq!(classify(&[2, 5, 7, 9, 11], true), HandType::Flush);
    }

    #[test]
    fn test_straight() {
        assert_eq!(classify(&[5, 6, 7, 8, 9], false), HandType::Straight);
        assert_eq!(classify(&[10, 11, 12, 13, 14], false), HandType::Straight);
    }

    #[test]
    fn test_three_of_a_kind() {
        assert_eq!(classify(&[8, 8, 8, 3, 5], false), HandType::ThreeKind);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(classify(&[4, 4, 7, 7, 9], false), HandType::TwoPair);
    }

    #[test]
    fn test_pair() {
        assert_eq!(classify(&[2, 2, 5, 8, 11], false), HandType::Pair);
    }

    #[test]
    fn test_high_card() {
        assert_eq!(classify(&[2, 5, 9, 11, 14], false), HandType::HighCard);
    }

    // --- classify_functional agrees with classify ---

    #[test]
    fn test_functional_matches_idiomatic() {
        let hands: Vec<(&[u8], bool)> = vec![
            (&[10, 11, 12, 13, 14], true), // straight flush
            (&[9, 9, 9, 9, 5], false),     // four of a kind
            (&[3, 3, 3, 7, 7], false),     // full house
            (&[2, 5, 7, 9, 11], true),     // flush
            (&[5, 6, 7, 8, 9], false),     // straight
            (&[8, 8, 8, 3, 5], false),     // three of a kind
            (&[4, 4, 7, 7, 9], false),     // two pair
            (&[2, 2, 5, 8, 11], false),    // pair
            (&[2, 5, 9, 11, 14], false),   // high card
        ];
        for (ranks, is_flush) in hands {
            assert_eq!(
                classify(ranks, is_flush),
                classify_functional(ranks, is_flush),
                "Mismatch for ranks {:?} flush={}",
                ranks,
                is_flush
            );
        }
    }

    // --- HandType::name ---

    #[test]
    fn test_hand_names() {
        assert_eq!(HandType::StraightFlush.name(), "Straight Flush");
        assert_eq!(HandType::FourKind.name(), "Four of a Kind");
        assert_eq!(HandType::FullHouse.name(), "Full House");
        assert_eq!(HandType::Flush.name(), "Flush");
        assert_eq!(HandType::Straight.name(), "Straight");
        assert_eq!(HandType::ThreeKind.name(), "Three of a Kind");
        assert_eq!(HandType::TwoPair.name(), "Two Pair");
        assert_eq!(HandType::Pair.name(), "Pair");
        assert_eq!(HandType::HighCard.name(), "High Card");
    }

    // --- HandType ordering (StraightFlush > FourKind > ... > HighCard) ---

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::StraightFlush > HandType::FourKind);
        assert!(HandType::FourKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::Flush);
        assert!(HandType::Flush > HandType::Straight);
        assert!(HandType::Straight > HandType::ThreeKind);
        assert!(HandType::ThreeKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::Pair);
        assert!(HandType::Pair > HandType::HighCard);
    }

    // --- Edge cases ---

    #[test]
    fn test_flush_not_straight() {
        // All same suit, non-consecutive — must be Flush, not Straight
        assert_eq!(classify(&[2, 4, 6, 8, 11], true), HandType::Flush);
    }

    #[test]
    fn test_straight_not_flush() {
        // Consecutive ranks, mixed suits — must be Straight, not Flush
        assert_eq!(classify(&[7, 8, 9, 10, 11], false), HandType::Straight);
    }

    #[test]
    fn test_royal_straight_flush() {
        assert_eq!(
            classify(&[10, 11, 12, 13, 14], true),
            HandType::StraightFlush
        );
        assert_eq!(
            classify(&[10, 11, 12, 13, 14], true).name(),
            "Straight Flush"
        );
    }
}
