// 1027: BTreeMap — Sorted Key Iteration
// Rust's BTreeMap keeps keys in sorted order (B-tree internally)

use std::collections::BTreeMap;

/// Build a BTreeMap and iterate — keys come out sorted
fn sorted_iteration() {
    let mut m = BTreeMap::new();
    m.insert(5, "five");
    m.insert(1, "one");
    m.insert(3, "three");
    m.insert(7, "seven");
    m.insert(2, "two");

    let keys: Vec<_> = m.keys().collect();
    assert_eq!(keys, vec![&1, &2, &3, &5, &7]);

    let values: Vec<_> = m.values().collect();
    assert_eq!(values, vec![&"one", &"two", &"three", &"five", &"seven"]);
}

/// Range queries using the `range` method
fn range_query() {
    let mut m = BTreeMap::new();
    for (k, v) in [(1, "a"), (2, "b"), (3, "c"), (4, "d"), (5, "e")] {
        m.insert(k, v);
    }

    // Get elements with keys in [2, 4]
    let range_keys: Vec<_> = m.range(2..=4).map(|(&k, _)| k).collect();
    assert_eq!(range_keys, vec![2, 3, 4]);

    // Half-open range [3, ∞)
    let tail: Vec<_> = m.range(3..).map(|(&k, _)| k).collect();
    assert_eq!(tail, vec![3, 4, 5]);
}

/// First and last key (min/max)
fn min_max() {
    let m: BTreeMap<i32, &str> = [(10, "ten"), (3, "three"), (7, "seven")]
        .into_iter()
        .collect();

    let (&min_k, _) = m.iter().next().unwrap();
    let (&max_k, _) = m.iter().next_back().unwrap();
    assert_eq!(min_k, 3);
    assert_eq!(max_k, 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_iteration() {
        sorted_iteration();
    }

    #[test]
    fn test_range_query() {
        range_query();
    }

    #[test]
    fn test_min_max() {
        min_max();
    }

    #[test]
    fn test_from_iterator() {
        let pairs = vec![(3, "c"), (1, "a"), (2, "b")];
        let m: BTreeMap<_, _> = pairs.into_iter().collect();
        let keys: Vec<_> = m.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }
}
