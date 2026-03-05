/// 746: Documentation Tests — rustdoc examples that are compiled and run

/// Clamps `x` to the inclusive range `[lo, hi]`.
///
/// # Examples
///
/// ```
/// # use example::clamp;
/// assert_eq!(clamp(0, 10, -5), 0);
/// assert_eq!(clamp(0, 10,  5), 5);
/// assert_eq!(clamp(0, 10, 15), 10);
/// // Boundaries are inclusive
/// assert_eq!(clamp(0, 10,  0), 0);
/// assert_eq!(clamp(0, 10, 10), 10);
/// ```
pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}

/// Repeats `s` exactly `n` times.
///
/// # Examples
///
/// ```
/// # use example::repeat;
/// assert_eq!(repeat("ab", 3), "ababab");
/// assert_eq!(repeat("x",  0), "");
/// assert_eq!(repeat("",   5), "");
/// ```
pub fn repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Splits `s` on the first occurrence of `delim`.
///
/// Returns `None` if `delim` is not found.
///
/// # Examples
///
/// ```
/// # use example::split_once_char;
/// assert_eq!(split_once_char("key:value", ':'), Some(("key", "value")));
/// assert_eq!(split_once_char("no-delim",  ':'), None);
/// assert_eq!(split_once_char("a:b:c",     ':'), Some(("a", "b:c")));
/// ```
pub fn split_once_char(s: &str, delim: char) -> Option<(&str, &str)> {
    s.split_once(delim)
}

/// Returns `Err` if divisor is zero.
///
/// # Errors
///
/// Returns `Err("division by zero")` when `b == 0`.
///
/// # Examples
///
/// ```
/// # use example::safe_div;
/// assert_eq!(safe_div(10, 2),  Ok(5));
/// assert_eq!(safe_div(10, 0),  Err("division by zero"));
/// assert_eq!(safe_div(-9, 3),  Ok(-3));
/// ```
pub fn safe_div(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 { Err("division by zero") } else { Ok(a / b) }
}

/// Panics on invalid input — shown in doc test with `should_panic`.
///
/// # Panics
///
/// Panics if `n` is zero.
///
/// ```should_panic
/// example::factorial(0); // panics!
/// ```
///
/// # Examples
///
/// ```
/// # use example::factorial;
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    if n == 0 { panic!("factorial(0) is undefined in this implementation") }
    (1..=n).product()
}

fn main() {
    println!("clamp(0,10,-5) = {}", clamp(0, 10, -5));
    println!("repeat(\"hi\",3) = {}", repeat("hi", 3));
    println!("split_once_char(\"k:v\",':') = {:?}", split_once_char("k:v", ':'));
    println!("safe_div(10,2) = {:?}", safe_div(10, 2));
    println!("safe_div(10,0) = {:?}", safe_div(10, 0));
    println!("factorial(5) = {}", factorial(5));
}

// Regular unit tests complement doc tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_edge_cases() {
        assert_eq!(clamp(i32::MIN, i32::MAX, 0), 0);
        assert_eq!(clamp(5, 5, 100), 5);  // lo == hi
    }

    #[test]
    fn repeat_unicode() {
        assert_eq!(repeat("🦀", 3), "🦀🦀🦀");
    }

    #[test]
    fn safe_div_negative() {
        assert_eq!(safe_div(-10, -2), Ok(5));
    }
}
