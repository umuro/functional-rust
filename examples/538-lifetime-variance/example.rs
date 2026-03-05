//! # 538. Variance: Covariant, Contravariant, Invariant
//! How subtyping propagates through type constructors.

use std::marker::PhantomData;

/// COVARIANT in 'a: &'long T can be used as &'short T
/// Covariant types: &'a T, *const T, Box<T>, Vec<T>, Option<T>
fn covariant_demo() {
    let long_str: &'static str = "static";
    // &'static coerces to &'shorter — safe for reading
    let shorter: &str = long_str; // OK: 'static <: 'any
    println!("Covariant (&T): {} used as shorter-lived ref", shorter);

    // Vec<T> is covariant in T (for immutable usage)
    let v: Vec<&'static str> = vec!["hello", "world"];
    let shorter_v: Vec<&str> = v; // Vec is covariant — OK
    println!("Covariant (Vec<&T>): {:?}", shorter_v);
}

/// CONTRAVARIANT in T: fn(Animal) works where fn(Dog) is expected
/// Because fn(Animal) accepts more than needed (Dogs are Animals)
fn contravariant_demo() {
    // fn(&'short T) is contravariant in 'short:
    // A function accepting &'short T can also accept &'long T (longer = more specific)
    // This is why fn pointers are contravariant in their argument lifetime

    // In practice: you can pass a broader-accepting function to a narrower requirement
    let accepts_static: fn(&'static str) = |s| println!("static: {}", s);
    // fn(&'static str) -> more restrictive (only accepts 'static)
    // fn(&str) -> less restrictive (accepts any lifetime)
    // For function args, less restrictive = subtype

    // Demonstration via higher-order functions:
    let apply = |f: fn(&str)| f("test");
    let print_any: fn(&str) = |s| println!("any: {}", s);
    apply(print_any);
    // applies: fn accepts &str, so contravariant in the argument lifetime
    let _ = accepts_static; // suppress unused
}

/// INVARIANT in T: &'a mut T — cannot coerce lifetime
/// If mutable refs were covariant, we could violate memory safety
fn invariant_demo() {
    let mut s = String::from("hello");
    let r: &mut String = &mut s;
    // &'a mut T is invariant — r must be used exactly as &'mut String
    // Cannot shorten or lengthen the lifetime without re-borrowing
    r.push_str(" world");
    println!("Invariant (&mut T): {}", r);
    // The invariance prevents:
    // - Storing a &mut T that outlives the data
    // - Multiple &mut T at the same lifetime (aliasing)
}

/// PhantomData to declare variance for generic types
/// Covariant phantom: T can be substituted with a subtype
struct CovariantWrapper<T> {
    value: T,
    _phantom: PhantomData<T>, // covariant in T
}

/// Invariant phantom: T must match exactly
struct InvariantWrapper<T> {
    _phantom: PhantomData<fn(T) -> T>, // invariant — in both covariant and contravariant positions
}

impl<T: std::fmt::Debug> CovariantWrapper<T> {
    fn new(value: T) -> Self {
        CovariantWrapper { value, _phantom: PhantomData }
    }
    fn get(&self) -> &T { &self.value }
}

fn main() {
    println!("=== Covariant (& and Box<T>) ===");
    covariant_demo();

    println!("\n=== Contravariant (fn arguments) ===");
    contravariant_demo();

    println!("\n=== Invariant (&mut T) ===");
    invariant_demo();

    println!("\n=== PhantomData variance ===");
    let w: CovariantWrapper<&'static str> = CovariantWrapper::new("hello");
    println!("CovariantWrapper: {}", w.get());

    // Practical implication of variance:
    println!("\n=== Variance in practice ===");
    // This is why Vec<&'static str> can be used as Vec<&str> (covariant)
    let statics: Vec<&'static str> = vec!["a", "b", "c"];
    let dynamic: Vec<&str> = statics; // covariant — OK!
    println!("Coerced Vec: {:?}", dynamic);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covariant_ref() {
        let s: &'static str = "test";
        let r: &str = s; // coercion
        assert_eq!(r, "test");
    }

    #[test]
    fn test_invariant_mut_ref() {
        let mut x = 42i32;
        let r: &mut i32 = &mut x;
        *r = 100;
        drop(r);
        assert_eq!(x, 100);
    }

    #[test]
    fn test_covariant_wrapper() {
        let w = CovariantWrapper::new(42i32);
        assert_eq!(*w.get(), 42);
    }
}
