//! # Topos Theory Concepts
//!
//! A topos is a category with:
//! - All finite limits
//! - Exponentials (function objects)
//! - Subobject classifier (generalizes Bool)
//!
//! Types in programming approximate Set, which is a topos.

use std::collections::HashSet;
use std::hash::Hash;

/// Subobject classifier - generalizes Bool
/// In Set, this is {true, false}
/// In a presheaf topos, it's more complex
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Omega {
    True,
    False,
}

impl Omega {
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Omega::True, Omega::True) => Omega::True,
            _ => Omega::False,
        }
    }
    
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Omega::False, Omega::False) => Omega::False,
            _ => Omega::True,
        }
    }
    
    pub fn implies(self, other: Self) -> Self {
        match (self, other) {
            (Omega::True, Omega::False) => Omega::False,
            _ => Omega::True,
        }
    }
    
    pub fn not(self) -> Self {
        match self {
            Omega::True => Omega::False,
            Omega::False => Omega::True,
        }
    }
}

/// Characteristic function: subset indicator
pub fn char_fn<T: Eq>(subset: &[T]) -> impl Fn(&T) -> Omega + '_ {
    move |x| {
        if subset.contains(x) {
            Omega::True
        } else {
            Omega::False
        }
    }
}

/// Power object P(A) - set of all subsets
pub struct Power<T>(pub HashSet<T>);

impl<T: Eq + Hash + Clone> Power<T> {
    pub fn empty() -> Self {
        Power(HashSet::new())
    }
    
    pub fn singleton(x: T) -> Self {
        let mut s = HashSet::new();
        s.insert(x);
        Power(s)
    }
    
    pub fn from_vec(v: Vec<T>) -> Self {
        Power(v.into_iter().collect())
    }
    
    pub fn union(&self, other: &Self) -> Self {
        Power(self.0.union(&other.0).cloned().collect())
    }
    
    pub fn intersection(&self, other: &Self) -> Self {
        Power(self.0.intersection(&other.0).cloned().collect())
    }
    
    pub fn contains(&self, x: &T) -> bool {
        self.0.contains(x)
    }
}

// Exponential in Set: function type
pub type Exponential<A, B> = Box<dyn Fn(A) -> B>;

/// Internal hom in Set - functions from A to B
pub fn internal_hom<A: Clone + 'static, B: Clone + 'static>(
    domain: Vec<A>,
    _codomain: Vec<B>,
) -> Vec<Exponential<A, B>> {
    // Returns set of all functions - exponentially large!
    // This is just a conceptual representation
    vec![]
}

/// Pullback as equalizer of kernels
pub fn pullback<A: Clone + Eq, B: Clone + Eq, C: Eq>(
    xs: Vec<A>,
    ys: Vec<B>,
    f: impl Fn(&A) -> C,
    g: impl Fn(&B) -> C,
) -> Vec<(A, B)> {
    xs.iter()
        .flat_map(|a| {
            ys.iter()
                .filter(|b| f(a) == g(b))
                .map(|b| (a.clone(), b.clone()))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Pushout (colimit)
pub fn pushout<A: Clone + Hash + Eq, B: Clone + Hash + Eq, C: Clone>(
    c_val: C,
    f: impl Fn(&C) -> A,
    g: impl Fn(&C) -> B,
) -> (HashSet<A>, HashSet<B>) {
    let a = f(&c_val);
    let b = g(&c_val);
    (std::iter::once(a).collect(), std::iter::once(b).collect())
}

/// Logical operations in internal logic
pub trait InternalLogic {
    fn top() -> Self;
    fn bottom() -> Self;
    fn meet(self, other: Self) -> Self;
    fn join(self, other: Self) -> Self;
    fn implies(self, other: Self) -> Self;
}

impl InternalLogic for bool {
    fn top() -> Self { true }
    fn bottom() -> Self { false }
    fn meet(self, other: Self) -> Self { self && other }
    fn join(self, other: Self) -> Self { self || other }
    fn implies(self, other: Self) -> Self { !self || other }
}

/// Sieves (for Grothendieck topology)
pub type Sieve<T> = HashSet<T>;

pub fn maximal_sieve<T: Clone + Hash + Eq>(domain: &[T]) -> Sieve<T> {
    domain.iter().cloned().collect()
}

pub fn empty_sieve<T>() -> Sieve<T> {
    HashSet::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omega_and() {
        assert_eq!(Omega::True.and(Omega::True), Omega::True);
        assert_eq!(Omega::True.and(Omega::False), Omega::False);
    }

    #[test]
    fn test_omega_or() {
        assert_eq!(Omega::False.or(Omega::True), Omega::True);
        assert_eq!(Omega::False.or(Omega::False), Omega::False);
    }

    #[test]
    fn test_omega_implies() {
        assert_eq!(Omega::True.implies(Omega::True), Omega::True);
        assert_eq!(Omega::True.implies(Omega::False), Omega::False);
        assert_eq!(Omega::False.implies(Omega::True), Omega::True);
    }

    #[test]
    fn test_char_fn() {
        let subset = vec![1, 3, 5];
        let chi = char_fn(&subset);
        
        assert_eq!(chi(&1), Omega::True);
        assert_eq!(chi(&2), Omega::False);
        assert_eq!(chi(&5), Omega::True);
    }

    #[test]
    fn test_power_union() {
        let a = Power::from_vec(vec![1, 2, 3]);
        let b = Power::from_vec(vec![3, 4, 5]);
        let c = a.union(&b);
        
        assert!(c.contains(&1));
        assert!(c.contains(&5));
    }

    #[test]
    fn test_power_intersection() {
        let a = Power::from_vec(vec![1, 2, 3]);
        let b = Power::from_vec(vec![2, 3, 4]);
        let c = a.intersection(&b);
        
        assert!(c.contains(&2));
        assert!(c.contains(&3));
        assert!(!c.contains(&1));
    }

    #[test]
    fn test_pullback() {
        let xs = vec![1, 2, 3, 4];
        let ys = vec!["a", "bb", "ccc", "dddd"];
        
        let pb = pullback(xs, ys, |x| *x as usize, |s| s.len());
        
        assert_eq!(pb.len(), 4);
    }

    #[test]
    fn test_internal_logic() {
        assert!(bool::top());
        assert!(!bool::bottom());
        assert!(true.meet(true));
        assert!(false.join(true));
    }

    #[test]
    fn test_sieve() {
        let domain = vec![1, 2, 3, 4, 5];
        let max = maximal_sieve(&domain);
        
        assert!(max.contains(&3));
        assert_eq!(max.len(), 5);
    }
}
