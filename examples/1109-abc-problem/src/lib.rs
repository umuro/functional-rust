/// The standard Rosetta Code ABC block set — 20 blocks, each with two letters.
const BLOCKS: &[(char, char)] = &[
    ('B', 'O'),
    ('X', 'K'),
    ('D', 'Q'),
    ('C', 'P'),
    ('N', 'A'),
    ('G', 'T'),
    ('R', 'E'),
    ('T', 'G'),
    ('Q', 'D'),
    ('F', 'S'),
    ('J', 'W'),
    ('H', 'U'),
    ('V', 'I'),
    ('A', 'N'),
    ('O', 'B'),
    ('E', 'R'),
    ('F', 'S'),
    ('L', 'Y'),
    ('P', 'C'),
    ('Z', 'M'),
];

fn block_has(block: (char, char), c: char) -> bool {
    block.0 == c || block.1 == c
}

// --- Solution 1: Idiomatic Rust ---
// Backtracking with slice patterns; `available` tracks indices of unused blocks.
// Each recursive call tries every block that contains the current letter,
// removes it, and recurses — then automatically backtracks on failure.

pub fn can_make_word(word: &str) -> bool {
    let letters: Vec<char> = word.to_uppercase().chars().collect();
    let available: Vec<usize> = (0..BLOCKS.len()).collect();
    can_spell(&letters, &available)
}

fn can_spell(letters: &[char], available: &[usize]) -> bool {
    match letters {
        [] => true,
        [c, rest @ ..] => available.iter().enumerate().any(|(pos, &idx)| {
            block_has(BLOCKS[idx], *c) && {
                // Remove block at `pos` and recurse
                let remaining: Vec<usize> = available[..pos]
                    .iter()
                    .chain(&available[pos + 1..])
                    .copied()
                    .collect();
                can_spell(rest, &remaining)
            }
        }),
    }
}

// --- Solution 2: Functional/recursive — mirrors OCaml partition style ---
// Uses `partition` to split blocks into those containing the letter and the rest,
// then tries each matching block (proper backtracking, unlike the greedy OCaml version).

pub fn can_make_word_functional(word: &str) -> bool {
    let letters: Vec<char> = word.to_uppercase().chars().collect();
    can_spell_functional(&letters, BLOCKS.to_vec())
}

fn can_spell_functional(letters: &[char], blocks: Vec<(char, char)>) -> bool {
    match letters {
        [] => true,
        [c, rest @ ..] => {
            let (matching, non_matching): (Vec<_>, Vec<_>) =
                blocks.into_iter().partition(|&b| block_has(b, *c));
            matching.iter().enumerate().any(|(i, _)| {
                // Re-pool all matching blocks except the one we use at index i
                let remaining: Vec<_> = matching[..i]
                    .iter()
                    .chain(&matching[i + 1..])
                    .chain(&non_matching)
                    .copied()
                    .collect();
                can_spell_functional(rest, remaining)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- idiomatic ---

    #[test]
    fn test_single_letter() {
        assert!(can_make_word("A"));
    }

    #[test]
    fn test_bark() {
        assert!(can_make_word("BARK"));
    }

    #[test]
    fn test_book_false() {
        // Two O's needed but only one O block after using (B,O) for B
        assert!(!can_make_word("BOOK"));
    }

    #[test]
    fn test_treat() {
        // Two T's needed — (G,T) and (T,G) both available
        assert!(can_make_word("TREAT"));
    }

    #[test]
    fn test_common_false() {
        // Two M's needed but only one M block
        assert!(!can_make_word("COMMON"));
    }

    #[test]
    fn test_squad() {
        assert!(can_make_word("SQUAD"));
    }

    #[test]
    fn test_confuse() {
        assert!(can_make_word("CONFUSE"));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(can_make_word("bark"));
        assert!(can_make_word("Confuse"));
        assert!(!can_make_word("book"));
    }

    // --- functional ---

    #[test]
    fn test_functional_single_letter() {
        assert!(can_make_word_functional("A"));
    }

    #[test]
    fn test_functional_bark() {
        assert!(can_make_word_functional("BARK"));
    }

    #[test]
    fn test_functional_book_false() {
        assert!(!can_make_word_functional("BOOK"));
    }

    #[test]
    fn test_functional_treat() {
        assert!(can_make_word_functional("TREAT"));
    }

    #[test]
    fn test_functional_common_false() {
        assert!(!can_make_word_functional("COMMON"));
    }

    #[test]
    fn test_functional_squad() {
        assert!(can_make_word_functional("SQUAD"));
    }

    #[test]
    fn test_functional_confuse() {
        assert!(can_make_word_functional("CONFUSE"));
    }

    #[test]
    fn test_both_implementations_agree() {
        let words = ["A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD", "CONFUSE"];
        for word in words {
            assert_eq!(
                can_make_word(word),
                can_make_word_functional(word),
                "disagreement on {word}"
            );
        }
    }
}
