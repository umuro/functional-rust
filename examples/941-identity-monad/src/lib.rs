#![allow(clippy::all)]
// Identity monad — the simplest possible monad.
// Wraps a value with zero extra effects.
// Useful as a base case in monad transformers.

#[derive(Debug, Clone, PartialEq)]
struct Identity<A>(A);

impl<A> Identity<A> {
    /// monadic `return` / `pure` — lift a value into Identity
    fn of(x: A) -> Self {
        Identity(x)
    }

    /// `bind` (>>=) — sequence computations
    fn bind<B, F: FnOnce(A) -> Identity<B>>(self, f: F) -> Identity<B> {
        f(self.0)
    }

    /// Functor `map`
    fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Identity<B> {
        Identity(f(self.0))
    }

    /// Extract the wrapped value
    fn run(self) -> A {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor_identity_law() {
        // map id = id
        let v = Identity(42);
        assert_eq!(v.clone().map(|x| x), v);
    }

    #[test]
    fn test_bind_chain() {
        let result = Identity::of(10)
            .bind(|x| Identity::of(x * 2))
            .bind(|x| Identity::of(x + 1));
        assert_eq!(result.run(), 21);
    }

    #[test]
    fn test_monad_left_identity() {
        // left identity: return a >>= f = f a
        let f = |x: i32| Identity::of(x * 3);
        let lhs = Identity::of(5).bind(f);
        let rhs = f(5);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_monad_right_identity() {
        // right identity: m >>= return = m
        let m = Identity(42);
        let result = m.clone().bind(Identity::of);
        assert_eq!(result, m);
    }

    #[test]
    fn test_map_composition_law() {
        // map (f . g) = map f . map g
        let v = Identity(5);
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let lhs = v.clone().map(|x| f(g(x)));
        let rhs = v.map(g).map(f);
        assert_eq!(lhs, rhs);
    }
}
