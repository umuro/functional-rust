//! # 508. Partial Application with Closures
//! Simulating partial application by capturing fixed arguments in closures.

/// Full function: add two numbers
fn add(x: i32, y: i32) -> i32 { x + y }

/// Full function: clamp x into [lo, hi]
fn clamp(lo: i32, hi: i32, x: i32) -> i32 { x.max(lo).min(hi) }

/// Full function: check if x is in [lo, hi]
fn between(lo: i32, hi: i32, x: i32) -> bool { x >= lo && x <= hi }

/// Generic partial application: fix the first argument of a 2-arg function
fn partial<A: Copy, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
{
    move |b| f(a, b)
}

/// Partial with two fixed args (fix lo and hi of a 3-arg function)
fn partial2<A: Copy, B: Copy, C, D, F>(f: F, a: A, b: B) -> impl Fn(C) -> D
where
    F: Fn(A, B, C) -> D,
{
    move |c| f(a, b, c)
}

fn main() {
    // Manual partial application via closures
    let add5 = |y| add(5, y);
    let double = |y| add(y, y); // or multiply(2, y)
    let clamp_0_100 = |x| clamp(0, 100, x);
    let in_teens = |x| between(13, 19, x);

    println!("add5(10) = {}", add5(10));
    println!("double(7) = {}", double(7));
    println!("clamp(150) = {}", clamp_0_100(150));
    println!("clamp(-5) = {}", clamp_0_100(-5));
    println!("in_teens(15) = {}", in_teens(15));
    println!("in_teens(20) = {}", in_teens(20));

    // Generic partial helper
    let times3 = partial(|x: i32, y: i32| x * y, 3);
    println!("times3(8) = {}", times3(8));

    let starts_with_hello = partial(|prefix: &str, s: &str| s.starts_with(prefix), "hello");
    println!("starts_with_hello(\"hello world\") = {}", starts_with_hello("hello world"));
    println!("starts_with_hello(\"hi there\") = {}", starts_with_hello("hi there"));

    // Partial2: fix lo=0, hi=100
    let clamp_fn = partial2(clamp, 0, 100);
    println!("clamp_fn(42) = {}", clamp_fn(42));
    println!("clamp_fn(200) = {}", clamp_fn(200));

    // In a pipeline
    let items = [1, 2, 3, 4, 5, 6];
    let add5_fn = |x: &i32| add(5, *x);
    let double_fn = |x: i32| add(x, x);
    let in_teens_fn = |x: &i32| between(13, 19, *x);

    let result: Vec<i32> = items.iter()
        .map(add5_fn)
        .map(double_fn)
        .filter(in_teens_fn)
        .collect();
    println!("pipeline result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_partial() {
        let add10 = |y: i32| add(10, y);
        assert_eq!(add10(5), 15);
        assert_eq!(add10(0), 10);
    }

    #[test]
    fn test_generic_partial() {
        let mul_by_7 = partial(|x: i32, y: i32| x * y, 7);
        assert_eq!(mul_by_7(6), 42);
    }

    #[test]
    fn test_partial2() {
        let check = partial2(between, 5, 10);
        assert!(check(7));
        assert!(!check(11));
    }

    #[test]
    fn test_partial_string() {
        let prefix_checker = partial(|p: &str, s: &str| s.starts_with(p), "rust");
        assert!(prefix_checker("rustacean"));
        assert!(!prefix_checker("python"));
    }
}
