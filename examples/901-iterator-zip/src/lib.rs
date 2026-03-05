//! 257. Pairing elements with zip()
//!
//! `zip()` pairs elements from two iterators, stopping at the shorter one.
//! Like OCaml's `List.combine`, but lazy and infallible — no panic on length mismatch.

use std::collections::HashMap;

/// Pair two slices element-by-element, returning a Vec of tuples.
/// Stops at the shorter slice — never panics on length mismatch.
pub fn zip_slices<A: Copy, B: Copy>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x, y)).collect()
}

/// Pair names and scores into a HashMap.
pub fn names_to_scores<'a>(names: &[&'a str], scores: &[u32]) -> HashMap<&'a str, u32> {
    names
        .iter()
        .zip(scores.iter())
        .map(|(&name, &score)| (name, score))
        .collect()
}

/// Enumerate items: pair each element with its index (like `List.mapi` in OCaml).
pub fn indexed<T: Copy>(items: &[T]) -> Vec<(usize, T)> {
    items.iter().copied().enumerate().collect()
}

/// Unzip a Vec of pairs back into two Vecs (inverse of zip).
pub fn unzip_pairs<A, B>(pairs: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    pairs.into_iter().unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_equal_length() {
        let a = [1i32, 2, 3];
        let b = [10i32, 20, 30];
        assert_eq!(zip_slices(&a, &b), vec![(1, 10), (2, 20), (3, 30)]);
    }

    #[test]
    fn test_zip_truncates_at_shorter() {
        let long = [1i32, 2, 3, 4, 5];
        let short = [10i32, 20];
        let result = zip_slices(&long, &short);
        assert_eq!(result.len(), 2);
        assert_eq!(result, vec![(1, 10), (2, 20)]);
    }

    #[test]
    fn test_zip_empty() {
        let a: [i32; 0] = [];
        let b = [1i32, 2, 3];
        assert_eq!(zip_slices(&a, &b), vec![]);
    }

    #[test]
    fn test_names_to_scores() {
        let names = ["Alice", "Bob", "Carol"];
        let scores = [95u32, 87, 92];
        let map = names_to_scores(&names, &scores);
        assert_eq!(map["Alice"], 95);
        assert_eq!(map["Bob"], 87);
        assert_eq!(map["Carol"], 92);
    }

    #[test]
    fn test_indexed() {
        let items = ['a', 'b', 'c'];
        assert_eq!(indexed(&items), vec![(0, 'a'), (1, 'b'), (2, 'c')]);
    }

    #[test]
    fn test_unzip_roundtrip() {
        let pairs = vec![(1i32, 'a'), (2, 'b'), (3, 'c')];
        let (nums, chars) = unzip_pairs(pairs);
        assert_eq!(nums, vec![1, 2, 3]);
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }
}
