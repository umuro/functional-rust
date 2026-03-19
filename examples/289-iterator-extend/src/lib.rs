//! # Extending Collections with extend()
//!
//! `extend()` appends elements from an iterator to an existing collection in place.

use std::collections::{HashMap, HashSet};

/// Extend a vector with elements from another source
pub fn extend_vec_demo() -> Vec<i32> {
    let mut base = vec![1, 2, 3];
    base.extend([4, 5, 6]);
    base.extend(7..=9);
    base
}

/// Extend a string with characters
pub fn extend_string(base: &str, suffix: &str) -> String {
    let mut s = String::from(base);
    s.extend(suffix.chars());
    s
}

/// Extend a HashMap with new entries
pub fn extend_hashmap<'a>(
    base: &mut HashMap<&'a str, i32>,
    entries: impl IntoIterator<Item = (&'a str, i32)>,
) {
    base.extend(entries);
}

/// Extend a HashSet (duplicates are ignored)
pub fn extend_hashset(
    mut set: HashSet<i32>,
    extras: impl IntoIterator<Item = i32>,
) -> HashSet<i32> {
    set.extend(extras);
    set
}

/// Extend with filtered iterator
pub fn extend_with_filter(
    base: &mut Vec<i32>,
    source: impl Iterator<Item = i32>,
    predicate: impl Fn(&i32) -> bool,
) {
    base.extend(source.filter(predicate));
}

/// Incremental building pattern
pub fn build_incrementally(batches: &[&[i32]]) -> Vec<i32> {
    let mut result = Vec::new();
    for batch in batches {
        result.extend(*batch);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_vec_basic() {
        let mut v = vec![1i32, 2, 3];
        v.extend([4, 5, 6]);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_extend_vec_demo() {
        let v = extend_vec_demo();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_extend_string() {
        let s = extend_string("hello", " world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_extend_with_range() {
        let mut v: Vec<i32> = vec![];
        v.extend(1..=5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_extend_hashset_dedup() {
        let set: HashSet<i32> = [1, 2].iter().copied().collect();
        let extended = extend_hashset(set, [2, 3]);
        assert_eq!(extended.len(), 3);
    }

    #[test]
    fn test_extend_hashmap() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("a", 1);
        extend_hashmap(&mut map, [("b", 2), ("c", 3)]);
        assert_eq!(map.len(), 3);
        assert_eq!(map["b"], 2);
    }

    #[test]
    fn test_extend_with_filter() {
        let mut evens = vec![2, 4];
        extend_with_filter(&mut evens, 1..=10, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_build_incrementally() {
        let result = build_incrementally(&[&[1, 2], &[3, 4], &[5, 6]]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }
}
