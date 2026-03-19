//! # Applicative Laws
#![allow(clippy::manual_is_multiple_of)]
n
#![allow(unused_variables)]
n
#![allow(clippy::match_like_matches)]
n
#![allow(clippy::type_complexity)]
n
#![allow(clippy::too_many_lines)]
n
#![allow(clippy::manual_range_contains)]
n
#![allow(clippy::explicit_iter_loop)]
n
#![allow(clippy::needless_lifetimes)]
n
#![allow(clippy::char_lit_as_u8)]
n
#![allow(clippy::while_let_loop)]
n
#![allow(clippy::manual_strip)]
n
#![allow(clippy::useless_vec)]
n
#![allow(clippy::needless_borrow)]
n
#![allow(clippy::redundant_closure)]
n
#![allow(unused_imports)]
n
#![allow(dead_code)]
n
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
