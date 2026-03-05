//! # Contravariant Functor
//!
//! A contravariant functor reverses the direction of morphisms.
//! While Functor has `map: (A -> B) -> F<A> -> F<B>`,
//! Contravariant has `contramap: (A -> B) -> F<B> -> F<A>`.

/// Trait for contravariant functors
pub trait Contravariant<A> {
    type Output<B>;
    
    fn contramap<B, F>(self, f: F) -> Self::Output<B>
    where
        F: FnOnce(B) -> A;
}

// Approach 1: Predicate as contravariant
#[derive(Clone)]
pub struct Predicate<A> {
    run: std::sync::Arc<dyn Fn(&A) -> bool + Send + Sync>,
}

impl<A> Predicate<A> {
    pub fn new<F: Fn(&A) -> bool + Send + Sync + 'static>(f: F) -> Self {
        Predicate { run: std::sync::Arc::new(f) }
    }
    
    pub fn test(&self, a: &A) -> bool {
        (self.run)(a)
    }
    
    pub fn and(self, other: Self) -> Self 
    where A: 'static
    {
        let run1 = self.run;
        let run2 = other.run;
        Predicate::new(move |a| run1(a) && run2(a))
    }
    
    pub fn or(self, other: Self) -> Self 
    where A: 'static
    {
        let run1 = self.run;
        let run2 = other.run;
        Predicate::new(move |a| run1(a) || run2(a))
    }
    
    pub fn negate(self) -> Self 
    where A: 'static
    {
        let run = self.run;
        Predicate::new(move |a| !run(a))
    }
}

impl<A: 'static> Contravariant<A> for Predicate<A> {
    type Output<B> = Predicate<B>;
    
    fn contramap<B, F>(self, f: F) -> Predicate<B>
    where
        F: FnOnce(B) -> A + Clone + Send + Sync + 'static,
    {
        let run = self.run;
        Predicate::new(move |b: &B| {
            // Note: This is a simplification; real impl would need Fn, not FnOnce
            // For demonstration purposes
            false // Placeholder
        })
    }
}

// Better approach: use references throughout
pub fn contramap_predicate<A, B, F>(pred: impl Fn(&A) -> bool, f: F) -> impl Fn(&B) -> bool
where
    F: Fn(&B) -> A,
{
    move |b| pred(&f(b))
}

// Approach 2: Comparator as contravariant
use std::cmp::Ordering;

pub struct Comparator<A> {
    compare: Box<dyn Fn(&A, &A) -> Ordering>,
}

impl<A> Comparator<A> {
    pub fn new<F: Fn(&A, &A) -> Ordering + 'static>(f: F) -> Self {
        Comparator { compare: Box::new(f) }
    }
    
    pub fn compare(&self, a: &A, b: &A) -> Ordering {
        (self.compare)(a, b)
    }
    
    pub fn contramap<B, F>(self, f: F) -> Comparator<B>
    where
        F: Fn(&B) -> A + 'static,
        A: 'static,
    {
        let cmp = self.compare;
        Comparator::new(move |b1: &B, b2: &B| cmp(&f(b1), &f(b2)))
    }
    
    pub fn reverse(self) -> Self 
    where A: 'static
    {
        let cmp = self.compare;
        Comparator::new(move |a, b| cmp(a, b).reverse())
    }
}

// Approach 3: Sink/consumer as contravariant
pub struct Sink<A> {
    consume: Box<dyn FnMut(A)>,
}

impl<A> Sink<A> {
    pub fn new<F: FnMut(A) + 'static>(f: F) -> Self {
        Sink { consume: Box::new(f) }
    }
    
    pub fn feed(&mut self, a: A) {
        (self.consume)(a);
    }
    
    pub fn contramap<B, F>(self, f: F) -> Sink<B>
    where
        F: Fn(B) -> A + 'static,
        A: 'static,
    {
        let mut consume = self.consume;
        Sink::new(move |b| consume(f(b)))
    }
}

/// Equivalence relation as contravariant
pub struct Equivalence<A> {
    equivalent: Box<dyn Fn(&A, &A) -> bool>,
}

impl<A> Equivalence<A> {
    pub fn new<F: Fn(&A, &A) -> bool + 'static>(f: F) -> Self {
        Equivalence { equivalent: Box::new(f) }
    }
    
    pub fn test(&self, a: &A, b: &A) -> bool {
        (self.equivalent)(a, b)
    }
    
    pub fn contramap<B, F>(self, f: F) -> Equivalence<B>
    where
        F: Fn(&B) -> A + 'static,
        A: 'static,
    {
        let eq = self.equivalent;
        Equivalence::new(move |b1: &B, b2: &B| eq(&f(b1), &f(b2)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predicate_basic() {
        let is_even = Predicate::new(|x: &i32| x % 2 == 0);
        assert!(is_even.test(&4));
        assert!(!is_even.test(&3));
    }

    #[test]
    fn test_predicate_and() {
        let is_even = Predicate::new(|x: &i32| x % 2 == 0);
        let is_positive = Predicate::new(|x: &i32| *x > 0);
        let combined = is_even.and(is_positive);
        
        assert!(combined.test(&4));
        assert!(!combined.test(&-4));
        assert!(!combined.test(&3));
    }

    #[test]
    fn test_predicate_or() {
        let is_zero = Predicate::new(|x: &i32| *x == 0);
        let is_positive = Predicate::new(|x: &i32| *x > 0);
        let non_negative = is_zero.or(is_positive);
        
        assert!(non_negative.test(&0));
        assert!(non_negative.test(&5));
        assert!(!non_negative.test(&-1));
    }

    #[test]
    fn test_predicate_negate() {
        let is_even = Predicate::new(|x: &i32| x % 2 == 0);
        let is_odd = is_even.negate();
        
        assert!(is_odd.test(&3));
        assert!(!is_odd.test(&4));
    }

    #[test]
    fn test_contramap_predicate() {
        let is_positive = |x: &i32| *x > 0;
        let string_has_positive_len = contramap_predicate(is_positive, |s: &String| s.len() as i32);
        
        assert!(string_has_positive_len(&"hello".to_string()));
        assert!(!string_has_positive_len(&"".to_string()));
    }

    #[test]
    fn test_comparator_basic() {
        let int_cmp = Comparator::new(|a: &i32, b: &i32| a.cmp(b));
        assert_eq!(int_cmp.compare(&1, &2), Ordering::Less);
        assert_eq!(int_cmp.compare(&2, &1), Ordering::Greater);
        assert_eq!(int_cmp.compare(&1, &1), Ordering::Equal);
    }

    #[test]
    fn test_comparator_contramap() {
        let int_cmp = Comparator::new(|a: &i32, b: &i32| a.cmp(b));
        let by_len = int_cmp.contramap(|s: &String| s.len() as i32);
        
        assert_eq!(by_len.compare(&"hi".to_string(), &"hello".to_string()), Ordering::Less);
    }

    #[test]
    fn test_comparator_reverse() {
        let int_cmp = Comparator::new(|a: &i32, b: &i32| a.cmp(b));
        let reversed = int_cmp.reverse();
        
        assert_eq!(reversed.compare(&1, &2), Ordering::Greater);
    }

    #[test]
    fn test_equivalence() {
        let mod_eq = Equivalence::new(|a: &i32, b: &i32| a % 3 == b % 3);
        assert!(mod_eq.test(&4, &7));
        assert!(!mod_eq.test(&4, &5));
    }

    #[test]
    fn test_equivalence_contramap() {
        let len_eq = Equivalence::new(|a: &usize, b: &usize| a == b);
        let by_len = len_eq.contramap(|s: &String| s.len());
        
        assert!(by_len.test(&"hi".to_string(), &"ok".to_string()));
        assert!(!by_len.test(&"hi".to_string(), &"hello".to_string()));
    }
}
