#![allow(clippy::all)]
//! # Advanced Splitting Patterns
//!
//! Split iterators into multiple collections in a single pass — unzip, partition, and multi-way categorization.

/// Partition numbers into negative and non-negative
pub fn partition_by_sign(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    nums.into_iter().partition(|&x| x < 0)
}

/// Unzip pairs into two separate collections
pub fn unzip_pairs<A, B>(pairs: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    pairs.into_iter().unzip()
}

/// Partition map pattern: split by parse success
pub fn partition_parse(data: &[&str]) -> (Vec<i32>, Vec<String>) {
    data.iter()
        .fold((Vec::new(), Vec::new()), |(mut nums, mut words), s| {
            match s.parse::<i32>() {
                Ok(n) => nums.push(n),
                Err(_) => words.push(s.to_string()),
            }
            (nums, words)
        })
}

/// Trisect: split into negative, zero, positive
pub fn trisect(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    nums.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut neg, mut zero, mut pos), n| {
            if n < 0 {
                neg.push(n);
            } else if n == 0 {
                zero.push(n);
            } else {
                pos.push(n);
            }
            (neg, zero, pos)
        },
    )
}

/// Categorize by size
pub fn categorize_by_size(values: &[u32]) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    values.iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut small, mut medium, mut large), &v| {
            match v {
                0..=10 => small.push(v),
                11..=100 => medium.push(v),
                _ => large.push(v),
            }
            (small, medium, large)
        },
    )
}

/// Nested unzip - separate pairs-of-pairs
pub fn nested_unzip(nested: Vec<((i32, i32), char)>) -> (Vec<i32>, Vec<i32>, Vec<char>) {
    let (pairs, labels): (Vec<(i32, i32)>, Vec<char>) = nested.into_iter().unzip();
    let (lefts, rights): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    (lefts, rights, labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_by_sign() {
        let (neg, non_neg) = partition_by_sign(vec![-3, -1, 0, 1, 2, 5]);
        assert_eq!(neg, vec![-3, -1]);
        assert_eq!(non_neg, vec![0, 1, 2, 5]);
    }

    #[test]
    fn test_unzip_pairs() {
        let (nums, chars) = unzip_pairs(vec![(1, 'a'), (2, 'b'), (3, 'c')]);
        assert_eq!(nums, vec![1, 2, 3]);
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_partition_parse() {
        let (nums, words) = partition_parse(&["1", "two", "3", "four"]);
        assert_eq!(nums, vec![1, 3]);
        assert_eq!(words, vec!["two", "four"]);
    }

    #[test]
    fn test_trisect() {
        let (neg, zero, pos) = trisect(vec![-3, 0, 1, -1, 0, 5, -2, 3]);
        assert_eq!(neg, vec![-3, -1, -2]);
        assert_eq!(zero, vec![0, 0]);
        assert_eq!(pos, vec![1, 5, 3]);
    }

    #[test]
    fn test_categorize_by_size() {
        let (small, medium, large) = categorize_by_size(&[1, 15, 100, 8, 50, 3, 200]);
        assert_eq!(small, vec![1, 8, 3]);
        assert_eq!(medium, vec![15, 100, 50]);
        assert_eq!(large, vec![200]);
    }

    #[test]
    fn test_nested_unzip() {
        let (lefts, rights, labels) = nested_unzip(vec![((1, 2), 'a'), ((3, 4), 'b')]);
        assert_eq!(lefts, vec![1, 3]);
        assert_eq!(rights, vec![2, 4]);
        assert_eq!(labels, vec!['a', 'b']);
    }
}
