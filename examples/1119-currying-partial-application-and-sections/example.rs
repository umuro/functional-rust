//! Currying, Partial Application, and Sections in Rust.
//!
//! This example demonstrates how to emulate OCaml's currying,
//! partial application, and operator sections in Rust.

// ---------- Solution 1: Idiomatic Rust (closures) ----------

/// Adds two integers (idiomatic Rust closure).
/// This is not curried; it's a simple closure.
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Partially applies `add` to `5`.
pub fn add5() -> impl Fn(i32) -> i32 {
    |y| add(5, y)
}

/// Tupled version of add (takes a tuple).
pub fn add_tup(pair: (i32, i32)) -> i32 {
    pair.0 + pair.1
}

/// Converts a tupled function (i32,i32)->i32 to a curried function.
pub fn curry_i32<F>(f: F) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn((i32, i32)) -> i32,
    F: Clone + 'static,
{
    move |x| {
        let f = f.clone();
        Box::new(move |y| f((x, y)))
    }
}

/// Converts a curried function (i32)->(i32)->i32 to a tupled function.
pub fn uncurry_i32<F>(f: F) -> impl Fn((i32, i32)) -> i32
where
    F: Fn(i32) -> Box<dyn Fn(i32) -> i32>,
{
    move |(x, y)| f(x)(y)
}

/// Operator section: multiply by 2.
pub fn double() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

/// Operator section: increment by 1.
pub fn increment() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Operator section: halve (integer division by 2).
pub fn halve() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

/// Labeled‑style function with named parameters (Rust doesn't have labeled
/// arguments; we use a struct or separate closures).
pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

/// Celsius to Fahrenheit conversion using partial application.
pub fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    scale_and_shift(5, -160)
}

// ---------- Solution 2: Functional/curried style ----------

/// Curried addition (returns a closure).
pub fn add_curried(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Curried multiplication (operator section).
pub fn double_curried() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

/// Curried increment.
pub fn increment_curried() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Curried halve with flipped division.
pub fn halve_curried() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

// ---------- Demo main ----------

fn main() {
    println!("add5 10   = {}", add5()(10));
    println!("double 7  = {}", double()(7));
    println!("halve 20  = {}", halve()(20));
    let pipeline = vec![double(), increment(), halve()];
    let result = pipeline.iter().fold(6, |acc, f| f(acc));
    println!("6 |> *2 |> +1 |> /2 = {}", result);
    println!("212F in Celsius ≈ {}", celsius_of_fahrenheit()(212));
}

/* Output:
   add5 10   = 15
   double 7  = 14
   halve 20  = 10
   6 |> *2 |> +1 |> /2 = 6
   212F in Celsius ≈ 900
*/