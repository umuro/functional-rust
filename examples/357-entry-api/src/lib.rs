#![allow(clippy::all)]
//! # Entry API
//! Efficient insert-or-update with HashMap's Entry API.

use std::collections::HashMap;

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        map.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }
    map
}

pub fn get_or_compute<K, V, F>(map: &mut HashMap<K, V>, key: K, compute: F) -> &V
where
    K: Eq + std::hash::Hash,
    F: FnOnce() -> V,
{
    map.entry(key).or_insert_with(compute)
}

pub fn update_with_default<K, V, F>(map: &mut HashMap<K, V>, key: K, default: V, update: F)
where
    K: Eq + std::hash::Hash,
    F: FnOnce(&mut V),
{
    map.entry(key).and_modify(update).or_insert(default);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_counting() {
        let counts = count_chars("hello");
        assert_eq!(counts[&'l'], 2);
        assert_eq!(counts[&'o'], 1);
    }
    #[test]
    fn lazy_compute() {
        let mut map = HashMap::new();
        let v = get_or_compute(&mut map, "key", || 42);
        assert_eq!(*v, 42);
        let v2 = get_or_compute(&mut map, "key", || 99);
        assert_eq!(*v2, 42); // not recomputed
    }
    #[test]
    fn update_existing() {
        let mut map = HashMap::new();
        map.insert("k", 10);
        update_with_default(&mut map, "k", 0, |v| *v *= 2);
        assert_eq!(map["k"], 20);
    }
}
