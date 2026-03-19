#![allow(clippy::all)]
//! Function Pointers vs Closures
//!
//! Comparing fn pointers and closures: size, capabilities, use cases.

/// Named functions — can be used as fn pointers.
pub fn square(x: i32) -> i32 {
    x * x
}
pub fn cube(x: i32) -> i32 {
    x * x * x
}
pub fn double(x: i32) -> i32 {
    x * 2
}

/// Accepts fn pointer — only non-capturing callables.
pub fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// Accepts any Fn — works with both fn ptrs and closures.
pub fn apply_generic<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

/// Table using fn pointers.
pub fn math_ops() -> Vec<(&'static str, fn(i32) -> i32)> {
    vec![
        ("square", square),
        ("cube", cube),
        ("double", double),
        ("negate", |x| -x),
    ]
}

/// Size comparison between fn pointer and closure.
pub fn size_comparison() -> (usize, usize, usize) {
    let fn_ptr_size = std::mem::size_of::<fn(i32) -> i32>();
    let non_capturing = std::mem::size_of_val(&|x: i32| x * 2);
    let y = 42i32;
    let capturing = std::mem::size_of_val(&move |x: i32| x + y);
    (fn_ptr_size, non_capturing, capturing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_ptr_with_named() {
        assert_eq!(apply_fn_ptr(square, 5), 25);
        assert_eq!(apply_fn_ptr(cube, 3), 27);
        assert_eq!(apply_fn_ptr(double, 7), 14);
    }

    #[test]
    fn test_fn_ptr_with_closure() {
        assert_eq!(apply_fn_ptr(|x| x + 1, 5), 6);
        assert_eq!(apply_fn_ptr(|x| -x, 5), -5);
    }

    #[test]
    fn test_generic_with_both() {
        assert_eq!(apply_generic(square, 5), 25);
        assert_eq!(apply_generic(|x| x + 10, 5), 15);

        let offset = 100;
        assert_eq!(apply_generic(|x| x + offset, 5), 105);
    }

    #[test]
    fn test_math_ops_table() {
        let ops = math_ops();
        assert_eq!(ops[0].1(5), 25); // square
        assert_eq!(ops[1].1(3), 27); // cube
        assert_eq!(ops[2].1(7), 14); // double
        assert_eq!(ops[3].1(5), -5); // negate
    }

    #[test]
    fn test_size_comparison() {
        let (fn_size, non_cap, cap) = size_comparison();
        assert_eq!(fn_size, std::mem::size_of::<usize>());
        assert_eq!(non_cap, 0); // non-capturing is zero-sized
        assert!(cap > 0); // capturing holds data
    }
}
