//! 265. Conditional skipping with skip_while()
//!
//! `skip_while(pred)` discards elements until predicate first returns false, then yields all remaining.

#[cfg(test)]
mod tests {
    #[test]
    fn test_skip_while_basic() {
        let result: Vec<i32> = [1, 2, 3, 4, 5]
            .iter()
            .copied()
            .skip_while(|&x| x < 3)
            .collect();
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_skip_while_includes_later_matches() {
        let result: Vec<i32> = [0i32, 0, 1, 0]
            .iter()
            .copied()
            .skip_while(|&x| x == 0)
            .collect();
        assert_eq!(result, vec![1, 0]);
    }

    #[test]
    fn test_skip_while_all() {
        let result: Vec<i32> = [1, 2, 3].iter().copied().skip_while(|&x| x < 10).collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_skip_while_none() {
        let result: Vec<i32> = [1, 2, 3].iter().copied().skip_while(|&x| x > 10).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }
}
