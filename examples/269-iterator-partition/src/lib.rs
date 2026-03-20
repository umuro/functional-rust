#![allow(clippy::all)]
//! 269. Splitting by predicate with partition()
//!
//! `partition(pred)` splits an iterator into two collections in a single pass.

#[cfg(test)]
mod tests {
    #[test]
    fn test_partition_even_odd() {
        let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=6).partition(|&x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_partition_results() {
        let v: Vec<Result<i32, i32>> = vec![Ok(1), Err(2), Ok(3)];
        let (oks, errs): (Vec<_>, Vec<_>) = v.into_iter().partition(Result::is_ok);
        assert_eq!(oks.len(), 2);
        assert_eq!(errs.len(), 1);
    }

    #[test]
    fn test_partition_all_true() {
        let (yes, no): (Vec<i32>, Vec<i32>) = [2i32, 4, 6].iter().copied().partition(|&x| x > 0);
        assert_eq!(yes.len(), 3);
        assert!(no.is_empty());
    }
}
