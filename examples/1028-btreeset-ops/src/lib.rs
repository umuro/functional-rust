// 1028: BTreeSet — Union, Intersection, Difference
// Rust's BTreeSet provides sorted set with efficient set operations

use std::collections::BTreeSet;

/// Basic set operations: union, intersection, difference
fn basic_ops() {
    let a: BTreeSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let b: BTreeSet<i32> = [3, 4, 5, 6, 7].into_iter().collect();

    let union: Vec<_> = a.union(&b).copied().collect();
    assert_eq!(union, vec![1, 2, 3, 4, 5, 6, 7]);

    let inter: Vec<_> = a.intersection(&b).copied().collect();
    assert_eq!(inter, vec![3, 4, 5]);

    let diff: Vec<_> = a.difference(&b).copied().collect();
    assert_eq!(diff, vec![1, 2]);

    // Symmetric difference: elements in either but not both
    let sym_diff: Vec<_> = a.symmetric_difference(&b).copied().collect();
    assert_eq!(sym_diff, vec![1, 2, 6, 7]);
}

/// Subset and disjoint checks
fn subset_checks() {
    let small: BTreeSet<i32> = [2, 3].into_iter().collect();
    let big: BTreeSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let other: BTreeSet<i32> = [5, 6].into_iter().collect();

    assert!(small.is_subset(&big));
    assert!(!big.is_subset(&small));
    assert!(big.is_superset(&small));
    assert!(small.is_disjoint(&other));
}

/// Iterator-based operations
fn iter_ops() {
    let s: BTreeSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let sum: i32 = s.iter().sum();
    assert_eq!(sum, 15);

    let evens: BTreeSet<_> = s.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let expected: BTreeSet<i32> = [2, 4].into_iter().collect();
    assert_eq!(evens, expected);

    // Range query on sorted set
    let range: Vec<_> = s.range(2..=4).copied().collect();
    assert_eq!(range, vec![2, 3, 4]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ops() { basic_ops(); }

    #[test]
    fn test_subset_checks() { subset_checks(); }

    #[test]
    fn test_iter_ops() { iter_ops(); }

    #[test]
    fn test_operator_style() {
        let a: BTreeSet<i32> = [1, 2, 3].into_iter().collect();
        let b: BTreeSet<i32> = [2, 3, 4].into_iter().collect();
        // Bitwise operators work as set operations
        let union: BTreeSet<_> = &a | &b;
        let inter: BTreeSet<_> = &a & &b;
        assert_eq!(union, [1, 2, 3, 4].into_iter().collect());
        assert_eq!(inter, [2, 3].into_iter().collect());
    }
}
