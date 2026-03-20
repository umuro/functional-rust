#![allow(clippy::all)]
// Example 088: Iterator Consumers
// fold, collect, sum, max, find, position

use std::collections::HashMap;

// === Approach 1: fold — the universal consumer ===
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn product(data: &[i32]) -> i32 {
    data.iter().product()
}

fn concat_strs(data: &[&str]) -> String {
    data.iter().copied().collect()
}

// fold for custom accumulation
fn running_average(data: &[f64]) -> f64 {
    let (sum, count) = data.iter().fold((0.0, 0usize), |(s, c), &x| (s + x, c + 1));
    if count == 0 {
        0.0
    } else {
        sum / count as f64
    }
}

// === Approach 2: Specific consumers ===
fn find_first(data: &[i32], pred: impl Fn(&i32) -> bool) -> Option<&i32> {
    data.iter().find(|x| pred(x))
}

fn find_position(data: &[i32], pred: impl Fn(&i32) -> bool) -> Option<usize> {
    data.iter().position(|x| pred(x))
}

fn max_of(data: &[i32]) -> Option<&i32> {
    data.iter().max()
}

fn min_of(data: &[i32]) -> Option<&i32> {
    data.iter().min()
}

fn count_matching(data: &[i32], pred: impl Fn(&i32) -> bool) -> usize {
    data.iter().filter(|x| pred(x)).count()
}

fn any_match(data: &[i32], pred: impl Fn(&i32) -> bool) -> bool {
    data.iter().any(|x| pred(&x))
}

fn all_match(data: &[i32], pred: impl Fn(&i32) -> bool) -> bool {
    data.iter().all(|x| pred(&x))
}

// === Approach 3: Complex consumers ===
fn frequencies(data: &[i32]) -> HashMap<i32, usize> {
    let mut map = HashMap::new();
    for &x in data {
        *map.entry(x).or_insert(0) += 1;
    }
    map
}

// Equivalent using fold
fn frequencies_fold(data: &[i32]) -> HashMap<i32, usize> {
    data.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}

fn group_by<T, K: std::hash::Hash + Eq>(data: &[T], key: impl Fn(&T) -> K) -> HashMap<K, Vec<&T>> {
    let mut map: HashMap<K, Vec<&T>> = HashMap::new();
    for item in data {
        map.entry(key(item)).or_default().push(item);
    }
    map
}

// Collect into different types
fn collect_examples() {
    let data = vec![1, 2, 3, 4, 5];

    let _vec: Vec<i32> = data.iter().copied().collect();
    let _set: std::collections::HashSet<i32> = data.iter().copied().collect();
    let _string: String = data
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    // Collect Result<T, E> — stops at first error
    let results: Vec<Result<i32, _>> = vec![Ok(1), Ok(2), Ok(3)];
    let _all: Result<Vec<i32>, String> = results.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4, 5]), 120);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat_strs(&["a", "b", "c"]), "abc");
    }

    #[test]
    fn test_average() {
        assert!((running_average(&[1.0, 2.0, 3.0]) - 2.0).abs() < 1e-10);
        assert_eq!(running_average(&[]), 0.0);
    }

    #[test]
    fn test_find() {
        assert_eq!(find_first(&[1, 2, 3, 4, 5], |x| *x > 3), Some(&4));
        assert_eq!(find_first(&[1, 2, 3], |x| *x > 10), None);
    }

    #[test]
    fn test_position() {
        assert_eq!(find_position(&[1, 2, 3, 4, 5], |x| *x > 3), Some(3));
        assert_eq!(find_position(&[1, 2, 3], |x| *x > 10), None);
    }

    #[test]
    fn test_max_min() {
        assert_eq!(max_of(&[3, 1, 4, 1, 5, 9]), Some(&9));
        assert_eq!(min_of(&[3, 1, 4, 1, 5, 9]), Some(&1));
        assert_eq!(max_of(&[]), None);
    }

    #[test]
    fn test_count() {
        assert_eq!(count_matching(&[1, 2, 3, 4, 5, 6], |x| x % 2 == 0), 3);
    }

    #[test]
    fn test_any_all() {
        assert!(any_match(&[1, 2, 3, 4, 5, 6], |x| *x > 5));
        assert!(all_match(&[1, 2, 3], |x| *x > 0));
        assert!(!all_match(&[1, -2, 3], |x| *x > 0));
    }

    #[test]
    fn test_frequencies() {
        let f = frequencies(&[1, 2, 1, 3, 2, 1]);
        assert_eq!(f[&1], 3);
        assert_eq!(f[&2], 2);
        assert_eq!(f[&3], 1);
    }

    #[test]
    fn test_frequencies_fold() {
        let f = frequencies_fold(&[1, 2, 1, 3, 2, 1]);
        assert_eq!(f[&1], 3);
    }

    #[test]
    fn test_group_by() {
        let words = vec!["hello", "hi", "world", "wow"];
        let groups = group_by(&words, |w| w.chars().next().unwrap());
        assert_eq!(groups[&'h'].len(), 2);
        assert_eq!(groups[&'w'].len(), 2);
    }
}
