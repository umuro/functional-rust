// Pure functions for integers
pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Pure functions for strings
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn add_exclaim(s: &str) -> String {
    format!("{}!", s)
}

// Idiomatic Rust: direct function composition
pub fn compute_result_idiomatic() -> i32 {
    add_one(double(5))  // 11
}

pub fn compute_greeting_idiomatic() -> String {
    add_exclaim(&shout("hello"))  // "HELLO!"
}

// Functional Rust: pipe function (mimics OCaml's |>)
pub fn pipe<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    f(value)
}

pub fn compute_result_with_pipe() -> i32 {
    pipe(pipe(5, double), add_one)  // 11
}

pub fn compute_greeting_with_pipe() -> String {
    let shouted = pipe("hello", shout);
    pipe(&shouted, |s| add_exclaim(s))  // "HELLO!"
}

// Composition function
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

pub fn compute_result_with_composition() -> i32 {
    let double_then_add_one = compose(double, add_one);
    double_then_add_one(5)  // 11
}

fn main() {
    println!("=== Pipeline Operator Demo ===\n");

    println!("Idiomatic Rust (nested function calls):");
    println!("  compute_result_idiomatic() = {}", compute_result_idiomatic());
    println!("  compute_greeting_idiomatic() = {}\n", compute_greeting_idiomatic());

    println!("Functional Rust (using pipe function):");
    println!("  compute_result_with_pipe() = {}", compute_result_with_pipe());
    println!("  compute_greeting_with_pipe() = {}\n", compute_greeting_with_pipe());

    println!("Function Composition:");
    println!("  compute_result_with_composition() = {}", compute_result_with_composition());
}

/* Output:
=== Pipeline Operator Demo ===

Idiomatic Rust (nested function calls):
  compute_result_idiomatic() = 11
  compute_greeting_idiomatic() = HELLO!

Functional Rust (using pipe function):
  compute_result_with_pipe() = 11
  compute_greeting_with_pipe() = HELLO!

Function Composition:
  compute_result_with_composition() = 11
*/
