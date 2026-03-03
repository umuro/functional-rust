/// Option and Result: safe error handling without exceptions.
///
/// OCaml uses `option` and `result` types. Rust has `Option<T>` and `Result<T, E>`.
/// Both replace null/exceptions with types the compiler forces you to handle.

// ── Option: safe lookups ────────────────────────────────────────────────────

/// Find first element matching predicate (idiomatic Rust)
pub fn find_first<T>(list: &[T], pred: impl Fn(&T) -> bool) -> Option<&T> {
    list.iter().find(|x| pred(x))
}

/// Safe division — returns None on divide-by-zero
pub fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

/// Safe head of list
pub fn head<T>(list: &[T]) -> Option<&T> {
    list.first()
}

/// Safe last element
pub fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}

// ── Option combinators (functional chaining) ────────────────────────────────

/// Chain optional operations: find element, then transform it
pub fn find_and_double(list: &[i64], pred: impl Fn(&i64) -> bool) -> Option<i64> {
    list.iter().find(|x| pred(x)).map(|x| x * 2)
}

/// Get the nth element safely, with a default
pub fn nth_or_default<T: Clone>(list: &[T], n: usize, default: T) -> T {
    list.get(n).cloned().unwrap_or(default)
}

// ── Result: error handling with context ─────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

/// Safe division returning Result with error context
pub fn checked_div(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        a.checked_div(b).ok_or(MathError::Overflow)
    }
}

/// Safe square root
pub fn checked_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

/// Chain computations with `?` operator — Rust's monadic bind
/// Computes: sqrt(a / b)
pub fn sqrt_of_division(a: f64, b: f64) -> Result<f64, MathError> {
    let quotient = safe_div(a, b).ok_or(MathError::DivisionByZero)?;
    checked_sqrt(quotient)
}

// ── Recursive style: Option threading ───────────────────────────────────────

/// Recursive lookup in an association list (like OCaml's List.assoc_opt)
pub fn assoc_opt<'a, K: PartialEq, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V> {
    match pairs.split_first() {
        None => None,
        Some(((k, v), _)) if k == key => Some(v),
        Some((_, rest)) => assoc_opt(key, rest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10.0, 2.0), Some(5.0));
        assert_eq!(safe_div(1.0, 0.0), None);
    }

    #[test]
    fn test_head_last() {
        assert_eq!(head(&[1, 2, 3]), Some(&1));
        assert_eq!(last(&[1, 2, 3]), Some(&3));
        assert_eq!(head::<i32>(&[]), None);
        assert_eq!(last::<i32>(&[]), None);
    }

    #[test]
    fn test_find_and_double() {
        assert_eq!(find_and_double(&[1, 2, 3, 4], |x| *x > 2), Some(6));
        assert_eq!(find_and_double(&[1, 2], |x| *x > 10), None);
    }

    #[test]
    fn test_nth_or_default() {
        assert_eq!(nth_or_default(&[10, 20, 30], 1, 0), 20);
        assert_eq!(nth_or_default(&[10, 20, 30], 5, 99), 99);
        assert_eq!(nth_or_default::<i32>(&[], 0, -1), -1);
    }

    #[test]
    fn test_checked_div() {
        assert_eq!(checked_div(10, 2), Ok(5));
        assert_eq!(checked_div(10, 0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_checked_sqrt() {
        assert!((checked_sqrt(4.0).unwrap() - 2.0).abs() < 1e-10);
        assert_eq!(checked_sqrt(-1.0), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_sqrt_of_division_chaining() {
        let r = sqrt_of_division(16.0, 4.0).unwrap();
        assert!((r - 2.0).abs() < 1e-10);
        assert_eq!(sqrt_of_division(16.0, 0.0), Err(MathError::DivisionByZero));
        assert_eq!(sqrt_of_division(-16.0, 1.0), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_assoc_opt() {
        let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
        assert_eq!(assoc_opt(&2, &pairs), Some(&"two"));
        assert_eq!(assoc_opt(&99, &pairs), None);
        assert_eq!(assoc_opt::<i32, &str>(&1, &[]), None);
    }
}

fn main() {
    println!("{:?}", safe_div(10.0, 2.0), Some(5.0));
    println!("{:?}", safe_div(1.0, 0.0), None);
    println!("{:?}", head(&[1, 2, 3]), Some(&1));
}
