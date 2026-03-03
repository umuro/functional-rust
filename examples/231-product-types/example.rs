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

pub fn uncurry<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn((A, B)) -> C {
    move |(a, b)| f(a, b)
}

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

pub fn distance(p: &Point2d, q: &Point2d) -> f64 {
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    (dx * dx + dy * dy).sqrt()
}

impl Point2d {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    // Record (struct) product type
    let p = Point2d { x: 0.0, y: 0.0 };
    let q = Point2d { x: 3.0, y: 4.0 };
    println!("distance (free fn) = {:.1}", distance(&p, &q));
    println!("distance (method)  = {:.1}", p.distance_to(&q));

    // Tuple product type
    let t = (42, "hello");
    println!("fst={} snd={}", fst(t), snd(t));
    println!("swap: snd of swap = {}", snd(swap((42, "hello"))));

    // pair constructor
    let constructed = pair(10, 20);
    println!("pair(10, 20) = {:?}", constructed);

    // uncurry
    let add_pair = uncurry(|a: i32, b: i32| a + b);
    println!("uncurry (+) (3,4) = {}", add_pair((3, 4)));

    // curry
    let curried_add = curry(uncurry(|a: i32, b: i32| a + b));
    println!("curry: 3 + 4 = {}", curried_add(3)(4));

    // Point3d
    let p3 = Point3d { x: 1.0, y: 2.0, z: 3.0 };
    println!("Point3d = {:?}", p3);
}

/* Output:
   distance (free fn) = 5.0
   distance (method)  = 5.0
   fst=42 snd=hello
   swap: snd of swap = 42
   pair(10, 20) = (10, 20)
   uncurry (+) (3,4) = 7
   curry: 3 + 4 = 7
   Point3d = Point3d { x: 1.0, y: 2.0, z: 3.0 }
*/

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
