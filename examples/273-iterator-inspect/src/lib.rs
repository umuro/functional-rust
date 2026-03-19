//! 273. Debugging iterators with inspect()
//!
//! `inspect(f)` taps into an iterator pipeline with a side-effect, passing values unchanged.

#[cfg(test)]
mod tests {
    #[test]
    fn test_inspect_no_change() {
        let result: Vec<i32> = [1, 2, 3].iter().copied().inspect(|_| {}).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_inspect_side_effect() {
        let mut seen = Vec::new();
        let _result: Vec<i32> = [1, 2, 3]
            .iter()
            .copied()
            .inspect(|&x| seen.push(x))
            .collect();
        assert_eq!(seen, vec![1, 2, 3]);
    }

    #[test]
    fn test_inspect_between_stages() {
        let mut after_filter = Vec::new();
        let result: Vec<i32> = (1..=6)
            .filter(|&x| x % 2 == 0)
            .inspect(|&x| after_filter.push(x))
            .map(|x| x * 10)
            .collect();
        assert_eq!(after_filter, vec![2, 4, 6]);
        assert_eq!(result, vec![20, 40, 60]);
    }
}
