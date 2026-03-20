#![allow(clippy::all)]
// 1043: Interval Map — BTreeMap-based Range Storage
// Map non-overlapping intervals [lo, hi) to values

use std::collections::BTreeMap;

/// Interval map: stores non-overlapping [lo, hi) -> value mappings
struct IntervalMap<V> {
    // Key = interval start, Value = (end, mapped_value)
    map: BTreeMap<i64, (i64, V)>,
}

impl<V: Clone> IntervalMap<V> {
    fn new() -> Self {
        IntervalMap {
            map: BTreeMap::new(),
        }
    }

    /// Insert interval [lo, hi) -> value, removing overlapping intervals
    fn insert(&mut self, lo: i64, hi: i64, value: V) {
        assert!(lo < hi, "interval must be non-empty");

        // Remove all intervals that overlap with [lo, hi)
        let overlapping: Vec<i64> = self
            .map
            .range(..hi)
            .filter(|(_, (end, _))| *end > lo)
            .map(|(&start, _)| start)
            .collect();

        for start in overlapping {
            self.map.remove(&start);
        }

        self.map.insert(lo, (hi, value));
    }

    /// Query: find value at a point
    fn query(&self, point: i64) -> Option<&V> {
        // Find the interval whose start <= point
        self.map.range(..=point).next_back().and_then(
            |(_, (hi, v))| {
                if point < *hi {
                    Some(v)
                } else {
                    None
                }
            },
        )
    }

    /// List all intervals as (start, end, value) triples
    fn intervals(&self) -> Vec<(i64, i64, &V)> {
        self.map.iter().map(|(&lo, (hi, v))| (lo, *hi, v)).collect()
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

fn basic_ops() {
    let mut im = IntervalMap::new();
    im.insert(0, 10, "low");
    im.insert(10, 20, "mid");
    im.insert(20, 30, "high");

    assert_eq!(im.query(5), Some(&"low"));
    assert_eq!(im.query(15), Some(&"mid"));
    assert_eq!(im.query(25), Some(&"high"));
    assert_eq!(im.query(30), None);
    assert_eq!(im.query(-1), None);
    assert_eq!(im.len(), 3);
}

fn overlap_test() {
    let mut im = IntervalMap::new();
    im.insert(0, 10, "a");
    im.insert(5, 15, "b"); // overlaps with "a"

    // "b" replaced "a"
    assert_eq!(im.query(7), Some(&"b"));
    assert_eq!(im.query(12), Some(&"b"));
}

fn intervals_listing() {
    let mut im = IntervalMap::new();
    im.insert(0, 5, "x");
    im.insert(10, 20, "y");

    let intervals = im.intervals();
    assert_eq!(intervals.len(), 2);
    assert_eq!(intervals[0], (0, 5, &"x"));
    assert_eq!(intervals[1], (10, 20, &"y"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_ops();
    }

    #[test]
    fn test_overlap() {
        overlap_test();
    }

    #[test]
    fn test_listing() {
        intervals_listing();
    }

    #[test]
    fn test_boundary() {
        let mut im = IntervalMap::new();
        im.insert(0, 10, "a");
        // Point at boundary: [0, 10) means 0 is in, 10 is out
        assert_eq!(im.query(0), Some(&"a"));
        assert_eq!(im.query(9), Some(&"a"));
        assert_eq!(im.query(10), None);
    }

    #[test]
    fn test_adjacent() {
        let mut im = IntervalMap::new();
        im.insert(0, 5, "a");
        im.insert(5, 10, "b");
        assert_eq!(im.query(4), Some(&"a"));
        assert_eq!(im.query(5), Some(&"b"));
    }
}
