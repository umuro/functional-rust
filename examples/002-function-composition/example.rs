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

fn main() {
    let square_then_double = compose(double, square);
    println!("square_then_double 3 = {}", square_then_double(3));
    println!("square_then_double 4 = {}", square_then_double(4));

    let double_then_square = compose(square, double);
    println!("double_then_square 3 = {}", double_then_square(3));
}

/* Output:
   square_then_double 3 = 18
   square_then_double 4 = 32
   double_then_square 3 = 36
*/
