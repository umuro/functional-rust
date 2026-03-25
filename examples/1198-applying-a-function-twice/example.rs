#![allow(dead_code)]

/// Idiomatic Rust: applies a function twice using a generic closure.
pub fn twice<T, F>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(f(x))
}

/// Curried form: returns a closure that applies `f` twice to its argument.
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

fn main() {
    // Direct application
    println!("twice double 3   = {}", twice(double, 3)); // 12
    println!("twice square 2   = {}", twice(square, 2)); // 16

    // Partial application — mirrors OCaml's `let quad = twice double`
    let quad = twice_curried(double);
    let fourth = twice_curried(square);
    println!("quad 3           = {}", quad(3)); // 12
    println!("fourth 2         = {}", fourth(2)); // 16

    // With an anonymous closure
    let add_ten = |x: i32| x + 10;
    println!("twice (+10) 5    = {}", twice(add_ten, 5)); // 25
}

/* Output:
   twice double 3   = 12
   twice square 2   = 16
   quad 3           = 12
   fourth 2         = 16
   twice (+10) 5    = 25
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twice_double() {
        assert_eq!(twice(double, 3), 12);
    }

    #[test]
    fn test_twice_square() {
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
        let add_ten = |x: i32| x + 10;
        assert_eq!(twice(add_ten, 5), 25);
    }

    #[test]
    fn test_twice_identity() {
        assert_eq!(twice(|x: i32| x, 42), 42);
    }

    #[test]
    fn test_twice_zero() {
        assert_eq!(twice(double, 0), 0);
        assert_eq!(twice(square, 0), 0);
    }
}
