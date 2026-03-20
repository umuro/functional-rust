#![allow(clippy::all)]
//! Counting Patterns in Macros
//!
//! Techniques for counting macro arguments.

/// Count using recursion.
#[macro_export]
macro_rules! count_recursive {
    () => { 0usize };
    ($head:tt $($tail:tt)*) => { 1 + count_recursive!($($tail)*) };
}

/// Count using array trick.
#[macro_export]
macro_rules! count_array {
    (@single $_:tt) => { () };
    ($($x:tt)*) => {
        <[()]>::len(&[$(count_array!(@single $x)),*])
    };
}

/// Count expressions.
#[macro_export]
macro_rules! count_exprs {
    () => { 0 };
    ($e:expr) => { 1 };
    ($e:expr, $($rest:expr),+) => { 1 + count_exprs!($($rest),+) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_recursive_empty() {
        assert_eq!(count_recursive!(), 0);
    }

    #[test]
    fn test_count_recursive() {
        assert_eq!(count_recursive!(a b c d e), 5);
    }

    #[test]
    fn test_count_array_empty() {
        assert_eq!(count_array!(), 0);
    }

    #[test]
    fn test_count_array() {
        assert_eq!(count_array!(1 2 3), 3);
    }

    #[test]
    fn test_count_exprs() {
        assert_eq!(count_exprs!(1, 2, 3, 4), 4);
    }

    #[test]
    fn test_count_exprs_single() {
        assert_eq!(count_exprs!(42), 1);
    }
}
