//! # Adjunction
//!
//! An adjunction is a pair of functors F and G with natural transformations:
//! - unit: Id -> G ∘ F
//! - counit: F ∘ G -> Id
//! 
//! Every adjunction gives rise to a monad (G ∘ F) and a comonad (F ∘ G).

use std::marker::PhantomData;

/// Represents an adjunction between functors F and G
/// F is left adjoint to G (F ⊣ G)
pub trait Adjunction<F, G> {
    type A;
    type B;
    
    /// unit: a -> G(F(a))
    fn unit(a: Self::A) -> G;
    
    /// counit: F(G(b)) -> b
    fn counit(fg: F) -> Self::B;
}

// Approach 1: Product-Exponential Adjunction
// (A × _) ⊣ (_ → A)
// This gives rise to the State monad!

/// Product functor: pairs with A
#[derive(Debug, Clone, PartialEq)]
pub struct Product<A, B>(pub A, pub B);

/// Exponential functor (Reader): functions from A
pub struct Exponential<A, B> {
    run: Box<dyn FnOnce(A) -> B>,
}

impl<A, B> Exponential<A, B> {
    pub fn new<F: FnOnce(A) -> B + 'static>(f: F) -> Self {
        Exponential { run: Box::new(f) }
    }
    
    pub fn run(self, a: A) -> B {
        (self.run)(a)
    }
}

/// Curry: (A × B) -> C  ≅  A -> (B -> C)
pub fn curry<A, B, C, F>(f: F) -> impl FnOnce(A) -> Box<dyn FnOnce(B) -> C>
where
    F: FnOnce(A, B) -> C + 'static,
    A: 'static,
    B: 'static,
    C: 'static,
{
    move |a| Box::new(move |b| f(a, b))
}

/// Uncurry: A -> (B -> C)  ≅  (A × B) -> C
pub fn uncurry<A, B, C, F, G>(f: F) -> impl FnOnce((A, B)) -> C
where
    F: FnOnce(A) -> G,
    G: FnOnce(B) -> C,
{
    move |(a, b)| f(a)(b)
}

// Approach 2: Free-Forgetful Adjunction
// Free ⊣ Forgetful (for monoids)

/// Free monoid over A (just Vec<A>)
pub type FreeMonoid<A> = Vec<A>;

/// Unit: A -> [A]
pub fn free_unit<A>(a: A) -> FreeMonoid<A> {
    vec![a]
}

/// Counit (fold): [A] -> A using monoid operation
pub fn free_counit<A: Clone, F>(xs: FreeMonoid<A>, empty: A, combine: F) -> A
where
    F: Fn(A, A) -> A,
{
    xs.into_iter().fold(empty, |acc, x| combine(acc, x))
}

// Approach 3: Sum-Product Adjunction (in Set)
// Δ ⊣ ×  (diagonal is left adjoint to product)

/// Diagonal functor: A -> (A, A)
pub fn diagonal<A: Clone>(a: A) -> (A, A) {
    (a.clone(), a)
}

/// Project first
pub fn fst<A, B>(pair: (A, B)) -> A {
    pair.0
}

/// Project second
pub fn snd<A, B>(pair: (A, B)) -> B {
    pair.1
}

/// State monad from Product-Exponential adjunction
#[derive(Clone)]
pub struct State<S, A> {
    run: std::sync::Arc<dyn Fn(S) -> (A, S) + Send + Sync>,
}

impl<S: Clone + 'static, A: Clone + 'static> State<S, A> {
    pub fn new<F: Fn(S) -> (A, S) + Send + Sync + 'static>(f: F) -> Self {
        State { run: std::sync::Arc::new(f) }
    }
    
    pub fn run(&self, s: S) -> (A, S) {
        (self.run)(s)
    }
    
    pub fn pure(a: A) -> Self {
        State::new(move |s| (a.clone(), s))
    }
    
    pub fn get() -> State<S, S> {
        State::new(|s: S| (s.clone(), s))
    }
    
    pub fn put(s: S) -> State<S, ()> {
        State::new(move |_| ((), s.clone()))
    }
    
    pub fn modify<F: Fn(S) -> S + Send + Sync + Clone + 'static>(f: F) -> State<S, ()> {
        State::new(move |s| ((), f(s)))
    }
    
    pub fn map<B: Clone + 'static, F: Fn(A) -> B + Send + Sync + Clone + 'static>(self, f: F) -> State<S, B> {
        let run = self.run;
        State::new(move |s| {
            let (a, s2) = run(s);
            (f(a), s2)
        })
    }
    
    pub fn flat_map<B: Clone + 'static, F>(self, f: F) -> State<S, B>
    where
        F: Fn(A) -> State<S, B> + Send + Sync + Clone + 'static,
    {
        let run = self.run;
        State::new(move |s| {
            let (a, s2) = run(s);
            f(a).run(s2)
        })
    }
}

/// Costate (Store) comonad from the same adjunction
#[derive(Clone)]
pub struct Store<S, A> {
    peek: std::sync::Arc<dyn Fn(S) -> A + Send + Sync>,
    pos: S,
}

impl<S: Clone + 'static, A: Clone + 'static> Store<S, A> {
    pub fn new<F: Fn(S) -> A + Send + Sync + 'static>(f: F, pos: S) -> Self {
        Store { peek: std::sync::Arc::new(f), pos }
    }
    
    pub fn extract(&self) -> A {
        (self.peek)(self.pos.clone())
    }
    
    pub fn extend<B: Clone + 'static, F>(self, f: F) -> Store<S, B>
    where
        F: Fn(&Store<S, A>) -> B + Send + Sync + 'static,
    {
        let peek = self.peek.clone();
        let pos = self.pos.clone();
        Store::new(
            move |s| {
                let store = Store { peek: peek.clone(), pos: s };
                f(&store)
            },
            pos
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curry_uncurry() {
        let add = |a: i32, b: i32| a + b;
        let curried = curry(add);
        let add_5 = curried(5);
        assert_eq!(add_5(3), 8);
    }

    #[test]
    fn test_free_monoid_unit() {
        let xs = free_unit(42);
        assert_eq!(xs, vec![42]);
    }

    #[test]
    fn test_free_monoid_counit() {
        let xs = vec![1, 2, 3, 4];
        let sum = free_counit(xs, 0, |a, b| a + b);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_diagonal() {
        let (a, b) = diagonal(5);
        assert_eq!(a, 5);
        assert_eq!(b, 5);
    }

    #[test]
    fn test_state_pure() {
        let s: State<i32, &str> = State::pure("hello");
        let (a, state) = s.run(100);
        assert_eq!(a, "hello");
        assert_eq!(state, 100);
    }

    #[test]
    fn test_state_get() {
        let s: State<i32, i32> = State::get();
        let (a, state) = s.run(42);
        assert_eq!(a, 42);
        assert_eq!(state, 42);
    }

    #[test]
    fn test_state_put() {
        let s: State<i32, ()> = State::put(100);
        let (_, state) = s.run(42);
        assert_eq!(state, 100);
    }

    #[test]
    fn test_state_modify() {
        let s: State<i32, ()> = State::modify(|x| x * 2);
        let (_, state) = s.run(21);
        assert_eq!(state, 42);
    }

    #[test]
    fn test_state_map() {
        let s = State::pure(5).map(|x| x * 2);
        let (a, _) = s.run(0);
        assert_eq!(a, 10);
    }

    #[test]
    fn test_state_flat_map() {
        let s = State::get().flat_map(|x: i32| State::pure(x * 2));
        let (a, state) = s.run(21);
        assert_eq!(a, 42);
        assert_eq!(state, 21);
    }

    #[test]
    fn test_store_extract() {
        let store = Store::new(|x: i32| x * 2, 5);
        assert_eq!(store.extract(), 10);
    }

    #[test]
    fn test_store_extend() {
        let store = Store::new(|x: i32| x * 2, 5);
        let store2 = store.extend(|s| s.extract() + 1);
        assert_eq!(store2.extract(), 11);
    }
}
