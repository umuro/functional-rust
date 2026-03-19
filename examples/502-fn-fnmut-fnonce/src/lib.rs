#![allow(clippy::all)]
//! # Fn FnMut FnOnce — Closure Traits

/// Fn: can be called multiple times, doesn't mutate state
pub fn call_fn<F: Fn() -> i32>(f: F) -> i32 {
    f() + f()
}

/// FnMut: can be called multiple times, may mutate state
pub fn call_fn_mut<F: FnMut() -> i32>(mut f: F) -> i32 {
    f() + f()
}

/// FnOnce: can only be called once, consumes captured values
pub fn call_fn_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

pub fn example_fn() -> i32 {
    let x = 10;
    call_fn(|| x) // Borrows x immutably
}

pub fn example_fn_mut() -> i32 {
    let mut counter = 0;
    call_fn_mut(|| {
        counter += 1;
        counter
    }) // Mutably borrows counter
}

pub fn example_fn_once() -> String {
    let s = String::from("owned");
    call_fn_once(|| s) // Moves s
}

/// All closures implement FnOnce, some also implement FnMut, some also implement Fn
pub fn closure_hierarchy() {
    let x = 10;
    let f: &dyn Fn() -> i32 = &|| x;
    let _: &dyn FnMut() -> i32 = f; // Fn implies FnMut
    let _: &dyn FnOnce() -> i32 = f; // FnMut implies FnOnce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
        assert_eq!(example_fn(), 20);
    }

    #[test]
    fn test_fn_mut() {
        assert_eq!(example_fn_mut(), 3); // 1 + 2
    }

    #[test]
    fn test_fn_once() {
        assert_eq!(example_fn_once(), "owned");
    }

    #[test]
    fn test_fn_pointer() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        let fp: fn(i32) -> i32 = add_one;
        assert_eq!(fp(41), 42);
    }
}
