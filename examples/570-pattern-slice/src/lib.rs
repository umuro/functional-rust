#![allow(clippy::all)]
//! Slice Patterns
//!
//! Matching arrays and slices with [a, b, c] patterns.

/// Match fixed-size array.
pub fn describe_triple(arr: &[i32; 3]) -> String {
    match arr {
        [0, 0, 0] => "all zeros".to_string(),
        [a, b, c] if a == b && b == c => format!("all same: {}", a),
        [a, _, c] if a == c => format!("first equals last: {}", a),
        [a, b, c] => format!("different: {}, {}, {}", a, b, c),
    }
}

/// Match slice head/tail.
pub fn first_and_rest(slice: &[i32]) -> Option<(i32, &[i32])> {
    match slice {
        [first, rest @ ..] => Some((*first, rest)),
        [] => None,
    }
}

/// Match multiple elements.
pub fn first_two_last(slice: &[i32]) -> Option<(i32, i32, i32)> {
    match slice {
        [first, second, .., last] => Some((*first, *second, *last)),
        _ => None,
    }
}

/// Match from end.
pub fn last_two(slice: &[i32]) -> Option<(i32, i32)> {
    match slice {
        [.., a, b] => Some((*a, *b)),
        _ => None,
    }
}

/// Match specific lengths.
pub fn describe_length(slice: &[i32]) -> &'static str {
    match slice {
        [] => "empty",
        [_] => "single",
        [_, _] => "pair",
        [_, _, _] => "triple",
        _ => "many",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple() {
        assert!(describe_triple(&[0, 0, 0]).contains("zeros"));
        assert!(describe_triple(&[5, 5, 5]).contains("same"));
    }

    #[test]
    fn test_first_and_rest() {
        let (first, rest) = first_and_rest(&[1, 2, 3]).unwrap();
        assert_eq!(first, 1);
        assert_eq!(rest, &[2, 3]);
    }

    #[test]
    fn test_first_two_last() {
        let (a, b, c) = first_two_last(&[1, 2, 3, 4, 5]).unwrap();
        assert_eq!((a, b, c), (1, 2, 5));
    }

    #[test]
    fn test_last_two() {
        assert_eq!(last_two(&[1, 2, 3]), Some((2, 3)));
    }

    #[test]
    fn test_describe_length() {
        assert_eq!(describe_length(&[]), "empty");
        assert_eq!(describe_length(&[1]), "single");
        assert_eq!(describe_length(&[1, 2, 3, 4]), "many");
    }
}
