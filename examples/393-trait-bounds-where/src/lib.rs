//! Trait Bounds and Where Clauses

use std::fmt::{Debug, Display};
use std::hash::Hash;

pub fn print_debug<T: Debug>(val: T) {
    println!("{:?}", val);
}
pub fn compare_and_display<T: PartialOrd + Display>(a: T, b: T) -> String {
    if a < b {
        format!("{} < {}", a, b)
    } else {
        format!("{} >= {}", a, b)
    }
}

pub fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Debug + Clone,
    U: Display + Hash,
{
    format!("{:?} and {}", t, u)
}

pub fn longest_with_debug<'a, T>(a: &'a T, b: &'a T) -> &'a T
where
    T: PartialOrd + Debug,
{
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert!(compare_and_display(1, 2).contains("<"));
    }
    #[test]
    fn test_complex() {
        assert!(complex_function(42, "hello").contains("42"));
    }
    #[test]
    fn test_longest() {
        assert_eq!(longest_with_debug(&5, &3), &5);
    }
    #[test]
    fn test_compare_eq() {
        assert!(compare_and_display(2, 2).contains(">="));
    }
}
