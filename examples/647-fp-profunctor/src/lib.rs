//! # Profunctor
//!
//! A profunctor is contravariant in the first argument and covariant in the second.
//! It generalizes functions: Fn(A) -> B is a profunctor.

/// Trait for profunctors
pub trait Profunctor<A, B> {
    type Output<C, D>;
    
    /// Map contravariantly on input, covariantly on output
    fn dimap<C, D, F, G>(self, f: F, g: G) -> Self::Output<C, D>
    where
        F: FnOnce(C) -> A,
        G: FnOnce(B) -> D;
    
    /// Map only the input (contravariant)
    fn lmap<C, F>(self, f: F) -> Self::Output<C, B>
    where
        F: FnOnce(C) -> A,
        Self: Sized,
    {
        self.dimap(f, |b| b)
    }
    
    /// Map only the output (covariant)
    fn rmap<D, G>(self, g: G) -> Self::Output<A, D>
    where
        G: FnOnce(B) -> D,
        Self: Sized,
    {
        self.dimap(|a| a, g)
    }
}

// Approach 1: Function wrapper as profunctor
pub struct Func<A, B>(Box<dyn FnOnce(A) -> B>);

impl<A, B> Func<A, B> {
    pub fn new<F: FnOnce(A) -> B + 'static>(f: F) -> Self {
        Func(Box::new(f))
    }
    
    pub fn call(self, a: A) -> B {
        (self.0)(a)
    }
}

impl<A: 'static, B: 'static> Profunctor<A, B> for Func<A, B> {
    type Output<C, D> = Func<C, D>;
    
    fn dimap<C, D, F, G>(self, f: F, g: G) -> Func<C, D>
    where
        F: FnOnce(C) -> A + 'static,
        G: FnOnce(B) -> D + 'static,
    {
        let inner = self.0;
        Func::new(move |c| g(inner(f(c))))
    }
}

// Approach 2: Optic-style profunctor (for lenses)
pub struct Star<F, A, B> {
    run: Box<dyn FnOnce(A) -> F>,
    _phantom: std::marker::PhantomData<B>,
}

// Approach 3: Simple function composition helpers
/// Compose functions f and g: input -> f -> result1, result1 -> g -> output
pub fn dimap_fn<A, B, C, D>(
    pre: impl FnOnce(C) -> A,
    post: impl FnOnce(B) -> D,
    f: impl FnOnce(A) -> B,
) -> impl FnOnce(C) -> D {
    move |c| post(f(pre(c)))
}

/// Pre-compose: apply transformation before function
pub fn lmap_fn<A, B, C>(
    pre: impl FnOnce(C) -> A,
    f: impl FnOnce(A) -> B,
) -> impl FnOnce(C) -> B {
    move |c| f(pre(c))
}

/// Post-compose: apply transformation after function
pub fn rmap_fn<A, B, D>(
    post: impl FnOnce(B) -> D,
    f: impl FnOnce(A) -> B,
) -> impl FnOnce(A) -> D {
    move |a| post(f(a))
}

/// Tagged value for profunctor operations
#[derive(Debug, Clone, PartialEq)]
pub struct Tagged<S, A> {
    pub value: A,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, A> Tagged<S, A> {
    pub fn new(value: A) -> Self {
        Tagged { value, _phantom: std::marker::PhantomData }
    }
    
    pub fn retag<T>(self) -> Tagged<T, A> {
        Tagged::new(self.value)
    }
}

/// Demonstrate profunctor laws
pub mod laws {
    use super::*;
    
    /// Identity: dimap id id = id
    pub fn identity_law<A: Clone + PartialEq>(f: impl FnOnce(A) -> A + Clone, x: A) -> bool {
        let f_clone = f.clone();
        let result = dimap_fn(|a: A| a, |b| b, f)(x.clone());
        result == f_clone(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_profunctor() {
        let f = Func::new(|x: i32| x.to_string());
        let g = f.dimap(|s: &str| s.len() as i32, |s| format!("Result: {}", s));
        let result = g.call("hello");
        assert_eq!(result, "Result: 5");
    }

    #[test]
    fn test_dimap_fn() {
        let f = |x: i32| x * 2;
        let g = dimap_fn(
            |s: &str| s.len() as i32,
            |x| format!("doubled: {}", x),
            f
        );
        assert_eq!(g("abc"), "doubled: 6");
    }

    #[test]
    fn test_lmap_fn() {
        let f = |x: i32| x * 2;
        let g = lmap_fn(|s: &str| s.len() as i32, f);
        assert_eq!(g("hello"), 10);
    }

    #[test]
    fn test_rmap_fn() {
        let f = |x: i32| x * 2;
        let g = rmap_fn(|x| x.to_string(), f);
        assert_eq!(g(21), "42");
    }

    #[test]
    fn test_tagged() {
        struct MyTag;
        let tagged: Tagged<MyTag, i32> = Tagged::new(42);
        assert_eq!(tagged.value, 42);
    }

    #[test]
    fn test_tagged_retag() {
        struct Tag1;
        struct Tag2;
        let t1: Tagged<Tag1, i32> = Tagged::new(100);
        let t2: Tagged<Tag2, i32> = t1.retag();
        assert_eq!(t2.value, 100);
    }

    #[test]
    fn test_profunctor_composition() {
        // dimap f g . dimap h i = dimap (h . f) (g . i)
        let f = |x: i32| x.to_string();
        let pre1 = |x: i32| x + 1;
        let post1 = |s: String| s.len();
        let pre2 = |x: i32| x * 2;
        let post2 = |n: usize| n > 0;
        
        let composed1 = dimap_fn(pre1, post1, f);
        let result1 = dimap_fn(pre2, post2, composed1)(5);
        
        assert!(result1); // ((5*2)+1).to_string().len() > 0
    }

    #[test]
    fn test_func_lmap() {
        let f = Func::new(|x: i32| x * 2);
        let g = f.lmap(|s: &str| s.len() as i32);
        assert_eq!(g.call("test"), 8);
    }

    #[test]
    fn test_func_rmap() {
        let f = Func::new(|x: i32| x * 2);
        let g = f.rmap(|x| format!("value: {}", x));
        assert_eq!(g.call(21), "value: 42");
    }
}
