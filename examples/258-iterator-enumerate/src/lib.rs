//! 258. Index-value pairs with enumerate()
//!
//! `enumerate()` adds a zero-based index to every iterator element.

#[cfg(test)]
mod tests {
    #[test]
    fn test_enumerate_indices() {
        let v = ["a", "b", "c"];
        let indices: Vec<usize> = v.iter().enumerate().map(|(i, _)| i).collect();
        assert_eq!(indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_enumerate_values() {
        let v = [10i32, 20, 30];
        let result: Vec<i32> = v
            .iter()
            .enumerate()
            .map(|(i, &val)| val + i as i32)
            .collect();
        assert_eq!(result, vec![10, 21, 32]);
    }

    #[test]
    fn test_enumerate_filter_even() {
        let v = ["a", "b", "c", "d"];
        let even: Vec<_> = v
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, v)| *v)
            .collect();
        assert_eq!(even, vec!["a", "c"]);
    }
}
