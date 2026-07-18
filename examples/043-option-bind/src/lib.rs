#![allow(clippy::all)]
// Option::and_then (monadic bind): sequences computations that each may fail.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn chained_div(a: i32, b: i32, c: i32) -> Option<i32> {
    safe_div(a, b).and_then(|x| safe_div(x, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_div_ok() {
        assert_eq!(safe_div(10, 2), Some(5));
    }

    #[test]
    fn test_safe_div_by_zero() {
        assert_eq!(safe_div(10, 0), None);
    }

    #[test]
    fn test_chained_div_ok() {
        assert_eq!(chained_div(100, 5, 2), Some(10));
    }

    #[test]
    fn test_chained_div_first_step_fails() {
        assert_eq!(chained_div(100, 0, 2), None);
    }

    #[test]
    fn test_chained_div_second_step_fails() {
        assert_eq!(chained_div(100, 5, 0), None);
    }
}
