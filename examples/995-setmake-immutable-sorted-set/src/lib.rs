// Set.Make — Immutable Sorted Set
//
// OCaml's Set.Make(Ord) creates a module for a persistent, immutable balanced
// BST set over a given ordered type. Rust's BTreeSet<T> provides the same
// semantics: ordered, no duplicates, O(log n) operations, clone-on-modify.

use std::collections::BTreeSet;

// --- idiomatic Rust -----------------------------------------------------------

/// Build a BTreeSet from a slice, mirroring OCaml's `Set.of_list`.
pub fn set_of_slice<T: Ord + Clone>(items: &[T]) -> BTreeSet<T> {
    items.iter().cloned().collect()
}

/// Union of two sets — elements present in either.
/// Mirrors `Set.union s1 s2`.
pub fn union<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.union(b).cloned().collect()
}

/// Intersection of two sets — elements present in both.
/// Mirrors `Set.inter s1 s2`.
pub fn inter<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.intersection(b).cloned().collect()
}

/// Set difference — elements in `a` but not in `b`.
/// Mirrors `Set.diff s1 s2`.
pub fn diff<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.difference(b).cloned().collect()
}

/// Return sorted elements as a Vec, mirroring `Set.elements`.
pub fn elements<T: Ord + Clone>(s: &BTreeSet<T>) -> Vec<T> {
    s.iter().cloned().collect()
}

// --- functional / iterator-chained variants -----------------------------------

/// Build a set from any iterator — shows the functional composition.
pub fn set_from_iter<T: Ord, I: IntoIterator<Item = T>>(iter: I) -> BTreeSet<T> {
    iter.into_iter().collect()
}

/// Check membership, mirroring `Set.mem x s`.
pub fn mem<T: Ord>(x: &T, s: &BTreeSet<T>) -> bool {
    s.contains(x)
}

/// Add an element, returning a new set — immutable style.
/// OCaml: `Set.add x s` returns a new set.
pub fn add<T: Ord + Clone>(x: T, s: &BTreeSet<T>) -> BTreeSet<T> {
    let mut next = s.clone(); // clone is the cost of immutability
    next.insert(x);
    next
}

/// Remove an element, returning a new set — immutable style.
/// OCaml: `Set.remove x s` returns a new set.
pub fn remove<T: Ord + Clone>(x: &T, s: &BTreeSet<T>) -> BTreeSet<T> {
    let mut next = s.clone(); // intentional clone for persistent semantics
    next.remove(x);
    next
}

/// Apply a predicate to filter a set, mirroring `Set.filter pred s`.
pub fn filter<T: Ord + Clone, F: Fn(&T) -> bool>(pred: F, s: &BTreeSet<T>) -> BTreeSet<T> {
    s.iter().filter(|x| pred(x)).cloned().collect()
}

/// Map a function over a set, collecting results into a new set.
/// OCaml: `Set.map f s`.
pub fn map_set<T: Ord + Clone, U: Ord, F: Fn(&T) -> U>(f: F, s: &BTreeSet<T>) -> BTreeSet<U> {
    s.iter().map(f).collect()
}

/// Fold over a set in ascending order, mirroring `Set.fold`.
pub fn fold_set<T: Ord, A, F: Fn(A, &T) -> A>(f: F, s: &BTreeSet<T>, init: A) -> A {
    s.iter().fold(init, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s1() -> BTreeSet<i32> {
        set_of_slice(&[1, 3, 5, 7, 9])
    }

    fn s2() -> BTreeSet<i32> {
        set_of_slice(&[2, 3, 5, 7, 11])
    }

    // --- set_of_slice ---

    #[test]
    fn test_of_slice_empty() {
        let s: BTreeSet<i32> = set_of_slice(&[]);
        assert!(s.is_empty());
    }

    #[test]
    fn test_of_slice_deduplicates() {
        let s = set_of_slice(&[1, 1, 2, 2, 3]);
        assert_eq!(elements(&s), vec![1, 2, 3]);
    }

    #[test]
    fn test_of_slice_sorted() {
        let s = set_of_slice(&[5, 1, 3, 2, 4]);
        assert_eq!(elements(&s), vec![1, 2, 3, 4, 5]);
    }

    // --- union ---

    #[test]
    fn test_union() {
        let u = union(&s1(), &s2());
        assert_eq!(elements(&u), vec![1, 2, 3, 5, 7, 9, 11]);
    }

    #[test]
    fn test_union_with_empty() {
        let empty: BTreeSet<i32> = BTreeSet::new();
        assert_eq!(union(&s1(), &empty), s1());
    }

    // --- inter ---

    #[test]
    fn test_inter() {
        let i = inter(&s1(), &s2());
        assert_eq!(elements(&i), vec![3, 5, 7]);
    }

    #[test]
    fn test_inter_disjoint() {
        let a = set_of_slice(&[1, 2]);
        let b = set_of_slice(&[3, 4]);
        assert!(inter(&a, &b).is_empty());
    }

    // --- diff ---

    #[test]
    fn test_diff() {
        let d = diff(&s1(), &s2());
        assert_eq!(elements(&d), vec![1, 9]);
    }

    #[test]
    fn test_diff_with_empty() {
        let empty: BTreeSet<i32> = BTreeSet::new();
        assert_eq!(diff(&s1(), &empty), s1());
    }

    // --- mem ---

    #[test]
    fn test_mem_present() {
        assert!(mem(&3, &s1()));
    }

    #[test]
    fn test_mem_absent() {
        assert!(!mem(&4, &s1()));
    }

    // --- add / remove ---

    #[test]
    fn test_add_new_element() {
        let s = add(4, &s1());
        assert!(mem(&4, &s));
        assert!(!mem(&4, &s1())); // original unchanged
    }

    #[test]
    fn test_add_existing_element() {
        let s = add(3, &s1());
        assert_eq!(s.len(), s1().len()); // no duplicates
    }

    #[test]
    fn test_remove_element() {
        let s = remove(&3, &s1());
        assert!(!mem(&3, &s));
        assert!(mem(&3, &s1())); // original unchanged
    }

    // --- filter ---

    #[test]
    fn test_filter_even() {
        let s = set_of_slice(&[1, 2, 3, 4, 5, 6]);
        let evens = filter(|x| x % 2 == 0, &s);
        assert_eq!(elements(&evens), vec![2, 4, 6]);
    }

    // --- map_set ---

    #[test]
    fn test_map_set_double() {
        let s = set_of_slice(&[1, 2, 3]);
        let doubled: BTreeSet<i32> = map_set(|x| x * 2, &s);
        assert_eq!(elements(&doubled), vec![2, 4, 6]);
    }

    #[test]
    fn test_map_set_may_merge() {
        // mapping i32 → bool can collapse many elements to {true, false}
        let s = set_of_slice(&[1, 2, 3, 4]);
        let parity: BTreeSet<bool> = map_set(|x| x % 2 == 0, &s);
        assert_eq!(parity.len(), 2);
    }

    // --- fold_set ---

    #[test]
    fn test_fold_sum() {
        let s = set_of_slice(&[1, 2, 3, 4, 5]);
        let sum = fold_set(|acc, x| acc + x, &s, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_fold_ascending_order() {
        let s = set_of_slice(&[3, 1, 4, 1, 5]);
        let ordered = fold_set(
            |mut acc, x| {
                acc.push(*x);
                acc
            },
            &s,
            Vec::new(),
        );
        assert_eq!(ordered, vec![1, 3, 4, 5]);
    }

    // --- set_from_iter ---

    #[test]
    fn test_from_iter() {
        let s: BTreeSet<i32> = set_from_iter(1..=5);
        assert_eq!(elements(&s), vec![1, 2, 3, 4, 5]);
    }
}
