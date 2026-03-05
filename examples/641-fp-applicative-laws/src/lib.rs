//! # Applicative Functor Laws
//!
//! Demonstrates the four applicative functor laws in Rust:
//! - Identity: pure id <*> v = v
//! - Homomorphism: pure f <*> pure x = pure (f x)
//! - Interchange: u <*> pure y = pure (|f| f(y)) <*> u
//! - Composition: pure (.) <*> u <*> v <*> w = u <*> (v <*> w)

/// A simple Applicative wrapper for demonstration
#[derive(Debug, Clone, PartialEq)]
pub struct Applicative<T>(pub T);

impl<T> Applicative<T> {
    /// Lift a value into the applicative context (pure)
    pub fn pure(value: T) -> Self {
        Applicative(value)
    }

    /// Extract the inner value
    pub fn unwrap(self) -> T {
        self.0
    }
}

impl<T: Clone> Applicative<T> {
    /// Apply a wrapped function to a wrapped value
    pub fn ap<U, F>(self, f: Applicative<F>) -> Applicative<U>
    where
        F: FnOnce(T) -> U,
    {
        Applicative(f.0(self.0))
    }
}

/// Identity function for law verification
pub fn identity<T>(x: T) -> T {
    x
}

/// Compose two functions
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    F: FnOnce(B) -> C,
    G: FnOnce(A) -> B,
{
    move |x| f(g(x))
}

// Alternative approach using Option as Applicative
pub mod option_applicative {
    /// Apply function inside Option to value inside Option
    pub fn ap<A, B, F>(opt_f: Option<F>, opt_a: Option<A>) -> Option<B>
    where
        F: FnOnce(A) -> B,
    {
        match (opt_f, opt_a) {
            (Some(f), Some(a)) => Some(f(a)),
            _ => None,
        }
    }

    /// Lift a value into Option (pure)
    pub fn pure<T>(value: T) -> Option<T> {
        Some(value)
    }

    /// Verify identity law: pure id <*> v = v
    pub fn verify_identity<T: Clone + PartialEq>(v: Option<T>) -> bool {
        let id_fn: fn(T) -> T = |x| x;
        ap(pure(id_fn), v.clone()) == v
    }

    /// Verify homomorphism: pure f <*> pure x = pure (f x)
    pub fn verify_homomorphism<A: Clone, B: PartialEq, F>(f: F, x: A) -> bool
    where
        F: FnOnce(A) -> B + Clone,
    {
        let f_clone = f.clone();
        ap(pure(f), pure(x.clone())) == pure(f_clone(x))
    }
}

// Third approach: Result as Applicative
pub mod result_applicative {
    /// Apply function inside Result to value inside Result
    pub fn ap<A, B, E, F>(res_f: Result<F, E>, res_a: Result<A, E>) -> Result<B, E>
    where
        F: FnOnce(A) -> B,
    {
        match (res_f, res_a) {
            (Ok(f), Ok(a)) => Ok(f(a)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }

    /// Lift a value into Result (pure)
    pub fn pure<T, E>(value: T) -> Result<T, E> {
        Ok(value)
    }

    /// Lift two values and apply a binary function
    pub fn lift2<A, B, C, E, F>(f: F, a: Result<A, E>, b: Result<B, E>) -> Result<C, E>
    where
        F: FnOnce(A, B) -> C,
    {
        match (a, b) {
            (Ok(a), Ok(b)) => Ok(f(a, b)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_applicative_pure() {
        let app = Applicative::pure(42);
        assert_eq!(app.unwrap(), 42);
    }

    #[test]
    fn test_applicative_ap() {
        let value = Applicative::pure(10);
        let func = Applicative::pure(|x: i32| x * 2);
        let result = value.ap(func);
        assert_eq!(result.unwrap(), 20);
    }

    #[test]
    fn test_identity_law() {
        // pure id <*> v = v
        let v = Applicative::pure(42);
        let id_fn = Applicative::pure(identity);
        let result = v.clone().ap(id_fn);
        assert_eq!(result, v);
    }

    #[test]
    fn test_option_identity_law() {
        use option_applicative::*;
        assert!(verify_identity(Some(42)));
        assert!(verify_identity(Some("hello".to_string())));
        assert!(verify_identity::<i32>(None));
    }

    #[test]
    fn test_option_homomorphism_law() {
        use option_applicative::*;
        let f = |x: i32| x * 2;
        assert!(verify_homomorphism(f, 21));
    }

    #[test]
    fn test_option_ap() {
        use option_applicative::*;
        let f = pure(|x: i32| x + 1);
        let v = pure(5);
        assert_eq!(ap(f, v), Some(6));
    }

    #[test]
    fn test_option_ap_none() {
        use option_applicative::*;
        let f: Option<fn(i32) -> i32> = None;
        let v = pure(5);
        assert_eq!(ap(f, v), None);
    }

    #[test]
    fn test_result_ap() {
        use result_applicative::*;
        let f: Result<fn(i32) -> i32, &str> = pure(|x| x * 3);
        let v = pure(7);
        assert_eq!(ap(f, v), Ok(21));
    }

    #[test]
    fn test_result_lift2() {
        use result_applicative::*;
        let a: Result<i32, &str> = Ok(10);
        let b: Result<i32, &str> = Ok(20);
        assert_eq!(lift2(|x, y| x + y, a, b), Ok(30));
    }

    #[test]
    fn test_result_lift2_error() {
        use result_applicative::*;
        let a: Result<i32, &str> = Ok(10);
        let b: Result<i32, &str> = Err("error");
        assert_eq!(lift2(|x, y| x + y, a, b), Err("error"));
    }

    #[test]
    fn test_compose_functions() {
        let f = |x: i32| x * 2;
        let g = |x: i32| x + 3;
        let composed = compose(f, g);
        assert_eq!(composed(5), 16); // (5 + 3) * 2
    }
}
