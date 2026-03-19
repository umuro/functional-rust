#![allow(clippy::all)]
//! # Advanced Product Types
//!
//! Tuples, records, and heterogeneous collections.

/// Pair type - canonical two-way product.
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B> Pair<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Pair(a, b)
    }
    pub fn fst(&self) -> &A {
        &self.0
    }
    pub fn snd(&self) -> &B {
        &self.1
    }
    pub fn swap(self) -> Pair<B, A> {
        Pair(self.1, self.0)
    }
    pub fn map_fst<C>(self, f: impl FnOnce(A) -> C) -> Pair<C, B> {
        Pair(f(self.0), self.1)
    }
    pub fn map_snd<C>(self, f: impl FnOnce(B) -> C) -> Pair<A, C> {
        Pair(self.0, f(self.1))
    }
}

/// Triple type.
#[derive(Debug, Clone, PartialEq)]
pub struct Triple<A, B, C>(pub A, pub B, pub C);

impl<A, B, C> Triple<A, B, C> {
    pub fn new(a: A, b: B, c: C) -> Self {
        Triple(a, b, c)
    }
    pub fn first(&self) -> &A {
        &self.0
    }
    pub fn second(&self) -> &B {
        &self.1
    }
    pub fn third(&self) -> &C {
        &self.2
    }
}

/// Zip two vectors into pairs.
pub fn zip<A, B>(xs: Vec<A>, ys: Vec<B>) -> Vec<Pair<A, B>> {
    xs.into_iter().zip(ys).map(|(a, b)| Pair(a, b)).collect()
}

/// Unzip pairs into two vectors.
pub fn unzip<A, B>(pairs: Vec<Pair<A, B>>) -> (Vec<A>, Vec<B>) {
    pairs.into_iter().map(|Pair(a, b)| (a, b)).unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let p = Pair::new(1, "hello");
        assert_eq!(*p.fst(), 1);
        assert_eq!(*p.snd(), "hello");
    }

    #[test]
    fn test_swap() {
        let p = Pair::new(1, 2);
        assert_eq!(p.swap(), Pair(2, 1));
    }

    #[test]
    fn test_zip_unzip() {
        let xs = vec![1, 2, 3];
        let ys = vec!["a", "b", "c"];
        let zipped = zip(xs, ys);
        assert_eq!(zipped.len(), 3);
        let (xs2, ys2) = unzip(zipped);
        assert_eq!(xs2, vec![1, 2, 3]);
    }
}
