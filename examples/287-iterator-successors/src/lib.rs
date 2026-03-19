//! # Recursive Sequences with successors()
//!
//! `successors(first, f)` generates a sequence: first, f(first), f(f(first)), ...

/// Generate powers of 2 up to a maximum
pub fn powers_of_2(max: u32) -> impl Iterator<Item = u32> {
    std::iter::successors(
        Some(1u32),
        move |&n| {
            if n < max {
                Some(n * 2)
            } else {
                None
            }
        },
    )
}

/// Generate the Collatz sequence from a starting number
pub fn collatz(start: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(start), |&n| {
        if n == 1 {
            None
        } else if n % 2 == 0 {
            Some(n / 2)
        } else {
            Some(3 * n + 1)
        }
    })
}

/// Generate a geometric sequence (multiply by factor each step)
pub fn geometric(start: i32, factor: i32, max: i32) -> impl Iterator<Item = i32> {
    std::iter::successors(Some(start), move |&n| {
        let next = n.saturating_mul(factor);
        if next.abs() > max.abs() {
            None
        } else {
            Some(next)
        }
    })
}

/// Newton's method to approximate square root
pub fn newton_sqrt(target: f64, tolerance: f64) -> impl Iterator<Item = f64> {
    std::iter::successors(Some(1.0f64), move |&x| {
        let next = 0.5 * (x + target / x);
        if (next - x).abs() < tolerance {
            None
        } else {
            Some(next)
        }
    })
}

/// Shrinking string - remove first character each step
pub fn shrinking_string(s: String) -> impl Iterator<Item = String> {
    std::iter::successors(Some(s), |s| {
        if s.is_empty() {
            None
        } else {
            Some(s.chars().skip(1).collect())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powers_of_2() {
        let result: Vec<u32> = powers_of_2(16).collect();
        assert_eq!(result, vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_powers_of_2_large() {
        let result: Vec<u32> = powers_of_2(512).collect();
        assert_eq!(result, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
    }

    #[test]
    fn test_collatz_6() {
        let result: Vec<u64> = collatz(6).collect();
        assert_eq!(result, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_27() {
        let result: Vec<u64> = collatz(27).collect();
        assert_eq!(result.len(), 112); // Famous long sequence
        assert_eq!(*result.last().unwrap(), 1);
    }

    #[test]
    fn test_geometric() {
        let result: Vec<i32> = geometric(1, 3, 729).collect();
        assert_eq!(result, vec![1, 3, 9, 27, 81, 243, 729]);
    }

    #[test]
    fn test_newton_sqrt() {
        let result: Vec<f64> = newton_sqrt(2.0, 1e-10).collect();
        let final_approx = *result.last().unwrap();
        assert!((final_approx - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_shrinking_string() {
        let result: Vec<String> = shrinking_string("abc".to_string()).collect();
        assert_eq!(
            result,
            vec![
                "abc".to_string(),
                "bc".to_string(),
                "c".to_string(),
                "".to_string()
            ]
        );
    }

    #[test]
    fn test_successors_empty_if_first_is_none() {
        let result: Vec<i32> = std::iter::successors(None, |&_n: &i32| Some(1)).collect();
        assert!(result.is_empty());
    }
}
