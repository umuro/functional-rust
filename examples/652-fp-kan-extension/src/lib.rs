//! # Kan Extensions
//!
//! Kan extensions are universal constructions that generalize many concepts:
//! - Right Kan extension (Ran): generalizes limits
//! - Left Kan extension (Lan): generalizes colimits
//!
//! Yoneda lemma is a special case of Kan extension.

use std::marker::PhantomData;

/// Right Kan Extension along K: C -> D of functor F: C -> E
/// Ran_K F (d) = ∫_c E(d, Kc) → F(c)
/// 
/// In Rust, we approximate with a simpler formulation
pub struct Ran<K, F, A> {
    run: Box<dyn FnOnce(Box<dyn FnOnce(K) -> A>) -> F>,
    _phantom: PhantomData<A>,
}

/// Left Kan Extension
/// Lan_K F (d) = ∫^c D(Kc, d) × F(c)
/// 
/// Simplified representation
pub struct Lan<K, F, A> {
    // Existential: exists c. (K(c) -> A, F(c))
    transform: Box<dyn FnOnce() -> A>,
    _phantom: PhantomData<(K, F)>,
}

// Approach 1: Codensity as Right Kan Extension along Identity
/// Codensity monad = Ran_Id Id
pub struct Codensity<A> {
    run: Box<dyn FnOnce(Box<dyn FnOnce(A) -> A>) -> A>,
}

impl<A: 'static> Codensity<A> {
    pub fn pure(a: A) -> Self {
        Codensity {
            run: Box::new(move |k| k(a))
        }
    }
    
    pub fn flat_map<B: 'static, F>(self, f: F) -> Codensity<B>
    where
        F: FnOnce(A) -> Codensity<B> + 'static,
    {
        Codensity {
            run: Box::new(move |k| {
                (self.run)(Box::new(move |a| {
                    (f(a).run)(k)
                }))
            })
        }
    }
    
    pub fn run(self) -> A {
        (self.run)(Box::new(|a| a))
    }
}

// Approach 2: Density as Left Kan Extension along Identity
/// Density comonad = Lan_Id Id
pub struct Density<A> {
    // Simplified: a value with a context
    value: A,
    context: Box<dyn FnOnce(A) -> A>,
}

impl<A: Clone + 'static> Density<A> {
    pub fn pure(a: A) -> Self {
        Density {
            value: a,
            context: Box::new(|x| x),
        }
    }
    
    pub fn extract(&self) -> A {
        self.value.clone()
    }
}

// Approach 3: Free monad via Left Kan Extension
/// Free F A = Lan_F Id A
pub enum Free<F, A> {
    Pure(A),
    Free(Box<F>),
}

// Practical Example: Kan extension for functors
/// Right Kan extension for Option
pub fn ran_option<A, B, F>(default: B, f: F) -> impl FnOnce(Option<A>) -> B
where
    F: FnOnce(A) -> B,
{
    move |opt| match opt {
        Some(a) => f(a),
        None => default,
    }
}

/// Left Kan extension for Option (Option acts as colimit)
pub fn lan_option<A>(value: A) -> Option<A> {
    Some(value)
}

/// Kan extension for Vec (as a limit/colimit)
pub fn ran_vec<A: Clone, B, F>(empty: B, combine: impl Fn(B, B) -> B, f: F) -> impl Fn(Vec<A>) -> B
where
    F: Fn(A) -> B,
{
    move |vec| {
        if vec.is_empty() {
            return empty.clone();
        }
        vec.into_iter()
            .map(&f)
            .reduce(|a, b| combine(a, b))
            .unwrap_or_else(|| empty.clone())
    }
}

/// Day convolution (related to Kan extensions)
pub struct Day<F, G, A> {
    left: F,
    right: G,
    combine: Box<dyn FnOnce(F, G) -> A>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codensity_pure() {
        let c = Codensity::pure(42);
        assert_eq!(c.run(), 42);
    }

    #[test]
    fn test_codensity_flat_map() {
        let c = Codensity::pure(10)
            .flat_map(|x| Codensity::pure(x + 5))
            .flat_map(|x| Codensity::pure(x * 2));
        assert_eq!(c.run(), 30);
    }

    #[test]
    fn test_density_extract() {
        let d = Density::pure(42);
        assert_eq!(d.extract(), 42);
    }

    #[test]
    fn test_ran_option_some() {
        let f = ran_option(0, |x: i32| x * 2);
        assert_eq!(f(Some(21)), 42);
    }

    #[test]
    fn test_ran_option_none() {
        let f = ran_option(0, |x: i32| x * 2);
        assert_eq!(f(None), 0);
    }

    #[test]
    fn test_lan_option() {
        assert_eq!(lan_option(42), Some(42));
    }

    #[test]
    fn test_ran_vec() {
        let sum_doubled = ran_vec(0, |a, b| a + b, |x: i32| x * 2);
        assert_eq!(sum_doubled(vec![1, 2, 3]), 12);
    }

    #[test]
    fn test_ran_vec_empty() {
        let sum = ran_vec(0, |a, b| a + b, |x: i32| x);
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn test_codensity_associativity() {
        let c1 = Codensity::pure(1)
            .flat_map(|x| Codensity::pure(x + 1))
            .flat_map(|x| Codensity::pure(x * 2));
        
        let c2 = Codensity::pure(1)
            .flat_map(|x| Codensity::pure(x + 1).flat_map(|y| Codensity::pure(y * 2)));
        
        assert_eq!(c1.run(), c2.run());
    }
}
