#![allow(clippy::all)]
//! # `matches!` Macro
//!
//! Test a value against a pattern and get a `bool` — without a full `match` expression.

/// User status enum for demonstration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Banned,
}

/// Check if a status is active using matches! macro.
pub fn is_active(status: &Status) -> bool {
    matches!(status, Status::Active)
}

/// Check if a status is usable (Active or Pending).
pub fn is_usable(status: &Status) -> bool {
    matches!(status, Status::Active | Status::Pending)
}

/// Alternative: traditional match approach (more verbose).
pub fn is_active_match(status: &Status) -> bool {
    match status {
        Status::Active => true,
        _ => false,
    }
}

/// Alternative: if-let approach.
pub fn is_active_if_let(status: &Status) -> bool {
    if let Status::Active = status {
        true
    } else {
        false
    }
}

/// Count active users in a slice.
pub fn count_active(users: &[Status]) -> usize {
    users.iter().filter(|u| matches!(u, Status::Active)).count()
}

/// Count usable users (Active or Pending).
pub fn count_usable(users: &[Status]) -> usize {
    users
        .iter()
        .filter(|u| matches!(u, Status::Active | Status::Pending))
        .count()
}

/// Filter even numbers that are small (≤ 6) using matches! with guard.
pub fn filter_even_small(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| matches!(n, x if x % 2 == 0 && x <= 6))
        .collect()
}

/// Alternative without matches! - more verbose.
pub fn filter_even_small_traditional(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n % 2 == 0 && n <= 6)
        .collect()
}

/// Shape enum with associated data.
#[derive(Debug, Clone)]
pub enum Shape {
    Circle(f64),
    Square(f64),
    Other,
}

/// Count circles in a collection.
pub fn count_circles(shapes: &[Shape]) -> usize {
    shapes
        .iter()
        .filter(|s| matches!(s, Shape::Circle(_)))
        .count()
}

/// Count large shapes (radius/side > 1.0).
pub fn count_large(shapes: &[Shape]) -> usize {
    shapes
        .iter()
        .filter(|s| matches!(s, Shape::Circle(r) | Shape::Square(r) if *r > 1.0))
        .count()
}

/// Check if a string matches known Rust keywords.
pub fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "fn" | "let" | "match" | "if" | "else" | "while" | "for" | "loop"
    )
}

/// Filter keywords from a slice of words.
pub fn filter_keywords<'a>(words: &[&'a str]) -> Vec<&'a str> {
    words
        .iter()
        .filter(|&&w| matches!(w, "fn" | "let" | "match" | "if"))
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_active() {
        assert!(is_active(&Status::Active));
        assert!(!is_active(&Status::Inactive));
        assert!(!is_active(&Status::Pending));
        assert!(!is_active(&Status::Banned));
    }

    #[test]
    fn test_is_usable() {
        assert!(is_usable(&Status::Active));
        assert!(is_usable(&Status::Pending));
        assert!(!is_usable(&Status::Inactive));
        assert!(!is_usable(&Status::Banned));
    }

    #[test]
    fn test_approaches_equivalent() {
        let statuses = [
            Status::Active,
            Status::Inactive,
            Status::Pending,
            Status::Banned,
        ];
        for s in &statuses {
            assert_eq!(is_active(s), is_active_match(s));
            assert_eq!(is_active(s), is_active_if_let(s));
        }
    }

    #[test]
    fn test_count_active() {
        let users = [
            Status::Active,
            Status::Inactive,
            Status::Pending,
            Status::Banned,
            Status::Active,
        ];
        assert_eq!(count_active(&users), 2);
    }

    #[test]
    fn test_count_usable() {
        let users = [
            Status::Active,
            Status::Inactive,
            Status::Pending,
            Status::Banned,
            Status::Active,
        ];
        assert_eq!(count_usable(&users), 3);
    }

    #[test]
    fn test_filter_even_small() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(filter_even_small(&nums), vec![2, 4, 6]);
        assert_eq!(
            filter_even_small(&nums),
            filter_even_small_traditional(&nums)
        );
    }

    #[test]
    fn test_matches_with_guard() {
        assert!(matches!(4, x if x % 2 == 0));
        assert!(!matches!(3, x if x % 2 == 0));
        assert!(matches!(6, x if x % 2 == 0 && x <= 6));
        assert!(!matches!(8, x if x % 2 == 0 && x <= 6));
    }

    #[test]
    fn test_shapes() {
        let shapes = vec![
            Shape::Circle(1.0),
            Shape::Square(2.0),
            Shape::Other,
            Shape::Circle(0.5),
        ];
        assert_eq!(count_circles(&shapes), 2);
        assert_eq!(count_large(&shapes), 1); // only Square(2.0)
    }

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("fn"));
        assert!(is_keyword("let"));
        assert!(is_keyword("match"));
        assert!(!is_keyword("hello"));
        assert!(!is_keyword("world"));
    }

    #[test]
    fn test_filter_keywords() {
        let words = ["fn", "let", "hello", "match", "world"];
        assert_eq!(filter_keywords(&words), vec!["fn", "let", "match"]);
    }

    #[test]
    fn test_matches_in_assert() {
        let r: Result<i32, &str> = Ok(42);
        assert!(matches!(r, Ok(n) if n > 0));

        let r2: Result<i32, &str> = Ok(-5);
        assert!(!matches!(r2, Ok(n) if n > 0));

        let r3: Result<i32, &str> = Err("error");
        assert!(!matches!(r3, Ok(_)));
    }
}
