#![allow(clippy::all)]
//! Lifetimes in Closures
//!
//! Captured references constrain closure lifetimes.

/// Closure capturing reference — bounded by capture lifetime.
pub fn make_prefixer<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a {
    move |s| format!("{}{}", prefix, s)
}

/// Closure capturing computed value (no lifetime needed).
pub fn make_sum_adder(data: &[i32]) -> impl Fn(i32) -> i32 {
    let sum: i32 = data.iter().sum(); // compute before capture
    move |x| x + sum
}

/// Closure with explicit lifetime bound.
pub fn make_checker<'a>(valid: &'a [&str]) -> impl Fn(&str) -> bool + 'a {
    move |s| valid.contains(&s)
}

/// FnMut closure with state.
pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// Returning closure that captures local — needs boxing.
pub fn make_formatter(width: usize) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |s| format!("{:>width$}", s, width = width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_prefixer() {
        let prefix = String::from("Hello, ");
        let greet = make_prefixer(&prefix);
        assert_eq!(greet("World"), "Hello, World");
    }

    #[test]
    fn test_make_sum_adder() {
        let data = vec![1, 2, 3, 4, 5];
        let adder = make_sum_adder(&data);
        assert_eq!(adder(10), 25); // 15 + 10
    }

    #[test]
    fn test_make_checker() {
        let valid = ["a", "b", "c"];
        let check = make_checker(&valid);
        assert!(check("a"));
        assert!(!check("d"));
    }

    #[test]
    fn test_make_counter() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_make_formatter() {
        let fmt = make_formatter(10);
        assert_eq!(fmt("hi"), "        hi");
    }
}
