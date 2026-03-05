//! # Iterator collect()
//!
//! Materialize a lazy iterator into any `FromIterator<T>` type.

use std::collections::{BTreeMap, HashMap, HashSet, LinkedList};

/// Collect squares into a Vec
pub fn collect_squares(n: u32) -> Vec<u32> {
    (0..n).map(|x| x * x).collect()
}

/// Collect unique elements into a HashSet
pub fn unique_elements<T: std::hash::Hash + Eq>(items: Vec<T>) -> HashSet<T> {
    items.into_iter().collect()
}

/// Collect into a HashMap from pairs
pub fn pairs_to_map<K, V>(pairs: Vec<(K, V)>) -> HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    pairs.into_iter().collect()
}

/// Collect chars into a String
pub fn chars_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}

/// Collect into sorted BTreeMap
pub fn sorted_map<K: Ord, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V> {
    pairs.into_iter().collect()
}

/// Collect Result<T> iterator into Result<Vec<T>>
pub fn parse_all(strs: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    strs.iter().map(|s| s.parse::<i32>()).collect()
}

/// Alternative: Collect into LinkedList
pub fn collect_linked_list<T>(items: impl Iterator<Item = T>) -> LinkedList<T> {
    items.collect()
}

/// Using turbofish syntax
pub fn turbofish_example() -> Vec<i32> {
    (1..=5).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_vec() {
        let v = collect_squares(5);
        assert_eq!(v, vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_collect_hashset_dedup() {
        let set = unique_elements(vec![1, 2, 2, 3, 3, 3]);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_collect_hashmap() {
        let map = pairs_to_map(vec![(0, 0), (1, 1), (2, 4)]);
        assert_eq!(map[&2], 4);
    }

    #[test]
    fn test_collect_string() {
        let s = chars_to_string(&['a', 'b', 'c']);
        assert_eq!(s, "abc");
    }

    #[test]
    fn test_collect_btreemap_sorted() {
        let map = sorted_map(vec![(3, "c"), (1, "a"), (2, "b")]);
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec![&1, &2, &3]); // Sorted!
    }

    #[test]
    fn test_collect_result_ok() {
        let ok = parse_all(&["1", "2", "3"]);
        assert_eq!(ok.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_result_err() {
        let err = parse_all(&["1", "x", "3"]);
        assert!(err.is_err());
    }

    #[test]
    fn test_linked_list() {
        let ll = collect_linked_list(1..=4);
        assert_eq!(ll.len(), 4);
        assert_eq!(*ll.front().unwrap(), 1);
    }

    #[test]
    fn test_turbofish() {
        let v = turbofish_example();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
