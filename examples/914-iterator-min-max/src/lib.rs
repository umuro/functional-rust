//! 275. Finding extremes: min() and max()
//!
//! `min()` and `max()` consume an iterator and return `Option<T>`.
//! They require `Ord` — use `min_by_key` / `max_by_key` for structs,
//! and `reduce(f64::min)` for floats which lack a total ordering.

/// Find the minimum integer in a slice, returning None for empty input.
pub fn slice_min(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().min()
}

/// Find the maximum integer in a slice, returning None for empty input.
pub fn slice_max(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().max()
}

/// Return the shortest string in a slice by character count.
pub fn shortest<'a>(words: &[&'a str]) -> Option<&'a str> {
    words.iter().copied().min_by_key(|w| w.len())
}

/// Return the longest string in a slice by character count.
pub fn longest<'a>(words: &[&'a str]) -> Option<&'a str> {
    words.iter().copied().max_by_key(|w| w.len())
}

#[derive(Debug, PartialEq)]
pub struct Student {
    pub name: &'static str,
    pub score: u32,
}

/// Return the student with the highest score.
pub fn top_student(students: &[Student]) -> Option<&Student> {
    students.iter().max_by_key(|s| s.score)
}

/// Return the student with the lowest score.
pub fn bottom_student(students: &[Student]) -> Option<&Student> {
    students.iter().min_by_key(|s| s.score)
}

/// Find the minimum of f64 values using reduce, since f64 is not Ord.
/// Returns None for empty input or if any value is NaN.
pub fn float_min(nums: &[f64]) -> Option<f64> {
    nums.iter().copied().reduce(f64::min)
}

/// Find the maximum of f64 values using reduce, since f64 is not Ord.
pub fn float_max(nums: &[f64]) -> Option<f64> {
    nums.iter().copied().reduce(f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_integers() {
        let nums = [3i32, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        assert_eq!(slice_min(&nums), Some(1));
        assert_eq!(slice_max(&nums), Some(9));
    }

    #[test]
    fn test_empty_returns_none() {
        let empty: &[i32] = &[];
        assert_eq!(slice_min(empty), None);
        assert_eq!(slice_max(empty), None);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(slice_min(&[42]), Some(42));
        assert_eq!(slice_max(&[42]), Some(42));
    }

    #[test]
    fn test_min_max_by_key_strings() {
        let words = ["banana", "apple", "fig", "kiwi", "cherry"];
        assert_eq!(shortest(&words), Some("fig"));
        assert_eq!(longest(&words), Some("banana"));
    }

    #[test]
    fn test_min_max_by_key_struct() {
        let students = vec![
            Student { name: "Alice", score: 95 },
            Student { name: "Bob", score: 72 },
            Student { name: "Carol", score: 88 },
        ];
        assert_eq!(top_student(&students).map(|s| s.name), Some("Alice"));
        assert_eq!(bottom_student(&students).map(|s| s.name), Some("Bob"));
    }

    #[test]
    fn test_float_min_max() {
        let floats = [3.1f64, 1.4, 1.5, 9.2, 2.6];
        assert_eq!(float_min(&floats), Some(1.4));
        assert_eq!(float_max(&floats), Some(9.2));
    }

    #[test]
    fn test_float_empty() {
        assert_eq!(float_min(&[]), None);
        assert_eq!(float_max(&[]), None);
    }

    #[test]
    fn test_all_same_values() {
        let nums = [7i32, 7, 7, 7];
        assert_eq!(slice_min(&nums), Some(7));
        assert_eq!(slice_max(&nums), Some(7));
    }
}
