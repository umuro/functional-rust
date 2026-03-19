#![allow(clippy::all)]
//! Map-Reduce with Closures
//!
//! Transforming and aggregating data with iterator map and fold.

use std::collections::HashMap;

/// Generic map-reduce: transform each element, then aggregate.
pub fn map_reduce<T, U, V, M, R>(items: &[T], mapper: M, reducer: R, init: V) -> V
where
    M: Fn(&T) -> U,
    R: Fn(V, U) -> V,
{
    items.iter().map(mapper).fold(init, reducer)
}

/// Word frequency count via map-reduce.
pub fn word_count<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    words.iter().fold(HashMap::new(), |mut acc, &word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    })
}

/// Sum of squares via map-reduce.
pub fn sum_of_squares(nums: &[i32]) -> i32 {
    map_reduce(nums, |&x| x * x, |acc, x| acc + x, 0)
}

/// Product of all elements.
pub fn product(nums: &[i32]) -> i32 {
    map_reduce(nums, |&x| x, |acc, x| acc * x, 1)
}

/// Concatenate strings with separator.
pub fn join_strings(strings: &[&str], sep: &str) -> String {
    if strings.is_empty() {
        return String::new();
    }
    strings[1..].iter().fold(strings[0].to_string(), |acc, &s| {
        format!("{}{}{}", acc, sep, s)
    })
}

/// Group by key function.
pub fn group_by_key<T: Clone, K: std::hash::Hash + Eq, F>(
    items: &[T],
    key_fn: F,
) -> HashMap<K, Vec<T>>
where
    F: Fn(&T) -> K,
{
    items.iter().fold(HashMap::new(), |mut acc, item| {
        acc.entry(key_fn(item)).or_default().push(item.clone());
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_reduce_sum() {
        let nums = [1, 2, 3, 4, 5];
        let sum = map_reduce(&nums, |&x| x, |acc, x| acc + x, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14); // 1 + 4 + 9
        assert_eq!(sum_of_squares(&[]), 0);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4]), 24);
        assert_eq!(product(&[5]), 5);
    }

    #[test]
    fn test_word_count() {
        let words = vec!["hello", "world", "hello", "rust", "world", "world"];
        let counts = word_count(&words);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&3));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_join_strings() {
        assert_eq!(join_strings(&["a", "b", "c"], ", "), "a, b, c");
        assert_eq!(join_strings(&["one"], "-"), "one");
        assert_eq!(join_strings(&[], "-"), "");
    }

    #[test]
    fn test_group_by_key() {
        let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
        let grouped = group_by_key(&words, |w| w.chars().next().unwrap());

        assert_eq!(grouped.get(&'a').unwrap().len(), 2);
        assert_eq!(grouped.get(&'b').unwrap().len(), 2);
        assert_eq!(grouped.get(&'c').unwrap().len(), 1);
    }

    #[test]
    fn test_map_reduce_max() {
        let nums = [3, 1, 4, 1, 5, 9, 2, 6];
        let max = map_reduce(&nums, |&x| x, |acc, x| acc.max(x), i32::MIN);
        assert_eq!(max, 9);
    }
}
