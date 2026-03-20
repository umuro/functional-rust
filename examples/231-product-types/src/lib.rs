#![allow(clippy::all)]
// Product types: combine multiple types into one.
// In category theory, the categorical product.

use std::rc::Rc;

// --- Record product types (structs in Rust) ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// --- Tuple operations ---

// Solution 1: Idiomatic — swap, fst, snd as free functions over generic tuples
pub fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}

pub fn fst<A, B>(pair: (A, B)) -> A {
    pair.0
}

pub fn snd<A, B>(pair: (A, B)) -> B {
    pair.1
}

pub fn pair<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

// --- Curry / Uncurry ---

// Solution 2: Functional — uncurry converts a two-arg function into a tuple-arg function
// OCaml: let uncurry f (a, b) = f a b
pub fn uncurry<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn((A, B)) -> C {
    move |(a, b)| f(a, b)
}

// Solution 2b: curry converts a tuple-arg function into a two-step curried function.
// Uses Rc for shared ownership of the inner function across calls.
// OCaml: let curry f a b = f (a, b)
pub fn curry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl Fn((A, B)) -> C + 'static,
) -> impl Fn(A) -> Box<dyn Fn(B) -> C> {
    let f = Rc::new(f);
    move |a: A| {
        let f = Rc::clone(&f);
        let a = a.clone();
        Box::new(move |b: B| f((a.clone(), b)))
    }
}

// --- Distance ---

// Solution 1: Free function (matches OCaml style)
pub fn distance(p: &Point2d, q: &Point2d) -> f64 {
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    (dx * dx + dy * dy).sqrt()
}

// Solution 2: Method style (idiomatic Rust — groups behaviour with the type)
impl Point2d {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        assert_eq!(swap((1, "hello")), ("hello", 1));
        assert_eq!(swap((true, 42u8)), (42u8, true));
    }

    #[test]
    fn test_fst_snd() {
        assert_eq!(fst((42, "hello")), 42);
        assert_eq!(snd((42, "hello")), "hello");
    }

    #[test]
    fn test_pair_constructor() {
        assert_eq!(pair(1, 2), (1, 2));
        assert_eq!(pair("a", true), ("a", true));
    }

    #[test]
    fn test_uncurry() {
        let add_pair = uncurry(|a: i32, b: i32| a + b);
        assert_eq!(add_pair((3, 4)), 7);
        assert_eq!(add_pair((0, 0)), 0);
        assert_eq!(add_pair((-1, 1)), 0);
    }

    #[test]
    fn test_curry_roundtrip() {
        let add_pair = uncurry(|a: i32, b: i32| a + b);
        let curried = curry(add_pair);
        assert_eq!(curried(3)(4), 7);
        assert_eq!(curried(10)(5), 15);
        assert_eq!(curried(0)(0), 0);
    }

    #[test]
    fn test_distance_free_fn() {
        let origin = Point2d { x: 0.0, y: 0.0 };
        let p = Point2d { x: 3.0, y: 4.0 };
        assert!((distance(&origin, &p) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_distance_zero() {
        let p = Point2d { x: 1.0, y: 2.0 };
        assert_eq!(distance(&p, &p), 0.0);
    }

    #[test]
    fn test_distance_method() {
        let origin = Point2d { x: 0.0, y: 0.0 };
        let p = Point2d { x: 3.0, y: 4.0 };
        assert!((origin.distance_to(&p) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_point3d_fields() {
        let p = Point3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }
}
