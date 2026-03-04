# 137: Rank-2 Types

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Pass a function that must work for *any* type — not a specific one chosen by the caller — so a single function can apply it to multiple different types in sequence.

## The Problem This Solves

You want to pass a polymorphic function — one that works for any type `T` — as an argument. The trouble is that in a normal generic function, the caller chooses `T`. If you write `fn use_fn<T, F: Fn(T) -> T>(f: F, x: T)`, the caller picks one specific `T` and `f` only works for that type. You can't use `f` on an `i32` and then on a `String` in the same call.

This is the difference between rank-1 and rank-2 polymorphism. Rank-1: the caller chooses the type parameter. Rank-2: the *function* gets to choose the type parameter for each internal use. A rank-2 identity function can apply itself to an `i32`, then to a `String`, within a single call.

Rust doesn't natively support rank-2 types at the function level, but you can simulate them using traits. A trait with a generic method is effectively a rank-2 type: the method is polymorphic, and the caller of the *outer* function is forced to provide something that implements the trait — which means it must work for all `T` the method might be called with.

## The Intuition

The key insight: a trait with a generic method is a "for all T" contract. When you require `f: &dyn IdFn` where `trait IdFn { fn apply<T>(&self, x: T) -> T; }`, you're saying "f must work for *every* type T." The type parameter `T` is universally quantified inside the trait.

This is unlike a closure `impl Fn(i32) -> i32` where `T` is fixed to `i32`. The trait version lets you call `f.apply(42)` (T=i32) and `f.apply("hello".to_string())` (T=String) in the same function body — because the trait guarantees f handles any T.

## How It Works in Rust

```rust
// Rank-2 identity via trait — the method is generic over T
trait IdFn {
    fn apply<T>(&self, x: T) -> T;  // must work for ALL T, not one specific T
}

struct Identity;
impl IdFn for Identity {
    fn apply<T>(&self, x: T) -> T { x }  // works for any T
}

// This function takes a rank-2 function (polymorphic identity)
// and uses it with TWO different types — only possible because of the trait
fn apply_id(f: &dyn IdFn) -> (i32, String) {
    let x = f.apply(42);                      // T = i32
    let y = f.apply("hello".to_string());     // T = String
    (x, y)  // both results from the SAME f
}

let (i, s) = apply_id(&Identity);
// i = 42, s = "hello" — one call to apply_id used f twice with different types
```

More general rank-2 — a "show anything" function:
```rust
trait ForAll {
    fn call<T: std::fmt::Debug + Clone>(&self, val: T) -> String;
}

struct ShowIt;
impl ForAll for ShowIt {
    fn call<T: std::fmt::Debug + Clone>(&self, val: T) -> String {
        format!("{:?}", val)
    }
}

fn apply_forall(f: &dyn ForAll) -> Vec<String> {
    vec![
        f.call(42),          // T = i32
        f.call("hello"),     // T = &str
        f.call(vec![1,2,3]), // T = Vec<i32>
    ]
}
// ["42", "\"hello\"", "[1, 2, 3]"]
```

Polymorphic transformation:
```rust
trait Transform {
    fn transform<T: Clone + std::ops::Add<Output = T>>(&self, x: T) -> T;
}

struct Double;
impl Transform for Double {
    fn transform<T: Clone + std::ops::Add<Output = T>>(&self, x: T) -> T {
        x.clone() + x   // works for i32, f64, or any addable type
    }
}

fn apply_transform(t: &dyn Transform) -> (i32, f64) {
    (t.transform(21), t.transform(1.5))  // same t, different types
}
// (42, 3.0)
```

## What This Unlocks

- **Safe resource encapsulation** — the Haskell `ST` monad pattern uses rank-2 types to prevent mutable references from escaping a local scope; the trait-based simulation achieves the same in Rust.
- **Generic test fixtures** — pass a "serialize then deserialize" roundtrip function that must work for any type implementing your codec trait.
- **Universal callbacks** — logging or tracing systems that receive a "format this value" callback that must handle any printable type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Rank-2 syntax | Record field: `type id_fn = { id : 'a. 'a -> 'a }` | Trait with generic method: `trait IdFn { fn apply<T>(&self, x: T) -> T; }` |
| Usage | `let (x, y) = apply_id { id = fun x -> x }` | `apply_id(&Identity)` where `Identity: IdFn` |
| Type constraint on T | `'a.` quantifier in record type | Trait bounds on the method's `T`: `fn apply<T: Debug>` |
| Limitation | Native: any type can be rank-2 function | Via trait only: closures can't be rank-2 in Rust |
