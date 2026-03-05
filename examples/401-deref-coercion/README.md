📖 **[View on hightechmind.io →](https://hightechmind.io/rust/401-deref-coercion)**

---

# 401: Deref and Deref Coercions

**Difficulty:** 2  **Level:** Intermediate

Automatic reference conversions that let smart pointers and owned types work seamlessly with borrowed slices and str.

## The Problem This Solves

You have a `String` but a function wants `&str`. You have a `Vec<T>` but a function wants `&[T]`. You have a `Box<T>` but want to call methods on `T`. Copying the data would be wasteful; manual casting is noise. Rust solves this with *Deref coercions*: automatic, zero-cost reference conversions that trigger when types don't match but have a `Deref` chain connecting them.

Deref coercions make APIs ergonomic without sacrificing safety. A function `fn greet(s: &str)` works with `&String`, `&Box<String>`, or `&Arc<String>` — all coerce automatically. Without this, every string function would need to be duplicated for every string-like type.

The coercion happens at compile time. The compiler follows the chain of `Deref` impls — `String: Deref<Target=str>`, `Box<T>: Deref<Target=T>`, `Arc<T>: Deref<Target=T>` — until the types align. No runtime cost, no allocation.

## The Intuition

Deref coercion is the compiler automatically inserting `*` dereferences until types match — like saying `&String` is "good enough" wherever `&str` is needed because `String` knows how to deref to `str`.

## How It Works in Rust

```rust
// String → str coercion (String: Deref<Target = str>)
fn greet(name: &str) { println!("Hello, {}!", name); }

let owned = String::from("Alice");
greet(&owned);          // &String coerces to &str automatically
greet(&owned[..]);      // explicit slice, same result

// Vec<T> → [T] coercion (Vec<T>: Deref<Target = [T]>)
fn sum(nums: &[i32]) -> i32 { nums.iter().sum() }

let v = vec![1, 2, 3];
sum(&v);                // &Vec<i32> coerces to &[i32]

// Box<T> → T coercion — call T's methods through Box
let boxed = Box::new(String::from("hello"));
greet(&boxed);          // Box<String> → String → str, two hops

// The chain: Box<String> → deref → String → deref → str
// Compiler inserts: &*(&*boxed) automatically
```

1. You write `&smart_pointer` where `&target` is needed.
2. Compiler checks `Deref` impls, follows the chain until types align.
3. Inserts the necessary `*` dereferences — all at compile time, zero cost.

## What This Unlocks

- **Universal API design**: Write functions accepting `&str` and `&[T]` — they work with all owning and smart-pointer types.
- **Custom smart pointers**: Implement `Deref<Target=T>` on your type and get automatic ergonomics for free.
- **Method resolution**: Calling `.len()` on a `Box<Vec<i32>>` traverses `Box → Vec → [T]` to find the method.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Automatic coercion | Subtyping (rare, mostly for polymorphism) | Deref chain coercion at compile time |
| String types | One string type (`string`) | `String` (owned) / `&str` (borrowed) — coerce automatically |
| Smart pointers | GC handles all; no manual Deref | `Box`, `Rc`, `Arc` all `Deref` to inner type |
| Zero-cost | GC overhead exists | Coercions are compile-time transformations, no runtime cost |
| Custom smart pointer ergonomics | N/A | `impl Deref for MyBox<T>` gives full coercion chain |
