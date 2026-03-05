//! # DoubleEndedIterator and rev()
//!
//! `DoubleEndedIterator` enables traversal from both ends; `rev()` swaps the direction.

/// A counter that can be consumed from either end
pub struct Counter {
    front: i32,
    back: i32,
}

impl Counter {
    pub fn new(n: i32) -> Self {
        Counter { front: 1, back: n }
    }

    pub fn range(start: i32, end: i32) -> Self {
        Counter {
            front: start,
            back: end,
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.front > self.back {
            return None;
        }
        let v = self.front;
        self.front += 1;
        Some(v)
    }
}

impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<i32> {
        if self.front > self.back {
            return None;
        }
        let v = self.back;
        self.back -= 1;
        Some(v)
    }
}

/// Reverse a string using DoubleEndedIterator
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Check if a sequence is a palindrome using DoubleEndedIterator
pub fn is_palindrome<I>(iter: I) -> bool
where
    I: DoubleEndedIterator + Clone,
    I::Item: PartialEq,
{
    let mut forward = iter.clone();
    let mut backward = iter.rev();

    loop {
        match (forward.next(), backward.next()) {
            (Some(a), Some(b)) if a == b => continue,
            (None, None) => return true,
            _ => return false,
        }
    }
}

/// Consume from both ends simultaneously
pub fn from_both_ends<I>(mut iter: I) -> Vec<(Option<I::Item>, Option<I::Item>)>
where
    I: DoubleEndedIterator,
{
    let mut result = Vec::new();
    loop {
        let front = iter.next();
        let back = iter.next_back();
        if front.is_none() && back.is_none() {
            break;
        }
        result.push((front, back));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_range() {
        let result: Vec<i32> = (1..=5).rev().collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_custom_dei_rev() {
        let result: Vec<i32> = Counter::new(5).rev().collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_next_back() {
        let mut c = Counter::new(5);
        assert_eq!(c.next_back(), Some(5));
        assert_eq!(c.next_back(), Some(4));
        assert_eq!(c.next(), Some(1));
    }

    #[test]
    fn test_rev_collect_string() {
        let result = reverse_string("hello");
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar".chars()));
        assert!(is_palindrome([1, 2, 3, 2, 1].iter()));
        assert!(!is_palindrome("hello".chars()));
    }

    #[test]
    fn test_from_both_ends() {
        let result = from_both_ends(Counter::new(5));
        assert_eq!(
            result,
            vec![
                (Some(1), Some(5)),
                (Some(2), Some(4)),
                (Some(3), None),
            ]
        );
    }

    #[test]
    fn test_last_3_evens_reversed() {
        let last_3_even: Vec<i32> = (1..=20).filter(|x| x % 2 == 0).rev().take(3).collect();
        assert_eq!(last_3_even, vec![20, 18, 16]);
    }
}
