#![allow(clippy::all)]
// Solution 1: Idiomatic Rust — Option's built-in monadic combinators
// `and_then` is Rust's bind (>>=), `map` is Rust's fmap (>>|)
pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

// Takes &[i32] — borrows the slice, no allocation needed
pub fn safe_head(list: &[i32]) -> Option<i32> {
    list.first().copied()
}

pub fn compute_idiomatic(lst: &[i32]) -> Option<i32> {
    safe_head(lst).and_then(|x| safe_div(100, x)).map(|r| r * 2)
}

// Solution 2: Explicit monadic bind — mirrors OCaml's >>= operator
// Demonstrates what and_then desugars to. Note: >>| (fmap) IS Option::map.
fn bind<T, U>(opt: Option<T>, f: impl FnOnce(T) -> Option<U>) -> Option<U> {
    match opt {
        None => None,
        Some(x) => f(x),
    }
}

pub fn compute_explicit(lst: &[i32]) -> Option<i32> {
    let divided = bind(safe_head(lst), |x| safe_div(100, x));
    divided.map(|r| r * 2) // >>| is just Option::map
}

// Solution 3: Using the `?` operator — Rust's ergonomic monadic shorthand
// `?` early-returns None if the value is None, like >>= but with explicit control flow
pub fn compute_question_mark(lst: &[i32]) -> Option<i32> {
    let x = safe_head(lst)?;
    let r = safe_div(100, x)?;
    Some(r * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case_all_approaches() {
        let lst = &[5, 3, 1];
        // 100 / 5 = 20, 20 * 2 = 40
        assert_eq!(compute_idiomatic(lst), Some(40));
        assert_eq!(compute_explicit(lst), Some(40));
        assert_eq!(compute_question_mark(lst), Some(40));
    }

    #[test]
    fn test_division_by_zero_propagates_none() {
        let lst = &[0, 1];
        // safe_div(100, 0) => None, propagates
        assert_eq!(compute_idiomatic(lst), None);
        assert_eq!(compute_explicit(lst), None);
        assert_eq!(compute_question_mark(lst), None);
    }

    #[test]
    fn test_empty_list_propagates_none() {
        let lst: &[i32] = &[];
        // safe_head([]) => None, propagates
        assert_eq!(compute_idiomatic(lst), None);
        assert_eq!(compute_explicit(lst), None);
        assert_eq!(compute_question_mark(lst), None);
    }

    #[test]
    fn test_single_element_list() {
        // 100 / 4 = 25, 25 * 2 = 50
        assert_eq!(compute_idiomatic(&[4]), Some(50));
        assert_eq!(compute_explicit(&[4]), Some(50));
        assert_eq!(compute_question_mark(&[4]), Some(50));
    }

    #[test]
    fn test_safe_div_nonzero() {
        assert_eq!(safe_div(100, 5), Some(20));
        assert_eq!(safe_div(7, 3), Some(2)); // integer division
    }

    #[test]
    fn test_safe_div_by_zero() {
        assert_eq!(safe_div(100, 0), None);
        assert_eq!(safe_div(0, 0), None);
    }

    #[test]
    fn test_safe_head() {
        assert_eq!(safe_head(&[1, 2, 3]), Some(1));
        assert_eq!(safe_head(&[42]), Some(42));
        assert_eq!(safe_head(&[]), None);
    }

    #[test]
    fn test_negative_head_element() {
        // safe_div(100, -4) = -25, -25 * 2 = -50
        assert_eq!(compute_idiomatic(&[-4, 1]), Some(-50));
        assert_eq!(compute_explicit(&[-4, 1]), Some(-50));
        assert_eq!(compute_question_mark(&[-4, 1]), Some(-50));
    }
}
