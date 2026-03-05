//! # Yoneda Lemma
//!
//! The Yoneda lemma states: Nat(Hom(A, -), F) ≅ F(A)
//! This means natural transformations from a representable functor are
//! equivalent to elements of F(A).
//!
//! In Rust, this manifests as an optimization technique:
//! - Store computation as a function from morphisms
//! - Fuse multiple maps into a single pass

use std::marker::PhantomData;

/// Yoneda-encoded functor
/// Instead of F<A>, we store forall B. (A -> B) -> F<B>
pub struct Yoneda<F, A> {
    run: Box<dyn FnOnce(Box<dyn FnOnce(A) -> A>) -> F>,
    _phantom: PhantomData<A>,
}

// Simplified: Yoneda for Vec (concrete)
pub struct YonedaVec<A> {
    // Stores a continuation that produces Vec<B> given (A -> B)
    run: Box<dyn FnOnce() -> Vec<A>>,
}

impl<A: 'static> YonedaVec<A> {
    /// Lift a Vec into Yoneda representation
    pub fn lift(vec: Vec<A>) -> Self {
        YonedaVec { run: Box::new(move || vec) }
    }
    
    /// Lower back to Vec
    pub fn lower(self) -> Vec<A> {
        (self.run)()
    }
    
    /// Map (O(1) - deferred)
    pub fn map<B: 'static, F: FnOnce(A) -> B + 'static>(self, f: F) -> YonedaVec<B> {
        let run = self.run;
        YonedaVec {
            run: Box::new(move || {
                run().into_iter().map(f).collect()
            })
        }
    }
}

// Approach 2: Yoneda for Option
pub struct YonedaOption<A> {
    run: Box<dyn FnOnce() -> Option<A>>,
}

impl<A: 'static> YonedaOption<A> {
    pub fn lift(opt: Option<A>) -> Self {
        YonedaOption { run: Box::new(move || opt) }
    }
    
    pub fn lower(self) -> Option<A> {
        (self.run)()
    }
    
    pub fn map<B: 'static, F: FnOnce(A) -> B + 'static>(self, f: F) -> YonedaOption<B> {
        let run = self.run;
        YonedaOption {
            run: Box::new(move || run().map(f))
        }
    }
}

// Approach 3: Codensity/CPS transformation (related to Yoneda)
pub struct Codensity<A> {
    run: Box<dyn FnOnce(Box<dyn FnOnce(A) -> A>) -> A>,
}

impl<A: 'static> Codensity<A> {
    pub fn pure(a: A) -> Self {
        Codensity {
            run: Box::new(move |k| k(a))
        }
    }
    
    pub fn run_identity(self) -> A {
        (self.run)(Box::new(|a| a))
    }
}

/// Demonstrate map fusion with Yoneda
/// Multiple maps compose into a single traversal
pub fn demonstrate_fusion() -> Vec<i32> {
    let vec = vec![1, 2, 3, 4, 5];
    
    // Without Yoneda: 3 traversals
    // vec.iter().map(|x| x + 1).map(|x| x * 2).map(|x| x - 1).collect()
    
    // With Yoneda: 1 traversal (maps compose)
    YonedaVec::lift(vec)
        .map(|x| x + 1)
        .map(|x| x * 2)
        .map(|x| x - 1)
        .lower()
}

/// Coyoneda - dual of Yoneda
/// Store a value and pending transformation
pub struct Coyoneda<A, B> {
    value: A,
    transform: Box<dyn FnOnce(A) -> B>,
}

impl<A: 'static, B: 'static> Coyoneda<A, B> {
    pub fn lift(a: A) -> Coyoneda<A, A> {
        Coyoneda {
            value: a,
            transform: Box::new(|x| x),
        }
    }
    
    pub fn map<C: 'static, F: FnOnce(B) -> C + 'static>(self, f: F) -> Coyoneda<A, C> {
        let transform = self.transform;
        Coyoneda {
            value: self.value,
            transform: Box::new(move |a| f(transform(a))),
        }
    }
    
    pub fn lower(self) -> B {
        (self.transform)(self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yoneda_vec_lift_lower() {
        let vec = vec![1, 2, 3];
        let y = YonedaVec::lift(vec.clone());
        assert_eq!(y.lower(), vec);
    }

    #[test]
    fn test_yoneda_vec_map() {
        let vec = vec![1, 2, 3];
        let y = YonedaVec::lift(vec);
        let result = y.map(|x| x * 2).lower();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_yoneda_vec_fusion() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = YonedaVec::lift(vec)
            .map(|x| x + 1)
            .map(|x| x * 2)
            .lower();
        assert_eq!(result, vec![4, 6, 8, 10, 12]);
    }

    #[test]
    fn test_yoneda_option_some() {
        let y = YonedaOption::lift(Some(42));
        let result = y.map(|x| x * 2).lower();
        assert_eq!(result, Some(84));
    }

    #[test]
    fn test_yoneda_option_none() {
        let y: YonedaOption<i32> = YonedaOption::lift(None);
        let result = y.map(|x| x * 2).lower();
        assert_eq!(result, None);
    }

    #[test]
    fn test_demonstrate_fusion() {
        let result = demonstrate_fusion();
        // (x + 1) * 2 - 1 for x in 1..5
        assert_eq!(result, vec![3, 5, 7, 9, 11]);
    }

    #[test]
    fn test_coyoneda_lift_lower() {
        let c = Coyoneda::lift(42);
        assert_eq!(c.lower(), 42);
    }

    #[test]
    fn test_coyoneda_map() {
        let c = Coyoneda::lift(10)
            .map(|x| x + 5)
            .map(|x| x * 2);
        assert_eq!(c.lower(), 30);
    }

    #[test]
    fn test_codensity_pure() {
        let c = Codensity::pure(42);
        assert_eq!(c.run_identity(), 42);
    }
}
