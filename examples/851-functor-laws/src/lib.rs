#![allow(clippy::all)]
// Example 052: Functor Laws
// Law 1 (Identity): map(id, x) == x
// Law 2 (Composition): map(f∘g, x) == map(f, map(g, x))

#[derive(Debug, PartialEq, Clone)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(f(x)),
        }
    }
}

// Approach 1: Verify laws for Maybe
fn identity<T>(x: T) -> T {
    x
}

fn verify_identity_law(x: Maybe<i32>) -> bool {
    x.clone().map(identity) == x
}

fn verify_composition_law(f: fn(i32) -> i32, g: fn(i32) -> i32, x: Maybe<i32>) -> bool {
    let composed = x.clone().map(|v| f(g(v)));
    let chained = x.map(g).map(f);
    composed == chained
}

// Approach 2: Verify laws for Vec (Rust's list)
fn vec_identity_law(xs: Vec<i32>) -> bool {
    let original = xs.clone();
    let mapped: Vec<i32> = xs.into_iter().map(identity).collect();
    mapped == original
}

fn vec_composition_law(f: fn(i32) -> i32, g: fn(i32) -> i32, xs: Vec<i32>) -> bool {
    let composed: Vec<i32> = xs.clone().into_iter().map(|x| f(g(x))).collect();
    let chained: Vec<i32> = xs.into_iter().map(g).map(f).collect();
    composed == chained
}

// Approach 3: Bad functor that breaks laws
#[derive(Debug, PartialEq, Clone)]
struct BadFunctor<T> {
    value: T,
    map_count: usize,
}

impl<T> BadFunctor<T> {
    fn new(value: T) -> Self {
        BadFunctor {
            value,
            map_count: 0,
        }
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BadFunctor<U> {
        BadFunctor {
            value: f(self.value),
            map_count: self.map_count + 1, // Breaks identity law!
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_law_just() {
        assert!(verify_identity_law(Maybe::Just(42)));
    }

    #[test]
    fn test_identity_law_nothing() {
        assert!(verify_identity_law(Maybe::Nothing));
    }

    #[test]
    fn test_composition_law_just() {
        let f: fn(i32) -> i32 = |x| x * 2;
        let g: fn(i32) -> i32 = |x| x + 3;
        assert!(verify_composition_law(f, g, Maybe::Just(5)));
    }

    #[test]
    fn test_composition_law_nothing() {
        let f: fn(i32) -> i32 = |x| x * 2;
        let g: fn(i32) -> i32 = |x| x + 3;
        assert!(verify_composition_law(f, g, Maybe::Nothing));
    }

    #[test]
    fn test_vec_identity_law() {
        assert!(vec_identity_law(vec![1, 2, 3]));
        assert!(vec_identity_law(vec![]));
    }

    #[test]
    fn test_vec_composition_law() {
        let f: fn(i32) -> i32 = |x| x * 2;
        let g: fn(i32) -> i32 = |x| x + 3;
        assert!(vec_composition_law(f, g, vec![1, 2, 3]));
    }

    #[test]
    fn test_bad_functor_breaks_identity() {
        let bad = BadFunctor::new(42);
        let mapped = bad.clone().map(identity);
        assert_ne!(mapped, bad); // Identity law violated!
    }

    #[test]
    fn test_bad_functor_breaks_composition() {
        let f: fn(i32) -> i32 = |x| x * 2;
        let g: fn(i32) -> i32 = |x| x + 3;
        let bad = BadFunctor::new(5);
        let composed = bad.clone().map(|x| f(g(x)));
        let chained = bad.map(g).map(f);
        // map_count differs: composed=1, chained=2
        assert_ne!(composed, chained);
    }
}
