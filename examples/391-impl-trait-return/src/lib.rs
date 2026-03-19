#![allow(clippy::all)]
//! impl Trait in Return Position

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
pub fn make_counter() -> impl Iterator<Item = i32> {
    0..
}
pub fn make_greeting(name: &str) -> impl std::fmt::Display + '_ {
    format!("Hello, {}!", name)
}

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut a = 0u64;
    let mut b = 1u64;
    std::iter::from_fn(move || {
        let c = a;
        a = b;
        b = c + b;
        Some(c)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
    }
    #[test]
    fn test_counter() {
        let first5: Vec<_> = make_counter().take(5).collect();
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_greeting() {
        assert_eq!(format!("{}", make_greeting("World")), "Hello, World!");
    }
    #[test]
    fn test_fib() {
        let fibs: Vec<_> = fibonacci().take(7).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8]);
    }
}
