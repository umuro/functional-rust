#![allow(clippy::all)]
//! 264. Conditional stopping with take_while()
//!
//! `take_while(pred)` yields elements until the predicate first returns false.

#[cfg(test)]
mod tests {
    #[test]
    fn test_take_while_basic() {
        let result: Vec<i32> = [1, 2, 3, 4, 5]
            .iter()
            .copied()
            .take_while(|&x| x < 4)
            .collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_none_match() {
        let result: Vec<i32> = [-1i32, 2, 3]
            .iter()
            .copied()
            .take_while(|&x| x > 0)
            .collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_take_while_stops_early() {
        let result: Vec<i32> = [1i32, 2, 5, 1, 2]
            .iter()
            .copied()
            .take_while(|&x| x < 3)
            .collect();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_take_while_infinite() {
        let result: Vec<u32> = (0u32..).take_while(|&x| x < 5).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }
}
