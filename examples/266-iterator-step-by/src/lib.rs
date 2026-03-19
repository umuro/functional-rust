#![allow(clippy::all)]
//! 266. Striding with step_by()
//!
//! `step_by(n)` yields every nth element — the first, then skips n-1, and so on.

#[cfg(test)]
mod tests {
    #[test]
    fn test_step_by_3() {
        let result: Vec<usize> = (0..10).step_by(3).collect();
        assert_eq!(result, vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_step_by_2() {
        let result: Vec<i32> = [1, 2, 3, 4, 5].iter().copied().step_by(2).collect();
        assert_eq!(result, vec![1, 3, 5]);
    }

    #[test]
    fn test_step_by_1_identity() {
        let result: Vec<i32> = (1..=4).step_by(1).collect();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_step_by_multiples() {
        let result: Vec<i32> = (0..=20).step_by(5).collect();
        assert_eq!(result, vec![0, 5, 10, 15, 20]);
    }
}
