#![allow(clippy::all)]
//! # BTreeMap Ordered
//! Sorted key-value map with efficient range queries.

use std::collections::BTreeMap;

pub fn sorted_map<K: Ord, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V> {
    pairs.into_iter().collect()
}

pub fn range_query<K: Ord + Clone, V: Clone>(
    map: &BTreeMap<K, V>,
    from: &K,
    to: &K,
) -> Vec<(K, V)> {
    map.range(from..=to)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

pub fn min_key<K: Clone, V>(map: &BTreeMap<K, V>) -> Option<K> {
    map.keys().next().cloned()
}

pub fn max_key<K: Clone, V>(map: &BTreeMap<K, V>) -> Option<K> {
    map.keys().next_back().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_iteration() {
        let m = sorted_map(vec![(3, 'c'), (1, 'a'), (2, 'b')]);
        let keys: Vec<_> = m.keys().cloned().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }
    #[test]
    fn range_works() {
        let m: BTreeMap<i32, i32> = (0..10).map(|i| (i, i * i)).collect();
        let r = range_query(&m, &3, &5);
        assert_eq!(r, vec![(3, 9), (4, 16), (5, 25)]);
    }
    #[test]
    fn min_max_keys() {
        let m = sorted_map(vec![(5, 'e'), (1, 'a'), (9, 'i')]);
        assert_eq!(min_key(&m), Some(1));
        assert_eq!(max_key(&m), Some(9));
    }
}
