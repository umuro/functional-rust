#![allow(clippy::all)]
//! 256. Chaining Iterators with chain()
//!
//! `chain()` concatenates two iterators lazily — no allocation for the combination,
//! just composition. The two source iterators are traversed in sequence.

/// Chain two slices into a collected Vec without allocating a combined slice.
pub fn chain_slices<T: Copy>(first: &[T], second: &[T]) -> Vec<T> {
    first.iter().chain(second.iter()).copied().collect()
}

/// Chain any two iterators that yield the same item type.
pub fn chain_iters<I, J, T>(a: I, b: J) -> Vec<T>
where
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    a.chain(b).collect()
}

/// Evens first, then odds — demonstrates chaining filtered iterators.
pub fn evens_then_odds(n: i32) -> Vec<i32> {
    let evens = (0..n).filter(|x| x % 2 == 0);
    let odds = (0..n).filter(|x| x % 2 != 0);
    evens.chain(odds).collect()
}

/// Chain three slices in sequence.
pub fn chain_three<T: Copy>(a: &[T], b: &[T], c: &[T]) -> Vec<T> {
    a.iter().chain(b.iter()).chain(c.iter()).copied().collect()
}

/// Sum a chain of two slices without collecting — fully lazy.
pub fn sum_chained(first: &[i32], second: &[i32]) -> i32 {
    first.iter().chain(second.iter()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_slices_integers() {
        let result = chain_slices(&[1, 2, 3], &[4, 5, 6]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_chain_slices_empty_first() {
        let result = chain_slices::<i32>(&[], &[1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_chain_slices_empty_second() {
        let result = chain_slices(&[1, 2, 3], &[]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_chain_slices_both_empty() {
        let result = chain_slices::<i32>(&[], &[]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_evens_then_odds() {
        let result = evens_then_odds(6);
        assert_eq!(result, vec![0, 2, 4, 1, 3, 5]);
    }

    #[test]
    fn test_chain_three() {
        let result = chain_three(&[1], &[2, 3], &[4, 5, 6]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sum_chained() {
        let total = sum_chained(&[1, 2, 3], &[4, 5, 6]);
        assert_eq!(total, 21);
    }

    #[test]
    fn test_chain_iters_ranges() {
        let result = chain_iters(0..3_i32, 10..13_i32);
        assert_eq!(result, vec![0, 1, 2, 10, 11, 12]);
    }
}
