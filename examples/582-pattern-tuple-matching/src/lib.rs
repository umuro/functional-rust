#![allow(clippy::all)]
//! # Tuple Pattern Matching
//!
//! Match on multiple values simultaneously using tuple patterns.

/// FizzBuzz using tuple pattern matching.
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => "FizzBuzz".into(),
        (true, false) => "Fizz".into(),
        (false, true) => "Buzz".into(),
        (false, false) => n.to_string(),
    }
}

/// Alternative using if-else chain.
pub fn fizzbuzz_if(n: u32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".into()
    } else if n % 3 == 0 {
        "Fizz".into()
    } else if n % 5 == 0 {
        "Buzz".into()
    } else {
        n.to_string()
    }
}

/// Traffic light state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

/// State machine for traffic light with emergency override.
pub fn next_light(light: Light, emergency: bool) -> Light {
    match (light, emergency) {
        (_, true) => Light::Red,
        (Light::Red, false) => Light::Green,
        (Light::Green, false) => Light::Yellow,
        (Light::Yellow, false) => Light::Red,
    }
}

/// Compare two values and return ordering description.
pub fn compare(a: i32, b: i32) -> &'static str {
    match (a > b, a < b) {
        (true, false) => "greater",
        (false, true) => "less",
        _ => "equal",
    }
}

/// Alternative using Ordering.
pub fn compare_ord(a: i32, b: i32) -> &'static str {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => "greater",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Equal => "equal",
    }
}

/// Match on three boolean conditions.
pub fn classify_triple(a: bool, b: bool, c: bool) -> &'static str {
    match (a, b, c) {
        (true, true, true) => "all true",
        (false, false, false) => "all false",
        (true, _, _) => "a is true",
        (_, true, _) => "b is true",
        (_, _, true) => "c is true",
        _ => "unreachable",
    }
}

/// Point classification using tuple matching.
pub fn quadrant(x: i32, y: i32) -> &'static str {
    match (x.signum(), y.signum()) {
        (1, 1) => "Q1",
        (-1, 1) => "Q2",
        (-1, -1) => "Q3",
        (1, -1) => "Q4",
        (0, _) | (_, 0) => "axis",
        _ => "origin",
    }
}

/// Match on Option pair.
pub fn both_some<T, U>(a: Option<T>, b: Option<U>) -> bool {
    matches!((a, b), (Some(_), Some(_)))
}

/// Extract values from Option pair.
pub fn extract_pair<T, U>(a: Option<T>, b: Option<U>) -> Option<(T, U)> {
    match (a, b) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn test_fizzbuzz_approaches_equivalent() {
        for n in 1..=30 {
            assert_eq!(fizzbuzz(n), fizzbuzz_if(n));
        }
    }

    #[test]
    fn test_next_light_normal() {
        assert_eq!(next_light(Light::Red, false), Light::Green);
        assert_eq!(next_light(Light::Green, false), Light::Yellow);
        assert_eq!(next_light(Light::Yellow, false), Light::Red);
    }

    #[test]
    fn test_next_light_emergency() {
        assert_eq!(next_light(Light::Red, true), Light::Red);
        assert_eq!(next_light(Light::Green, true), Light::Red);
        assert_eq!(next_light(Light::Yellow, true), Light::Red);
    }

    #[test]
    fn test_compare() {
        assert_eq!(compare(5, 3), "greater");
        assert_eq!(compare(3, 5), "less");
        assert_eq!(compare(4, 4), "equal");
    }

    #[test]
    fn test_compare_approaches_equivalent() {
        let cases = [(1, 2), (2, 1), (3, 3), (-1, 1), (0, 0)];
        for (a, b) in cases {
            assert_eq!(compare(a, b), compare_ord(a, b));
        }
    }

    #[test]
    fn test_classify_triple() {
        assert_eq!(classify_triple(true, true, true), "all true");
        assert_eq!(classify_triple(false, false, false), "all false");
        assert_eq!(classify_triple(true, false, false), "a is true");
        assert_eq!(classify_triple(false, true, false), "b is true");
        assert_eq!(classify_triple(false, false, true), "c is true");
    }

    #[test]
    fn test_quadrant() {
        assert_eq!(quadrant(1, 1), "Q1");
        assert_eq!(quadrant(-1, 1), "Q2");
        assert_eq!(quadrant(-1, -1), "Q3");
        assert_eq!(quadrant(1, -1), "Q4");
        assert_eq!(quadrant(0, 5), "axis");
        assert_eq!(quadrant(5, 0), "axis");
    }

    #[test]
    fn test_both_some() {
        assert!(both_some(Some(1), Some(2)));
        assert!(!both_some(Some(1), None::<i32>));
        assert!(!both_some(None::<i32>, Some(2)));
        assert!(!both_some(None::<i32>, None::<i32>));
    }

    #[test]
    fn test_extract_pair() {
        assert_eq!(extract_pair(Some(1), Some("a")), Some((1, "a")));
        assert_eq!(extract_pair(Some(1), None::<&str>), None);
        assert_eq!(extract_pair(None::<i32>, Some("a")), None);
    }
}
