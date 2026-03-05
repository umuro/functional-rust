//! # BTreeSet Sorted
//! Sorted set with set operations.

use std::collections::BTreeSet;

pub fn sorted_set<T: Ord>(items: Vec<T>) -> BTreeSet<T> { items.into_iter().collect() }

pub fn union<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.union(b).cloned().collect()
}

pub fn intersection<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.intersection(b).cloned().collect()
}

pub fn difference<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.difference(b).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sorted_order() {
        let s = sorted_set(vec![3, 1, 4, 1, 5]);
        let v: Vec<_> = s.iter().cloned().collect();
        assert_eq!(v, vec![1, 3, 4, 5]); // sorted, deduplicated
    }
    #[test] fn set_operations() {
        let a = sorted_set(vec![1, 2, 3]);
        let b = sorted_set(vec![2, 3, 4]);
        assert_eq!(intersection(&a, &b).iter().cloned().collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(difference(&a, &b).iter().cloned().collect::<Vec<_>>(), vec![1]);
    }
}
