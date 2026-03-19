//! macro_rules! Basics
//!
//! Declarative macros for code generation at compile time.

/// Simple assertion macro with custom failure message.
#[macro_export]
macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!(
                "check_eq failed: {:?} != {:?} at {}:{}",
                $left,
                $right,
                file!(),
                line!()
            );
        }
    };
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            panic!(
                "check_eq failed ({}): {:?} != {:?} at {}:{}",
                $msg,
                $left,
                $right,
                file!(),
                line!()
            );
        }
    };
}

/// Repeat a block n times.
#[macro_export]
macro_rules! repeat {
    ($n:expr, $body:block) => {
        for _ in 0..$n $body
    };
}

/// Minimum of multiple values.
#[macro_export]
macro_rules! min_of {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {
        {
            let first = $a;
            let rest_min = min_of!($($rest),+);
            if first < rest_min { first } else { rest_min }
        }
    };
}

/// Maximum of multiple values.
#[macro_export]
macro_rules! max_of {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {
        {
            let first = $a;
            let rest_max = max_of!($($rest),+);
            if first > rest_max { first } else { rest_max }
        }
    };
}

/// HashMap literal macro.
#[macro_export]
macro_rules! map {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(m.insert($k, $v);)+
            m
        }
    };
}

/// Vec literal with transformation.
#[macro_export]
macro_rules! vec_map {
    ($transform:expr; $($item:expr),* $(,)?) => {
        {
            let f = $transform;
            vec![$(f($item)),*]
        }
    };
}

/// Simple timing macro for benchmarking.
#[macro_export]
macro_rules! time_it {
    ($label:expr, $body:expr) => {{
        let start = ::std::time::Instant::now();
        let result = $body;
        let elapsed = start.elapsed();
        println!("{}: {:?}", $label, elapsed);
        result
    }};
}

/// Match guard with default.
#[macro_export]
macro_rules! with_default {
    ($opt:expr, $default:expr) => {
        match $opt {
            Some(v) => v,
            None => $default,
        }
    };
}

// Public functions to demonstrate macro usage

/// Demonstrates min_of macro.
pub fn find_minimum(values: &[i32]) -> i32 {
    match values {
        [] => panic!("empty slice"),
        [a] => *a,
        [a, b] => min_of!(*a, *b),
        [a, b, c] => min_of!(*a, *b, *c),
        _ => *values.iter().min().unwrap(),
    }
}

/// Demonstrates max_of macro.
pub fn find_maximum(values: &[i32]) -> i32 {
    match values {
        [] => panic!("empty slice"),
        [a] => *a,
        [a, b] => max_of!(*a, *b),
        [a, b, c] => max_of!(*a, *b, *c),
        _ => *values.iter().max().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_eq_pass() {
        check_eq!(2 + 2, 4);
        check_eq!("hello".len(), 5, "string length");
    }

    #[test]
    #[should_panic(expected = "check_eq failed")]
    fn test_check_eq_fail() {
        check_eq!(1, 2);
    }

    #[test]
    fn test_min_of_single() {
        assert_eq!(min_of!(42), 42);
    }

    #[test]
    fn test_min_of_multiple() {
        assert_eq!(min_of!(5, 3, 7, 1, 9), 1);
        assert_eq!(min_of!(10, 20), 10);
        assert_eq!(min_of!(1, 2, 3), 1);
    }

    #[test]
    fn test_max_of_single() {
        assert_eq!(max_of!(42), 42);
    }

    #[test]
    fn test_max_of_multiple() {
        assert_eq!(max_of!(5, 3, 7, 1, 9), 9);
        assert_eq!(max_of!(10, 20), 20);
        assert_eq!(max_of!(1, 2, 3), 3);
    }

    #[test]
    fn test_map_macro_empty() {
        let m: std::collections::HashMap<&str, i32> = map!();
        assert!(m.is_empty());
    }

    #[test]
    fn test_map_macro_entries() {
        let m = map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
        };
        assert_eq!(m["one"], 1);
        assert_eq!(m["two"], 2);
        assert_eq!(m["three"], 3);
    }

    #[test]
    fn test_vec_map() {
        let v = vec_map!(|x| x * 2; 1, 2, 3);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_with_default() {
        let some_val: Option<i32> = Some(42);
        let none_val: Option<i32> = None;

        assert_eq!(with_default!(some_val, 0), 42);
        assert_eq!(with_default!(none_val, 0), 0);
    }

    #[test]
    fn test_repeat() {
        let mut count = 0;
        repeat!(5, {
            count += 1;
        });
        assert_eq!(count, 5);
    }

    #[test]
    fn test_find_minimum() {
        assert_eq!(find_minimum(&[5, 3, 8]), 3);
        assert_eq!(find_minimum(&[10]), 10);
    }

    #[test]
    fn test_find_maximum() {
        assert_eq!(find_maximum(&[5, 3, 8]), 8);
        assert_eq!(find_maximum(&[10]), 10);
    }
}
