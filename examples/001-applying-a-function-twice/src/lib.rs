/// Apply a function twice to a value.
/// Demonstrates higher-order functions and partial application.
///
/// Takes a function `f` and a value `x`, applies `f` to `x`, then applies `f` again.
pub fn twice<T>(f: impl Fn(T) -> T, x: T) -> T {
    f(f(x))
}

/// Alternative using function pointers (more explicit type).
/// Useful when you need to pass function pointers directly.
pub fn twice_fn<T: Copy>(f: fn(T) -> T, x: T) -> T {
    f(f(x))
}

/// Functional approach: compose a function with itself.
/// Returns a closure that applies the original function twice.
pub fn twice_compose<T: 'static>(f: impl Fn(T) -> T + 'static) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

// Helper functions for examples
pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn square(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twice_double_zero() {
        assert_eq!(twice(double, 0), 0);
    }

    #[test]
    fn test_twice_double_positive() {
        // double(3) = 6, double(6) = 12
        assert_eq!(twice(double, 3), 12);
    }

    #[test]
    fn test_twice_square_small() {
        // square(2) = 4, square(4) = 16
        assert_eq!(twice(square, 2), 16);
    }

    #[test]
    fn test_twice_square_one() {
        assert_eq!(twice(square, 1), 1);
    }

    #[test]
    fn test_twice_fn_double() {
        assert_eq!(twice_fn(double, 3), 12);
    }

    #[test]
    fn test_twice_fn_square() {
        assert_eq!(twice_fn(square, 2), 16);
    }

    #[test]
    fn test_twice_compose_double() {
        let quad = twice_compose(double);
        assert_eq!(quad(3), 12);
    }

    #[test]
    fn test_twice_compose_square() {
        let fourth = twice_compose(square);
        assert_eq!(fourth(2), 16);
    }

    #[test]
    fn test_twice_with_closure() {
        let add_five = |x| x + 5;
        assert_eq!(twice(add_five, 0), 10);
    }

    #[test]
    fn test_twice_negative() {
        assert_eq!(twice(double, -3), -12);
    }
}
