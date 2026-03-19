#![allow(clippy::all)]
// Example 894: Step By, Enumerate, Rev
// Zero-cost iterator adapters for structured traversal

// === Approach 1: step_by — every nth element ===

pub fn every_nth(data: &[i32], n: usize) -> Vec<i32> {
    data.iter().step_by(n).copied().collect()
}

pub fn range_step(start: i32, stop: i32, step: usize) -> Vec<i32> {
    (start..stop).step_by(step).collect()
}

// === Approach 2: enumerate — pair elements with their index ===

pub fn find_with_index(data: &[i32], pred: impl Fn(&i32) -> bool) -> Option<(usize, i32)> {
    data.iter()
        .enumerate()
        .find(|(_, x)| pred(x))
        .map(|(i, &x)| (i, x))
}

pub fn indexed_filter(data: &[i32], pred: impl Fn(&i32) -> bool) -> Vec<(usize, i32)> {
    data.iter()
        .enumerate()
        .filter(|(_, x)| pred(x))
        .map(|(i, &x)| (i, x))
        .collect()
}

pub fn format_numbered(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i + 1, s))
        .collect()
}

// === Approach 3: rev — reverse iteration ===

pub fn reverse_collect<T: Copy>(data: &[T]) -> Vec<T> {
    data.iter().rev().copied().collect()
}

pub fn rev_map(data: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    data.iter().rev().map(|&x| f(x)).collect()
}

// Combining adapters: enumerate + rev to get (reversed-index, value)
pub fn enumerate_reversed(data: &[i32]) -> Vec<(usize, i32)> {
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- step_by tests ---

    #[test]
    fn test_every_nth_step2() {
        assert_eq!(every_nth(&[1, 2, 3, 4, 5, 6], 2), vec![1, 3, 5]);
    }

    #[test]
    fn test_every_nth_step3() {
        assert_eq!(
            every_nth(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
            vec![0, 3, 6, 9]
        );
    }

    #[test]
    fn test_every_nth_step1() {
        // step_by(1) returns all elements
        assert_eq!(every_nth(&[10, 20, 30], 1), vec![10, 20, 30]);
    }

    #[test]
    fn test_every_nth_empty() {
        assert_eq!(every_nth(&[], 2), Vec::<i32>::new());
    }

    #[test]
    fn test_range_step() {
        assert_eq!(range_step(0, 10, 3), vec![0, 3, 6, 9]);
        assert_eq!(range_step(1, 8, 2), vec![1, 3, 5, 7]);
    }

    // --- enumerate tests ---

    #[test]
    fn test_find_with_index_found() {
        assert_eq!(
            find_with_index(&[10, 20, 30, 40], |&x| x > 25),
            Some((2, 30))
        );
    }

    #[test]
    fn test_find_with_index_not_found() {
        assert_eq!(find_with_index(&[1, 2, 3], |&x| x > 100), None);
    }

    #[test]
    fn test_indexed_filter() {
        let result = indexed_filter(&[1, 2, 3, 4, 5], |&x| x % 2 == 0);
        assert_eq!(result, vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn test_format_numbered() {
        let items = vec!["alpha", "beta", "gamma"];
        assert_eq!(
            format_numbered(&items),
            vec!["1. alpha", "2. beta", "3. gamma"]
        );
    }

    // --- rev tests ---

    #[test]
    fn test_reverse_collect() {
        assert_eq!(reverse_collect(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_collect_empty() {
        assert_eq!(reverse_collect::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_rev_map() {
        // reverse then double
        assert_eq!(rev_map(&[1, 2, 3], |x| x * 2), vec![6, 4, 2]);
    }

    #[test]
    fn test_enumerate_reversed() {
        // reversed enumeration: last element gets index 0
        let result = enumerate_reversed(&[10, 20, 30]);
        assert_eq!(result, vec![(0, 30), (1, 20), (2, 10)]);
    }

    // --- composition tests ---

    #[test]
    fn test_step_then_enumerate() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result: Vec<(usize, i32)> = data
            .iter()
            .step_by(2)
            .enumerate()
            .map(|(i, &x)| (i, x))
            .collect();
        assert_eq!(result, vec![(0, 0), (1, 2), (2, 4), (3, 6), (4, 8)]);
    }

    #[test]
    fn test_step_by_rev() {
        // every 2nd element, reversed
        let data = [0, 1, 2, 3, 4, 5];
        let result: Vec<i32> = data.iter().step_by(2).rev().copied().collect();
        assert_eq!(result, vec![4, 2, 0]);
    }
}
