// # Currying and Partial Application
//
// OCaml: `let add x y = x + y` — all functions are automatically curried.
// Rust has no auto-currying, but closures make partial application natural.

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — closures for partial application
// ---------------------------------------------------------------------------

/// A plain two-argument function — Rust's default style.
fn add(x: i64, y: i64) -> i64 {
    x + y
}

/// Returns a closure that adds `x` to its argument.
fn add_partial(x: i64) -> impl Fn(i64) -> i64 {
    move |y| x + y
}

// ---------------------------------------------------------------------------
// Solution 2: Curried style — mirrors OCaml's `let add x y = x + y`
// ---------------------------------------------------------------------------

/// Fully curried: each argument returns a closure expecting the next.
fn add_curried(x: i64) -> impl Fn(i64) -> i64 {
    move |y| x + y
}

/// A generic curried add for any type supporting `Add`.
fn add_curried_generic<T>(x: T) -> impl Fn(T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    move |y| x + y
}

// ---------------------------------------------------------------------------
// Solution 3: Higher-order — curry any two-argument function
// ---------------------------------------------------------------------------

/// Transforms a two-argument function into a curried chain.
fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Copy + 'static,
    B: 'static,
    C: 'static,
    F: Fn(A, B) -> C + Copy + 'static,
{
    move |a: A| Box::new(move |b: B| f(a, b))
}

fn main() {
    // Direct call
    println!("add(3, 4) = {}", add(3, 4));

    // Partial application via closure
    let add5 = add_partial(5);
    println!("add_partial(5)(3) = {}", add5(3));

    // Curried style — like OCaml's `add 10 1`
    let add10 = add_curried(10);
    println!("add_curried(10)(1) = {}", add10(1));

    // One-shot curried call
    println!("add_curried(2)(3) = {}", add_curried(2)(3));

    // Generic currying works with f64 too
    let add_half = add_curried_generic(0.5_f64);
    println!("add_curried_generic(0.5)(1.5) = {}", add_half(1.5));

    // The curry combinator applied to any two-arg function
    let curried_add = curry(add);
    let add7 = curried_add(7);
    println!("curry(add)(7)(3) = {}", add7(3));

    let mul = |a: i64, b: i64| a * b;
    let curried_mul = curry(mul);
    let double = curried_mul(2);
    println!("curry(mul)(2)(5) = {}", double(5));
}

/* Output:
   add(3, 4) = 7
   add_partial(5)(3) = 8
   add_curried(10)(1) = 11
   add_curried(2)(3) = 5
   add_curried_generic(0.5)(1.5) = 2
   curry(add)(7)(3) = 10
   curry(mul)(2)(5) = 10
*/
