#![allow(clippy::all)]
// 1030: Group Elements by Key — HashMap<K, Vec<V>>
// The classic group-by pattern using Entry API

use std::collections::HashMap;

/// Group words by their first character
fn group_by_first_letter() {
    let words = vec!["apple", "avocado", "banana", "blueberry", "cherry"];

    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
    for word in &words {
        let key = word.chars().next().unwrap();
        groups.entry(key).or_default().push(word);
    }

    assert_eq!(groups[&'a'], vec!["apple", "avocado"]);
    assert_eq!(groups[&'b'], vec!["banana", "blueberry"]);
    assert_eq!(groups[&'c'], vec!["cherry"]);
}

/// Group numbers by parity
fn group_by_parity() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut groups: HashMap<&str, Vec<i32>> = HashMap::new();
    for &n in &nums {
        let key = if n % 2 == 0 { "even" } else { "odd" };
        groups.entry(key).or_default().push(n);
    }

    assert_eq!(groups["even"], vec![2, 4, 6, 8]);
    assert_eq!(groups["odd"], vec![1, 3, 5, 7]);
}

/// Generic group_by function
fn group_by<T, K, F>(items: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups: HashMap<K, Vec<&T>> = HashMap::new();
    for item in items {
        groups.entry(key_fn(item)).or_default().push(item);
    }
    groups
}

fn test_generic_group_by() {
    let data = vec![("Alice", 90), ("Bob", 85), ("Alice", 92), ("Bob", 88)];
    let groups = group_by(&data, |&(name, _)| name);

    assert_eq!(groups["Alice"].len(), 2);
    assert_eq!(groups["Bob"].len(), 2);
    assert_eq!(groups["Alice"][0].1, 90);
    assert_eq!(groups["Alice"][1].1, 92);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_first_letter() {
        group_by_first_letter();
    }

    #[test]
    fn test_group_by_parity() {
        group_by_parity();
    }

    #[test]
    fn test_generic() {
        test_generic_group_by();
    }

    #[test]
    fn test_group_by_length() {
        let words = vec!["hi", "hey", "hello", "yo", "yes"];
        let groups = group_by(&words, |w| w.len());
        assert_eq!(groups[&2].len(), 2); // "hi", "yo"
        assert_eq!(groups[&3].len(), 2); // "hey", "yes"
        assert_eq!(groups[&5].len(), 1); // "hello"
    }
}
