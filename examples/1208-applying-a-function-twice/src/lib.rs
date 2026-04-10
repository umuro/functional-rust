#![allow(dead_code)]

// Solution 1: Idiomatic Rust — generic over any Fn(T) -> T
// Applies f to x, then applies f to that result: f(f(x))
pub fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T {
    f(f(x))
}

// Solution 2: Using a function pointer (more explicit, less flexible than Fn)
// fn(T) -> T is Copy, so no special handling needed
pub fn twice_ptr<T>(f: fn(T) -> T, x: T) -> T {
    f(f(x))
}

// Solution 3: Higher-order — returns a closure (mirrors OCaml partial application)
// `let quad = twice_compose(double)` mirrors OCaml's `let quad = twice double`
pub fn twice_compose<T, F: Fn(T) -> T>(f: F) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn double(x: i32) -> i32 {
        2 * x
    }

    fn square(x: i32) -> i32 {
        x * x
    }

    #[test]
    fn test_twice_double() {
        // double(3) = 6, double(6) = 12
        assert_eq!(twice(double, 3), 12);
    }

    #[test]
    fn test_twice_square() {
        // square(2) = 4, square(4) = 16
        assert_eq!(twice(square, 2), 16);
    }

    #[test]
    fn test_twice_with_closure() {
        // increment twice: 5 + 1 + 1 = 7
        assert_eq!(twice(|x: i32| x + 1, 5), 7);
    }

    #[test]
    fn test_twice_identity() {
        assert_eq!(twice(|x: i32| x, 42), 42);
    }

    #[test]
    fn test_twice_ptr_double() {
        assert_eq!(twice_ptr(double, 3), 12);
    }

    #[test]
    fn test_twice_compose_quad_mirrors_ocaml() {
        // mirrors: let quad = twice double
        let quad = twice_compose(double);
        assert_eq!(quad(3), 12);
    }

    #[test]
    fn test_twice_compose_fourth_mirrors_ocaml() {
        // mirrors: let fourth = twice square
        let fourth = twice_compose(square);
        assert_eq!(fourth(2), 16);
    }

    #[test]
    fn test_twice_compose_closure() {
        let add_ten = twice_compose(|x: i32| x + 5);
        assert_eq!(add_ten(0), 10);
    }
}
