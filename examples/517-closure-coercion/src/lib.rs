//! Closure-to-fn-pointer Coercion
//!
//! Non-capturing closures coerce to fn pointers; capturing ones cannot.

/// Accept a function pointer explicitly.
pub fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// Named functions are fn pointers.
pub fn double(x: i32) -> i32 {
    x * 2
}

pub fn triple(x: i32) -> i32 {
    x * 3
}

pub fn negate(x: i32) -> i32 {
    -x
}

/// Array of function pointers (all same size — no fat pointers).
pub fn make_transform_table() -> [fn(i32) -> i32; 4] {
    [
        double,
        triple,
        negate,
        |x| x + 10, // non-capturing closure coerces to fn ptr
    ]
}

/// Function that stores fn pointers in a Vec.
pub fn build_pipeline(ops: Vec<fn(i32) -> i32>) -> impl Fn(i32) -> i32 {
    move |x| ops.iter().fold(x, |acc, f| f(acc))
}

/// Demonstrate coercion rules.
pub fn coercion_demo() {
    // Non-capturing closure → fn pointer: OK
    let _: fn(i32) -> i32 = |x| x * 2;

    // Named function → fn pointer: OK
    let _: fn(i32) -> i32 = double;

    // Capturing closure → fn pointer: NOT OK (won't compile)
    // let y = 5;
    // let _: fn(i32) -> i32 = |x| x + y;  // ERROR!
}

/// C FFI often requires fn pointers.
pub type Callback = fn(i32) -> i32;

pub fn register_callback(cb: Callback) -> i32 {
    cb(100)
}

/// When you need to store capturing closures, use Box<dyn Fn>.
pub fn store_capturing_closures() -> Vec<Box<dyn Fn(i32) -> i32>> {
    let a = 5;
    let b = 10;
    vec![
        Box::new(|x| x + 1),    // non-capturing
        Box::new(move |x| x + a), // capturing
        Box::new(move |x| x * b), // capturing
    ]
}

/// Size comparison: fn pointers vs closures.
pub fn size_demo() -> (usize, usize, usize) {
    let fn_ptr_size = std::mem::size_of::<fn(i32) -> i32>();
    let closure_size = std::mem::size_of_val(&|x: i32| x * 2);
    let capturing_size = {
        let y = 42i32;
        std::mem::size_of_val(&move |x: i32| x + y)
    };
    (fn_ptr_size, closure_size, capturing_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_fn_ptr_with_function() {
        assert_eq!(apply_fn_ptr(double, 5), 10);
        assert_eq!(apply_fn_ptr(triple, 5), 15);
        assert_eq!(apply_fn_ptr(negate, 5), -5);
    }

    #[test]
    fn test_apply_fn_ptr_with_closure() {
        // Non-capturing closure coerces to fn pointer
        assert_eq!(apply_fn_ptr(|x| x + 1, 5), 6);
        assert_eq!(apply_fn_ptr(|x| x * x, 5), 25);
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
    fn test_build_pipeline() {
        let pipeline = build_pipeline(vec![double, |x| x + 1, triple]);
        // (5 * 2 + 1) * 3 = 33
        assert_eq!(pipeline(5), 33);
    }

    #[test]
    fn test_register_callback() {
        assert_eq!(register_callback(double), 200);
        assert_eq!(register_callback(|x| x / 2), 50);
    }

    #[test]
    fn test_store_capturing_closures() {
        let closures = store_capturing_closures();
        assert_eq!(closures[0](10), 11); // +1
        assert_eq!(closures[1](10), 15); // +5
        assert_eq!(closures[2](10), 100); // *10
    }

    #[test]
    fn test_fn_pointer_size() {
        let (fn_size, non_cap, _cap) = size_demo();
        // fn pointer is one pointer size
        assert_eq!(fn_size, std::mem::size_of::<usize>());
        // non-capturing closure is zero-sized
        assert_eq!(non_cap, 0);
    }

    #[test]
    fn test_explicit_coercion() {
        let f: fn(i32) -> i32 = |x| x * 2;
        assert_eq!(f(21), 42);
    }
}
