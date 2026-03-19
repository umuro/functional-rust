#![allow(clippy::all)]
/// Continuation Monad — Delimited Continuations in Rust
///
/// The continuation monad wraps computations that pass their result to a callback.
/// type Cont r a = (a -> r) -> r

pub struct Cont<R, A> {
    run: Box<dyn Fn(Box<dyn Fn(A) -> R>) -> R>,
}

impl<R: 'static, A: 'static> Cont<R, A> {
    pub fn new(f: impl Fn(Box<dyn Fn(A) -> R>) -> R + 'static) -> Self {
        Cont { run: Box::new(f) }
    }

    pub fn run_cont(self, k: impl Fn(A) -> R + 'static) -> R {
        (self.run)(Box::new(k))
    }
}

/// Wrap a pure value in Cont: \k -> k(a)
pub fn cont_return<R: 'static, A: Clone + 'static>(a: A) -> Cont<R, A> {
    Cont::new(move |k: Box<dyn Fn(A) -> R>| k(a.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cps_add() {
        let result = (|a: i32, b: i32, k: &dyn Fn(i32) -> i32| k(a + b))(3, 4, &|x| x);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_factorial_cps() {
        fn fact(n: i32, k: &dyn Fn(i32) -> i32) -> i32 {
            if n <= 1 {
                k(1)
            } else {
                fact(n - 1, &|r| k(n * r))
            }
        }
        assert_eq!(fact(5, &|x| x), 120);
    }

    #[test]
    fn test_cont_return() {
        let c = cont_return::<i32, i32>(42);
        assert_eq!(c.run_cont(|x| x), 42);
        let c2 = cont_return::<i32, i32>(10);
        assert_eq!(c2.run_cont(|x| x * 2), 20);
    }
}
