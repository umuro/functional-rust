#![allow(clippy::all)]
//! Recursive Macro Patterns
//!
//! Macros that call themselves for compile-time computation.

/// Count elements at compile time.
#[macro_export]
macro_rules! count {
    () => { 0usize };
    ($head:expr $(, $tail:expr)*) => {
        1usize + count!($($tail),*)
    };
}

/// Reverse an array at compile time using accumulator pattern.
#[macro_export]
macro_rules! reverse_list {
    // Internal: accumulator-based recursion
    (@acc [$($acc:expr),*]) => {
        [$($acc),*]
    };
    (@acc [$($acc:expr),*] $head:expr $(, $tail:expr)*) => {
        reverse_list!(@acc [$head $(, $acc)*] $($tail),*)
    };
    // Public entry point
    ($($x:expr),* $(,)?) => {
        reverse_list!(@acc [] $($x),*)
    };
}

/// Check if value equals any of the options.
#[macro_export]
macro_rules! one_of {
    ($val:expr, $single:expr) => {
        $val == $single
    };
    ($val:expr, $first:expr $(, $rest:expr)+) => {
        $val == $first || one_of!($val $(, $rest)+)
    };
}

/// Concatenate strings with separator.
#[macro_export]
macro_rules! join_with {
    ($sep:expr; $single:expr) => {
        $single.to_string()
    };
    ($sep:expr; $first:expr $(, $rest:expr)+) => {
        format!("{}{}{}", $first, $sep, join_with!($sep; $($rest),+))
    };
}

/// Sum values recursively.
#[macro_export]
macro_rules! sum_rec {
    () => { 0 };
    ($single:expr) => { $single };
    ($first:expr $(, $rest:expr)+) => {
        $first + sum_rec!($($rest),+)
    };
}

/// Maximum of values recursively.
#[macro_export]
macro_rules! max_rec {
    ($single:expr) => { $single };
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
    ($first:expr $(, $rest:expr)+) => {
        {
            let rest_max = max_rec!($($rest),+);
            if $first > rest_max { $first } else { rest_max }
        }
    };
}

/// Generate nested tuples (cons-list style).
#[macro_export]
macro_rules! nested_tuple {
    ($single:expr) => { $single };
    ($head:expr, $($tail:expr),+) => {
        ($head, nested_tuple!($($tail),+))
    };
}

/// Power of 2 at compile time (for small n).
#[macro_export]
macro_rules! pow2 {
    (0) => {
        1usize
    };
    (1) => {
        2usize
    };
    (2) => {
        4usize
    };
    (3) => {
        8usize
    };
    (4) => {
        16usize
    };
    (5) => {
        32usize
    };
    (6) => {
        64usize
    };
    (7) => {
        128usize
    };
    (8) => {
        256usize
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_empty() {
        assert_eq!(count!(), 0);
    }

    #[test]
    fn test_count_single() {
        assert_eq!(count!(42), 1);
    }

    #[test]
    fn test_count_multiple() {
        assert_eq!(count!(1, 2, 3, 4, 5), 5);
    }

    #[test]
    fn test_reverse_list() {
        let rev = reverse_list![1, 2, 3, 4, 5];
        assert_eq!(rev, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_empty() {
        let rev: [i32; 0] = reverse_list![];
        assert_eq!(rev, []);
    }

    #[test]
    fn test_one_of_true() {
        assert!(one_of!(3, 1, 2, 3, 4, 5));
    }

    #[test]
    fn test_one_of_false() {
        assert!(!one_of!(10, 1, 2, 3, 4, 5));
    }

    #[test]
    fn test_one_of_single() {
        assert!(one_of!(42, 42));
        assert!(!one_of!(42, 0));
    }

    #[test]
    fn test_join_with() {
        assert_eq!(join_with!(", "; "a", "b", "c"), "a, b, c");
        assert_eq!(join_with!("-"; "x"), "x");
    }

    #[test]
    fn test_sum_rec() {
        assert_eq!(sum_rec!(), 0);
        assert_eq!(sum_rec!(5), 5);
        assert_eq!(sum_rec!(1, 2, 3, 4), 10);
    }

    #[test]
    fn test_max_rec() {
        assert_eq!(max_rec!(42), 42);
        assert_eq!(max_rec!(3, 7), 7);
        assert_eq!(max_rec!(5, 2, 9, 1, 6), 9);
    }

    #[test]
    fn test_nested_tuple() {
        let t = nested_tuple!(1, 2, 3);
        assert_eq!(t, (1, (2, 3)));
    }

    #[test]
    fn test_pow2() {
        assert_eq!(pow2!(0), 1);
        assert_eq!(pow2!(3), 8);
        assert_eq!(pow2!(8), 256);
    }
}
