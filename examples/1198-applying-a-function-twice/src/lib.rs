#![allow(dead_code)]

/// Idiomatic Rust: applies a function twice using a generic closure.
/// Takes `F: Fn(T) -> T` — the function must map T to T (same type in and out).
pub fn twice<T, F>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(f(x))
}

/// Curried form: returns a closure that applies `f` twice to its argument.
/// This mirrors the OCaml `let quad = twice double` partial application pattern.
pub fn twice_curried<T, F>(f: F) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
{
    move |x| f(f(x))
}

pub fn double(x: i64) -> i64 {
    2 * x
}

pub fn square(x: i64) -> i64 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twice_double() {
        // twice double 3 = double(double(3)) = double(6) = 12
        assert_eq!(twice(double, 3), 12);
    }

    #[test]
    fn test_twice_square() {
        // twice square 2 = square(square(2)) = square(4) = 16
        assert_eq!(twice(square, 2), 16);
    }

    #[test]
    fn test_twice_curried_partial_application() {
        let quad = twice_curried(double);
        let fourth = twice_curried(square);
        assert_eq!(quad(3), 12);
        assert_eq!(fourth(2), 16);
    }

    #[test]
    fn test_twice_with_closure() {
        // Applying an anonymous closure twice
        let add_ten = |x: i32| x + 10;
        assert_eq!(twice(add_ten, 5), 25);
    }

    #[test]
    fn test_twice_identity() {
        // Applying identity twice is still identity
        assert_eq!(twice(|x: i32| x, 42), 42);
    }

    #[test]
    fn test_twice_zero() {
        assert_eq!(twice(double, 0), 0);
        assert_eq!(twice(square, 0), 0);
    }
}
