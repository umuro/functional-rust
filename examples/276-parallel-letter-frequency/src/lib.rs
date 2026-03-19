#![allow(clippy::all)]
use std::collections::HashMap;

/// Count letter frequencies in a single string, ignoring non-alphabetic characters.
/// All letters are lowercased before counting.
///
/// # Idiomatic Rust — fold over chars into a HashMap
pub fn letter_freq(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut map, c| {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_lowercase() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    })
}

/// Merge two frequency maps by summing counts.
pub fn merge_maps(mut a: HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    for (&ch, &count) in b {
        *a.entry(ch).or_insert(0) += count;
    }
    a
}

/// Map-reduce: compute letter frequencies across multiple texts.
///
/// Maps each text to its frequency map, then folds (reduces) all maps
/// into one by merging counts — the classic map-reduce pattern.
pub fn parallel_frequency(texts: &[&str]) -> HashMap<char, usize> {
    texts
        .iter()
        .map(|text| letter_freq(text))
        .fold(HashMap::new(), |acc, freq| merge_maps(acc, &freq))
}

/// Functional/recursive version — processes texts recursively instead of with fold.
pub fn parallel_frequency_recursive(texts: &[&str]) -> HashMap<char, usize> {
    match texts {
        [] => HashMap::new(),
        [single] => letter_freq(single),
        [head, rest @ ..] => {
            let head_freq = letter_freq(head);
            let rest_freq = parallel_frequency_recursive(rest);
            merge_maps(head_freq, &rest_freq)
        }
    }
}

/// Iterator-chain version using `for_each` for accumulation.
pub fn parallel_frequency_iter(texts: &[&str]) -> HashMap<char, usize> {
    let mut combined = HashMap::new();
    texts.iter().for_each(|text| {
        for c in text.chars() {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_lowercase() {
                *combined.entry(c).or_insert(0) += 1;
            }
        }
    });
    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert!(parallel_frequency(&[]).is_empty());
        assert!(parallel_frequency(&[""]).is_empty());
    }

    #[test]
    fn test_single_text() {
        let freq = parallel_frequency(&["aab"]);
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 1);
    }

    #[test]
    fn test_multiple_texts() {
        let freq = parallel_frequency(&["Hello World", "Functional Programming", "OCaml is Great"]);
        // 'l' appears in "Hello World" (3) + "Functional" (2) + "OCaml" (1) + "Great" (0) = 6
        // Let's just check some known chars
        assert_eq!(freq[&'o'], 5); // HellO WOrld, FunctiOnal PrOgramming, OCaml is Great
        assert!(freq[&'h'] >= 1);
    }

    #[test]
    fn test_ignores_non_alpha() {
        let freq = parallel_frequency(&["123!!!", "a1b2c3"]);
        assert_eq!(freq.len(), 3); // only a, b, c
        assert_eq!(freq[&'a'], 1);
    }

    #[test]
    fn test_case_insensitive() {
        let freq = parallel_frequency(&["AaBb"]);
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 2);
    }

    #[test]
    fn test_recursive_matches_iterative() {
        let texts = &["Hello", "World", "Rust"];
        let iter_result = parallel_frequency(texts);
        let rec_result = parallel_frequency_recursive(texts);
        assert_eq!(iter_result, rec_result);
    }

    #[test]
    fn test_iter_version_matches() {
        let texts = &["Hello", "World"];
        let fold_result = parallel_frequency(texts);
        let iter_result = parallel_frequency_iter(texts);
        assert_eq!(fold_result, iter_result);
    }
}
