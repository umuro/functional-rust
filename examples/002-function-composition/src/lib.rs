// Solution 1: Idiomatic Rust — compose as a higher-order function
// Takes two functions and returns a closure that applies them in sequence
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

// Solution 2: Using function pointers — when you need a concrete type
// More restrictive than closures but gives an explicit function type
pub fn compose_fn<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

// Helper functions matching the OCaml example
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
    fn test_compose_square_then_double_3() {
        // square(3) = 9, double(9) = 18
        let square_then_double = compose(double, square);
        assert_eq!(square_then_double(3), 18);
    }

    #[test]
    fn test_compose_square_then_double_4() {
        // square(4) = 16, double(16) = 32
        let square_then_double = compose(double, square);
        assert_eq!(square_then_double(4), 32);
    }

    #[test]
    fn test_compose_zero() {
        // square(0) = 0, double(0) = 0
        let square_then_double = compose(double, square);
        assert_eq!(square_then_double(0), 0);
    }

    #[test]
    fn test_compose_negative() {
        // square(-2) = 4, double(4) = 8
        let square_then_double = compose(double, square);
        assert_eq!(square_then_double(-2), 8);
    }

    #[test]
    fn test_compose_double_then_square() {
        // Reverse order: double first, then square
        let double_then_square = compose(square, double);
        // double(3) = 6, square(6) = 36
        assert_eq!(double_then_square(3), 36);
    }

    #[test]
    fn test_compose_with_custom_functions() {
        let add_one = |x: i32| x + 1;
        let multiply_by_3 = |x: i32| x * 3;
        // multiply_by_3 first, then add_one
        let composed = compose(add_one, multiply_by_3);
        // multiply_by_3(5) = 15, add_one(15) = 16
        assert_eq!(composed(5), 16);
    }

    #[test]
    fn test_compose_fn_pointers() {
        // Using function pointers instead of closures
        let square_then_double = compose_fn(double, square);
        assert_eq!(square_then_double(3), 18);
    }
}
