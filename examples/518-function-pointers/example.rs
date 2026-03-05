//! # 518. Function Pointers vs Closures
//! Comparing fn pointers and closures: size, capabilities, use cases.

/// Named functions — can be used as fn pointers
fn square(x: i32) -> i32 { x * x }
fn cube(x: i32) -> i32 { x * x * x }
fn double(x: i32) -> i32 { x * 2 }

/// Accepts fn pointer — only non-capturing callables
fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }

/// Accepts any Fn — works with both fn ptrs and closures
fn apply_generic<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }

/// Table using fn pointers (array — constant size, stack allocated)
// fn pointer arrays can be indexed at compile time
type MathEntry = (&'static str, fn(i32) -> i32);

fn math_ops() -> Vec<(&'static str, fn(i32) -> i32)> {
    vec![
        ("square", square),
        ("cube",   cube),
        ("double", double),
        ("negate", |x| -x), // non-capturing closure coerces to fn ptr
    ]
}

/// Size comparison
fn show_sizes() {
    println!("Size of fn(i32)->i32: {} bytes", std::mem::size_of::<fn(i32) -> i32>());

    // Non-capturing closure: zero size (or same as fn ptr — compiler-dependent)
    let nc_closure = |x: i32| x + 1;
    println!("Non-capturing closure size: {} bytes", std::mem::size_of_val(&nc_closure));

    // Capturing closure: contains captured value
    let offset = 42i32;
    let cap_closure = move |x: i32| x + offset;
    println!("Capturing closure (i32) size: {} bytes", std::mem::size_of_val(&cap_closure));

    let big_data = vec![0u8; 100];
    let cap_big = move |_: i32| big_data.len() as i32;
    println!("Capturing closure (Vec) size: {} bytes", std::mem::size_of_val(&cap_big));
}

fn main() {
    println!("=== Function pointer table ===");
    for (name, f) in math_ops() {
        println!("{}(5) = {}", name, f(5));
    }

    println!("\n=== fn ptr is Copy ===");
    let f: fn(i32) -> i32 = square;
    let g = f; // copied — both usable
    println!("f(3) = {}, g(3) = {}", f(3), g(3));

    println!("\n=== apply_fn_ptr vs apply_generic ===");
    println!("fn ptr: {}", apply_fn_ptr(double, 7));
    println!("generic (fn ptr): {}", apply_generic(double, 7));
    // Closure captures — only works with generic:
    let offset = 100;
    let add_offset = move |x: i32| x + offset;
    // apply_fn_ptr(add_offset, 5) // ERROR: captures
    println!("generic (closure): {}", apply_generic(add_offset, 7));

    println!("\n=== Size comparison ===");
    show_sizes();

    println!("\n=== Dynamic dispatch ===");
    // Vec of fn pointers — all same type, no boxing needed
    let ops: Vec<fn(i32) -> i32> = vec![square, cube, double];
    for (i, op) in ops.iter().enumerate() {
        println!("  ops[{}](4) = {}", i, op(4));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_ptr_basic() {
        let f: fn(i32) -> i32 = square;
        assert_eq!(f(4), 16);
        assert_eq!(apply_fn_ptr(f, 3), 9);
    }

    #[test]
    fn test_fn_ptr_is_copy() {
        let f: fn(i32) -> i32 = cube;
        let g = f; // Copy
        assert_eq!(f(2), g(2));
    }

    #[test]
    fn test_generic_accepts_both() {
        // fn pointer
        assert_eq!(apply_generic(square, 5), 25);
        // closure
        let n = 10;
        assert_eq!(apply_generic(move |x| x + n, 5), 15);
    }

    #[test]
    fn test_math_table() {
        let ops = math_ops();
        let sq = ops.iter().find(|(name, _)| *name == "square").unwrap().1;
        assert_eq!(sq(6), 36);
    }
}
