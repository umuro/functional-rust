//! # 517. Closure-to-fn-pointer Coercion
//! Non-capturing closures coerce to fn pointers; capturing ones cannot.

/// Accept a function pointer explicitly
fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// Named functions are fn pointers
fn double(x: i32) -> i32 { x * 2 }
fn triple(x: i32) -> i32 { x * 3 }
fn negate(x: i32) -> i32 { -x  }

/// Array of function pointers (all same size — no fat pointers)
fn make_transform_table() -> [fn(i32) -> i32; 4] {
    [
        double,
        triple,
        negate,
        |x| x + 10, // non-capturing closure coerces to fn ptr
    ]
}

/// Function that stores fn pointers in a Vec
fn build_pipeline(ops: Vec<fn(i32) -> i32>) -> impl Fn(i32) -> i32 {
    move |x| ops.iter().fold(x, |acc, f| f(acc))
}

/// Demonstrate what CAN and CANNOT be coerced
fn coercion_examples() {
    // ✅ Non-capturing: coerces to fn(i32) -> i32
    let f1: fn(i32) -> i32 = |x| x + 1;
    println!("fn ptr: f1(5) = {}", f1(5));

    // ✅ Named function — is a fn pointer
    let f2: fn(i32) -> i32 = double;
    println!("named fn: f2(5) = {}", f2(5));

    // ✅ Stored in array (all same size — 1 word each)
    let table: [fn(i32) -> i32; 3] = [double, triple, |x| x * x];
    for (i, f) in table.iter().enumerate() {
        println!("  table[{}](4) = {}", i, f(4));
    }

    // ❌ Capturing closure CANNOT coerce to fn ptr
    let offset = 42;
    // This would NOT compile: let f3: fn(i32) -> i32 = |x| x + offset;
    // Must use impl Fn or Box<dyn Fn>:
    let f3: Box<dyn Fn(i32) -> i32> = Box::new(move |x| x + offset);
    println!("capturing (Box<dyn Fn>): f3(0) = {}", f3(0));
}

/// FFI-style: C functions expect fn pointers
/// (simulated — not actual FFI here)
type Callback = fn(i32) -> i32;

fn register_callback(cb: Callback) -> Callback {
    cb // store/return it like a C callback
}

fn main() {
    println!("=== Named fn pointers ===");
    println!("apply_fn_ptr(double, 5) = {}", apply_fn_ptr(double, 5));
    println!("apply_fn_ptr(negate, 5) = {}", apply_fn_ptr(negate, 5));

    println!("\n=== Transform table ===");
    let table = make_transform_table();
    let names = ["double", "triple", "negate", "+10"];
    for (name, f) in names.iter().zip(table.iter()) {
        println!("{}: {}", name, f(7));
    }

    println!("\n=== Pipeline of fn ptrs ===");
    let pipeline = build_pipeline(vec![double, triple, negate]);
    println!("pipeline(3) = {}", pipeline(3)); // negate(triple(double(3))) = -18

    println!("\n=== Coercion examples ===");
    coercion_examples();

    println!("\n=== Callback registration ===");
    let cb = register_callback(|x| x * x);
    println!("callback(9) = {}", cb(9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_ptr_apply() {
        assert_eq!(apply_fn_ptr(double, 5), 10);
        assert_eq!(apply_fn_ptr(negate, 3), -3);
    }

    #[test]
    fn test_non_capturing_coercion() {
        let f: fn(i32) -> i32 = |x| x * x;
        assert_eq!(f(4), 16);
        assert_eq!(apply_fn_ptr(f, 5), 25);
    }

    #[test]
    fn test_transform_table() {
        let table = make_transform_table();
        assert_eq!(table[0](5), 10); // double
        assert_eq!(table[1](5), 15); // triple
        assert_eq!(table[2](5), -5); // negate
        assert_eq!(table[3](5), 15); // +10
    }

    #[test]
    fn test_pipeline() {
        let p = build_pipeline(vec![|x: i32| x + 1, |x| x * 2]);
        assert_eq!(p(3), 8); // (3+1)*2
    }
}
