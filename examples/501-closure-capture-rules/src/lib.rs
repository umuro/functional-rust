//! # Closure Capture Rules — Borrowing and Moving

pub fn capture_by_ref() {
    let s = String::from("hello");
    let f = || println!("{}", s); // Borrows s
    f();
    f();
    println!("{}", s); // s still valid
}

pub fn capture_by_mut_ref() {
    let mut v = vec![1, 2, 3];
    let mut f = || v.push(4); // Mutably borrows v
    f();
    // Can't use v here until f is dropped
    drop(f);
    println!("{:?}", v);
}

pub fn capture_by_move() {
    let s = String::from("hello");
    let f = move || println!("{}", s); // Moves s into closure
    f();
    // s is no longer valid here
}

pub fn different_captures() -> impl Fn() {
    let a = 42; // Copy type
    let b = String::from("hello"); // Move type

    move || {
        println!("{} {}", a, b); // a copied, b moved
    }
}

pub fn return_closure_capturing_param(x: i32) -> impl Fn() -> i32 {
    move || x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captures() {
        capture_by_ref();
        capture_by_mut_ref();
        capture_by_move();
    }

    #[test]
    fn test_return_closure() {
        let f = return_closure_capturing_param(21);
        assert_eq!(f(), 42);
    }

    #[test]
    fn test_closure_copies_copy_types() {
        let x = 42;
        let f = move || x;
        assert_eq!(f(), 42);
        assert_eq!(x, 42); // x still valid because i32 is Copy
    }
}
