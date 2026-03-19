// 1031: Count Frequencies
// The classic frequency counting pattern with Entry API

use std::collections::HashMap;

/// Count character frequencies
fn char_frequency() {
    let text = "abracadabra";

    let mut counts: HashMap<char, usize> = HashMap::new();
    for ch in text.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    assert_eq!(counts[&'a'], 5);
    assert_eq!(counts[&'b'], 2);
    assert_eq!(counts[&'r'], 2);
    assert_eq!(counts[&'c'], 1);
    assert_eq!(counts[&'d'], 1);
}

/// Count word frequencies using and_modify
fn word_frequency() {
    let words = vec!["the", "cat", "sat", "on", "the", "mat", "the", "cat"];

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in &words {
        counts.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }

    assert_eq!(counts["the"], 3);
    assert_eq!(counts["cat"], 2);
    assert_eq!(counts["sat"], 1);
}

/// Find the most frequent element
fn most_frequent() {
    let items = vec![1, 2, 3, 2, 1, 2, 3, 2, 2];

    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &x in &items {
        *counts.entry(x).or_insert(0) += 1;
    }

    let (most, count) = counts
        .iter()
        .max_by_key(|&(_, &v)| v)
        .map(|(&k, &v)| (k, v))
        .unwrap();

    assert_eq!(most, 2);
    assert_eq!(count, 5);
}

/// Frequency counting with iterators (functional style)
fn functional_counting() {
    let data = vec![1, 1, 2, 3, 3, 3];

    let counts: HashMap<i32, usize> = data.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    assert_eq!(counts[&1], 2);
    assert_eq!(counts[&3], 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_frequency() {
        char_frequency();
    }

    #[test]
    fn test_word_frequency() {
        word_frequency();
    }

    #[test]
    fn test_most_frequent() {
        most_frequent();
    }

    #[test]
    fn test_functional_counting() {
        functional_counting();
    }

    #[test]
    fn test_empty_input() {
        let empty: Vec<i32> = vec![];
        let counts: HashMap<i32, usize> = empty.iter().fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
        assert!(counts.is_empty());
    }
}
