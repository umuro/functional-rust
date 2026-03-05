//! # Monad Laws
//!
//! Monads must satisfy left identity, right identity, and associativity.

/// Monad operations for Option.
pub trait OptionMonad<A> {
    fn pure(a: A) -> Self;
    fn bind<B>(self, f: impl FnOnce(A) -> Option<B>) -> Option<B>;
}

impl<A> OptionMonad<A> for Option<A> {
    fn pure(a: A) -> Self { Some(a) }
    fn bind<B>(self, f: impl FnOnce(A) -> Option<B>) -> Option<B> { self.and_then(f) }
}

/// Left identity: pure(a).bind(f) == f(a)
pub fn check_left_identity<A: Clone, B: PartialEq>(a: A, f: impl Fn(A) -> Option<B>) -> bool {
    let left = Option::pure(a.clone()).bind(&f);
    let right = f(a);
    left == right
}

/// Right identity: m.bind(pure) == m
pub fn check_right_identity<A: Clone + PartialEq>(m: Option<A>) -> bool {
    let left = m.clone().bind(Option::pure);
    left == m
}

/// Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
pub fn check_associativity<A: Clone, B: Clone, C: PartialEq>(
    m: Option<A>,
    f: impl Fn(A) -> Option<B> + Clone,
    g: impl Fn(B) -> Option<C> + Clone,
) -> bool {
    let left = m.clone().bind(&f).bind(&g);
    let right = m.bind(|x| f(x).bind(&g));
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_identity() {
        let f = |x: i32| Some(x * 2);
        assert!(check_left_identity(5, f));
    }

    #[test]
    fn test_right_identity() {
        assert!(check_right_identity(Some(42)));
        assert!(check_right_identity(None::<i32>));
    }

    #[test]
    fn test_associativity() {
        let f = |x: i32| Some(x + 1);
        let g = |x: i32| Some(x * 2);
        assert!(check_associativity(Some(5), f, g));
    }

    #[test]
    fn test_bind_chain() {
        let result = Some(5)
            .bind(|x| Some(x + 1))
            .bind(|x| Some(x * 2));
        assert_eq!(result, Some(12));
    }
}
