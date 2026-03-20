#![allow(clippy::all)]
//! Closure Type Inference
//!
//! How Rust infers closure types and when annotations are needed.

/// Apply a function to a value.
pub fn apply<F, T, U>(f: F, x: T) -> U
where
    F: Fn(T) -> U,
{
    f(x)
}

/// Demonstrates type inference in closures.
pub fn inference_demo() -> Vec<i32> {
    // Type inferred from first use
    let double = |x| x * 2;
    let _ = double(5i32); // fixes type as i32

    // Explicit input type, inferred return
    let square = |x: i32| x * x;

    // Inferred from context
    let nums = vec![1, 2, 3, 4, 5];
    nums.iter().map(|&x| square(double(x))).collect()
}

/// When type context is needed.
pub fn needs_annotation<T: std::ops::Add<Output = T> + Copy>(x: T, y: T) -> T {
    let add = |a: T, b: T| a + b;
    add(x, y)
}

/// Multiple uses must be consistent.
pub fn consistent_types() {
    let process = |x| x + 1;
    let _: i32 = process(5);
    // process(5.0); // ERROR: already fixed as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_inference() {
        let double = |x| x * 2;
        assert_eq!(double(5), 10);
    }

    #[test]
    fn test_explicit_input() {
        let square = |x: i32| x * x;
        assert_eq!(square(4), 16);
    }

    #[test]
    fn test_inference_demo() {
        let result = inference_demo();
        // (1*2)^2, (2*2)^2, (3*2)^2, (4*2)^2, (5*2)^2
        assert_eq!(result, vec![4, 16, 36, 64, 100]);
    }

    #[test]
    fn test_apply_generic() {
        assert_eq!(apply(|x: i32| x + 1, 5), 6);
        assert_eq!(apply(|s: &str| s.len(), "hello"), 5);
    }

    #[test]
    fn test_needs_annotation() {
        assert_eq!(needs_annotation(3i32, 4i32), 7);
        assert_eq!(needs_annotation(3.0f64, 4.0f64), 7.0);
    }

    #[test]
    fn test_iterator_context() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_closure_in_struct() {
        struct Holder<F> {
            f: F,
        }

        let h = Holder { f: |x: i32| x + 1 };
        assert_eq!((h.f)(5), 6);
    }
}
