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

    pub fn pure_val(a: A) -> Self where A: Clone {
        Cont::new(move |k| k(Box::new(|_| unreachable!()) as Box<dyn Fn(A)->R>))
    }
}

pub fn cont_return<R: 'static, A: Clone + 'static>(a: A) -> Cont<R, A> {
    Cont::new(move |k| k(Box::new(move |_: A| { let a2 = a.clone(); k(Box::new(move |_| unreachable!())) })))
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_cps_add() {
        let result = (|a: i32, b: i32, k: &dyn Fn(i32) -> i32| k(a + b))(3, 4, &|x| x);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_factorial_cps() {
        fn fact(n: i32, k: &dyn Fn(i32) -> i32) -> i32 {
            if n <= 1 { k(1) } else { fact(n - 1, &|r| k(n * r)) }
        }
        assert_eq!(fact(5, &|x| x), 120);
    }
}
