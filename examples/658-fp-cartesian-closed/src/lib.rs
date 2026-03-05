//! # Cartesian Closed Categories
//!
//! A CCC has products, terminal objects, and exponentials.
//! Programming types form a CCC with (,), (), and -> as the structures.

use std::marker::PhantomData;

/// Terminal object - only one value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Terminal;

/// Unique morphism to terminal
pub fn terminal<A>(_: A) -> Terminal {
    Terminal
}

/// Product (binary product)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product<A, B>(pub A, pub B);

impl<A, B> Product<A, B> {
    pub fn new(a: A, b: B) -> Self { Product(a, b) }
    pub fn fst(&self) -> &A { &self.0 }
    pub fn snd(&self) -> &B { &self.1 }
    pub fn split(self) -> (A, B) { (self.0, self.1) }
}

/// Exponential (function type)
pub struct Exponential<A, B> {
    apply: Box<dyn FnOnce(A) -> B>,
}

impl<A, B> Exponential<A, B> {
    pub fn new<F: FnOnce(A) -> B + 'static>(f: F) -> Self {
        Exponential { apply: Box::new(f) }
    }
    
    pub fn apply(self, a: A) -> B {
        (self.apply)(a)
    }
}

// CCC laws

/// curry: (A × B → C) → (A → B → C)
pub fn curry<A: 'static + Clone, B: 'static, C: 'static>(
    f: impl FnOnce((A, B)) -> C + 'static,
) -> impl FnOnce(A) -> Exponential<B, C> {
    move |a| Exponential::new(move |b| f((a, b)))
}

/// uncurry: (A → B → C) → (A × B → C)
pub fn uncurry<A, B, C>(
    f: impl FnOnce(A) -> Exponential<B, C>,
) -> impl FnOnce((A, B)) -> C {
    move |(a, b)| f(a).apply(b)
}

/// Product universal property: given f: X → A and g: X → B, there's unique h: X → A × B
pub fn product_universal<X: Clone, A, B>(
    f: impl Fn(X) -> A,
    g: impl Fn(X) -> B,
) -> impl Fn(X) -> Product<A, B> {
    move |x| Product(f(x.clone()), g(x))
}

/// Diagonal morphism: A → A × A
pub fn diagonal<A: Clone>(a: A) -> Product<A, A> {
    Product(a.clone(), a)
}

/// Projections
pub fn fst<A, B>(p: Product<A, B>) -> A { p.0 }
pub fn snd<A, B>(p: Product<A, B>) -> B { p.1 }

// Exponential laws

/// eval: (B^A × A) → B
pub fn eval<A, B>(pair: (Exponential<A, B>, A)) -> B {
    pair.0.apply(pair.1)
}

/// Precomposition: given f: A → B, get (C^B → C^A)
pub fn precompose<A: 'static, B: 'static, C: 'static>(
    f: impl Fn(A) -> B + 'static + Clone,
) -> impl Fn(Exponential<B, C>) -> Exponential<A, C> {
    move |g| {
        let f = f.clone();
        Exponential::new(move |a| g.apply(f(a)))
    }
}

/// Postcomposition: given f: B → C, get (B^A → C^A)  
pub fn postcompose<A: 'static, B: 'static, C: 'static>(
    f: impl FnOnce(B) -> C + 'static,
) -> impl FnOnce(Exponential<A, B>) -> Exponential<A, C> {
    move |g| Exponential::new(move |a| f(g.apply(a)))
}

// Practical applications

/// Church encoding of pairs
pub type ChurchPair<A, B> = Box<dyn FnOnce(Box<dyn FnOnce(A, B) -> bool>) -> bool>;

/// Apply function to both elements
pub fn bimap<A, B, C, D>(
    p: Product<A, B>,
    f: impl FnOnce(A) -> C,
    g: impl FnOnce(B) -> D,
) -> Product<C, D> {
    Product(f(p.0), g(p.1))
}

/// Swap product elements
pub fn swap<A, B>(p: Product<A, B>) -> Product<B, A> {
    Product(p.1, p.0)
}

/// Associate products: ((A × B) × C) → (A × (B × C))
pub fn assoc_r<A, B, C>(p: Product<Product<A, B>, C>) -> Product<A, Product<B, C>> {
    Product(p.0.0, Product(p.0.1, p.1))
}

pub fn assoc_l<A, B, C>(p: Product<A, Product<B, C>>) -> Product<Product<A, B>, C> {
    Product(Product(p.0, p.1.0), p.1.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal() {
        assert_eq!(terminal(42), Terminal);
        assert_eq!(terminal("hello"), Terminal);
    }

    #[test]
    fn test_product() {
        let p = Product::new(1, "hello");
        assert_eq!(*p.fst(), 1);
        assert_eq!(*p.snd(), "hello");
    }

    #[test]
    fn test_exponential() {
        let f = Exponential::new(|x: i32| x * 2);
        assert_eq!(f.apply(21), 42);
    }

    #[test]
    fn test_curry_uncurry() {
        let add = |(a, b): (i32, i32)| a + b;
        let curried = curry(add);
        let add_5 = curried(5);
        assert_eq!(add_5.apply(3), 8);
    }

    #[test]
    fn test_product_universal() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = product_universal(f, g);
        
        let result = h(5);
        assert_eq!(result, Product(6, 10));
    }

    #[test]
    fn test_diagonal() {
        assert_eq!(diagonal(42), Product(42, 42));
    }

    #[test]
    fn test_eval() {
        let f = Exponential::new(|x: i32| x.to_string());
        let result = eval((f, 42));
        assert_eq!(result, "42");
    }

    #[test]
    fn test_bimap() {
        let p = Product(10, "hi");
        let result = bimap(p, |x| x * 2, |s| s.len());
        assert_eq!(result, Product(20, 2));
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap(Product(1, 2)), Product(2, 1));
    }

    #[test]
    fn test_assoc() {
        let p = Product(Product(1, 2), 3);
        let r = assoc_r(p);
        assert_eq!(r, Product(1, Product(2, 3)));
        
        let l = assoc_l(r);
        assert_eq!(l, Product(Product(1, 2), 3));
    }

    #[test]
    fn test_projections() {
        let p = Product(42, "hello");
        assert_eq!(fst(p.clone()), 42);
        assert_eq!(snd(p), "hello");
    }
}
