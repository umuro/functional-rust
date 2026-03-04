/// Yacht dice scoring categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Yacht,
    Choice,
}

/// Count how many dice show the value `n`.
fn count(dice: &[u8], n: u8) -> u8 {
    dice.iter().filter(|&&d| d == n).count() as u8
}

/// Score idiomatic Rust style — exhaustive match, iterator-based helpers.
pub fn score(dice: &[u8; 5], category: Category) -> u32 {
    match category {
        Category::Ones => u32::from(count(dice, 1)),
        Category::Twos => 2 * u32::from(count(dice, 2)),
        Category::Threes => 3 * u32::from(count(dice, 3)),
        Category::Fours => 4 * u32::from(count(dice, 4)),
        Category::Fives => 5 * u32::from(count(dice, 5)),
        Category::Sixes => 6 * u32::from(count(dice, 6)),
        Category::Choice => dice.iter().map(|&d| u32::from(d)).sum(),
        Category::Yacht => {
            if dice.iter().all(|&d| d == dice[0]) {
                50
            } else {
                0
            }
        }
        Category::FullHouse => {
            // Counts per face value; a full house is exactly two distinct faces
            // with counts 2 and 3.
            let mut counts = [0u8; 7];
            for &d in dice {
                counts[d as usize] += 1;
            }
            let freqs: Vec<u8> = counts.iter().copied().filter(|&c| c > 0).collect();
            let mut sorted_freqs = freqs.clone();
            sorted_freqs.sort_unstable();
            if sorted_freqs == [2, 3] {
                dice.iter().map(|&d| u32::from(d)).sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            // Find a face value that appears at least 4 times
            (1u8..=6)
                .find(|&n| count(dice, n) >= 4)
                .map(|n| 4 * u32::from(n))
                .unwrap_or(0)
        }
        Category::LittleStraight => {
            let mut sorted = *dice;
            sorted.sort_unstable();
            if sorted == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            let mut sorted = *dice;
            sorted.sort_unstable();
            if sorted == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
    }
}

/// Functional/recursive style — mirrors the OCaml approach more closely.
/// Recursively scans face values 1..=6 to find one appearing >= 4 times.
pub fn score_four_of_a_kind_recursive(dice: &[u8; 5], face: u8) -> u32 {
    if face > 6 {
        return 0;
    }
    if count(dice, face) >= 4 {
        4 * u32::from(face)
    } else {
        score_four_of_a_kind_recursive(dice, face + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Yacht ---
    #[test]
    fn test_yacht_all_same() {
        assert_eq!(score(&[5, 5, 5, 5, 5], Category::Yacht), 50);
    }

    #[test]
    fn test_yacht_not_all_same() {
        assert_eq!(score(&[1, 2, 3, 4, 5], Category::Yacht), 0);
    }

    // --- Number categories ---
    #[test]
    fn test_ones() {
        assert_eq!(score(&[1, 1, 2, 3, 4], Category::Ones), 2);
    }

    #[test]
    fn test_sixes() {
        assert_eq!(score(&[6, 6, 6, 6, 6], Category::Sixes), 30);
    }

    #[test]
    fn test_twos_none() {
        assert_eq!(score(&[1, 3, 4, 5, 6], Category::Twos), 0);
    }

    // --- Choice ---
    #[test]
    fn test_choice_sum() {
        assert_eq!(score(&[1, 2, 3, 4, 5], Category::Choice), 15);
    }

    #[test]
    fn test_choice_all_sixes() {
        assert_eq!(score(&[6, 6, 6, 6, 6], Category::Choice), 30);
    }

    // --- FullHouse ---
    #[test]
    fn test_full_house_two_then_three() {
        assert_eq!(score(&[2, 2, 3, 3, 3], Category::FullHouse), 13);
    }

    #[test]
    fn test_full_house_three_then_two() {
        assert_eq!(score(&[3, 3, 3, 2, 2], Category::FullHouse), 13);
    }

    #[test]
    fn test_full_house_five_of_a_kind_not_full_house() {
        assert_eq!(score(&[5, 5, 5, 5, 5], Category::FullHouse), 0);
    }

    #[test]
    fn test_full_house_all_different() {
        assert_eq!(score(&[1, 2, 3, 4, 5], Category::FullHouse), 0);
    }

    // --- FourOfAKind ---
    #[test]
    fn test_four_of_a_kind_basic() {
        assert_eq!(score(&[3, 3, 3, 3, 1], Category::FourOfAKind), 12);
    }

    #[test]
    fn test_four_of_a_kind_five_of_a_kind() {
        // five-of-a-kind satisfies four-of-a-kind
        assert_eq!(score(&[4, 4, 4, 4, 4], Category::FourOfAKind), 16);
    }

    #[test]
    fn test_four_of_a_kind_none() {
        assert_eq!(score(&[1, 2, 3, 4, 5], Category::FourOfAKind), 0);
    }

    // --- Straights ---
    #[test]
    fn test_little_straight() {
        assert_eq!(score(&[1, 2, 3, 4, 5], Category::LittleStraight), 30);
    }

    #[test]
    fn test_little_straight_unordered() {
        assert_eq!(score(&[3, 1, 2, 5, 4], Category::LittleStraight), 30);
    }

    #[test]
    fn test_big_straight() {
        assert_eq!(score(&[2, 3, 4, 5, 6], Category::BigStraight), 30);
    }

    #[test]
    fn test_little_straight_fails_for_big() {
        assert_eq!(score(&[1, 2, 3, 4, 5], Category::BigStraight), 0);
    }

    // --- Recursive helper ---
    #[test]
    fn test_recursive_four_of_a_kind() {
        assert_eq!(score_four_of_a_kind_recursive(&[2, 2, 2, 2, 5], 1), 8);
        assert_eq!(score_four_of_a_kind_recursive(&[1, 2, 3, 4, 5], 1), 0);
    }
}
