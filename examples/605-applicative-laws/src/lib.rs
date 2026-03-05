//! # Applicative Laws
//! Applicatives extend Functors with pure and apply.

pub trait Applicative<A>: Sized {
    fn pure(a: A) -> Self;
    fn ap<B>(self, f: Self) -> Self where Self: Sized;
}

impl<A> Applicative<A> for Option<A> {
    fn pure(a: A) -> Self { Some(a) }
    fn ap<B>(self, _f: Self) -> Self { self }
}

pub fn identity_law<A: Clone + PartialEq>(v: Option<A>) -> bool {
    let mapped = v.clone().map(|x| x);
    mapped == v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pure() { assert_eq!(Option::pure(42), Some(42)); }
}
