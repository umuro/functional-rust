#![allow(dead_code)]

// Solution 1: Idiomatic Rust — generic over any Fn(T) -> T
pub fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T {
    f(f(x))
}

// Solution 2: Using a function pointer
pub fn twice_ptr<T>(f: fn(T) -> T, x: T) -> T {
    f(f(x))
}

// Solution 3: Higher-order — returns a closure (mirrors OCaml partial application)
// `let quad = twice_compose(double)` mirrors OCaml's `let quad = twice double`
pub fn twice_compose<T, F: Fn(T) -> T>(f: F) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

fn double(x: i32) -> i32 {
    2 * x
}

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    // Direct style — function + value together
    println!("twice double 3  = {}", twice(double, 3)); // 12
    println!("twice square 2  = {}", twice(square, 2)); // 16

    // Partial application style — mirrors OCaml's `let quad = twice double`
    let quad = twice_compose(double);
    let fourth = twice_compose(square);
    println!("quad 3   = {}", quad(3)); // 12
    println!("fourth 2 = {}", fourth(2)); // 16
}

/* Output:
   twice double 3  = 12
   twice square 2  = 16
   quad 3   = 12
   fourth 2 = 16
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
    fn test_twice_with_closure() {
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
        let quad = twice_compose(double);
        assert_eq!(quad(3), 12);
    }

    #[test]
    fn test_twice_compose_fourth_mirrors_ocaml() {
        let fourth = twice_compose(square);
        assert_eq!(fourth(2), 16);
    }

    #[test]
    fn test_twice_compose_closure() {
        let add_ten = twice_compose(|x: i32| x + 5);
        assert_eq!(add_ten(0), 10);
    }
}
