# 102: Clone and Copy

**Difficulty:** 2  **Level:** Intermediate

Copy types are silently duplicated when passed; Clone types require an explicit `.clone()` — making expensive copies visible.

## The Problem This Solves

Languages with GC (Python, Java, OCaml) make all copies implicit. You never think about whether copying is cheap or expensive — the runtime handles it. This is convenient, but it hides costs. A Python function that receives a list and "accidentally" modifies it can surprise the caller. A Java method that stores a reference to your object can mutate it later without your knowledge.

Rust separates two fundamentally different things: cheap stack copies (integers, booleans — a handful of bytes, trivially duplicated) and expensive heap copies (strings, vectors — arbitrary amounts of data that require allocating new memory). The `Copy` trait marks the first kind. The `Clone` trait handles the second — and it requires you to call `.clone()` explicitly, making the cost visible in your code.

This design means: when you see `.clone()` in Rust code, you know a potentially expensive heap allocation is happening. When you don't see it, you know the copy is cheap. No hidden costs.

## The Intuition

Types that fit entirely on the stack (`Copy`) are silently duplicated when passed; types that own heap data (`Clone`) require explicit `.clone()` because the compiler won't hide an expensive allocation from you.

## How It Works in Rust

```rust
// Copy types: assignment and passing create silent copies
let x: i32 = 42;
let y = x;          // copy, not move — x is still valid
println!("{}", x);  // works fine

// This applies to: i8..i128, u8..u128, f32, f64, bool, char,
// tuples/arrays of Copy types, raw pointers, references

// Clone types: heap data requires explicit .clone()
let s1 = String::from("hello");
// let s2 = s1;     // MOVE — s1 is gone
let s2 = s1.clone(); // CLONE — s1 is still valid, s2 is a fresh copy
println!("{} {}", s1, s2); // both work

// Structs: all fields must be Copy to derive Copy
#[derive(Copy, Clone)]
struct Point { x: f64, y: f64 }  // f64 is Copy → Point can be Copy

// This can't be Copy — String is not Copy
#[derive(Clone)]
struct Person { name: String, age: u32 }
// Can only derive Clone, not Copy

let p1 = Person { name: "Alice".to_string(), age: 30 };
let p2 = p1.clone(); // explicit deep copy — allocates new String
// let p3 = p1;      // would be a MOVE, not copy

// Vec<T> is Clone (not Copy) — cloning copies all elements
let v1 = vec![1, 2, 3];
let v2 = v1.clone(); // new allocation, copies all elements
```

## What This Unlocks

- **No hidden allocation costs** — every heap copy is explicit; `.clone()` in code review immediately signals "this allocates."
- **Predictable performance** — `Copy` types have zero overhead when passed to functions; you never accidentally O(n)-copy a large structure.
- **Composable safety** — structs automatically become `Copy` when all their fields are, giving you value semantics for free on plain-data types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Copying primitives | Implicit (GC pointer or bitwise) | Implicit via `Copy` trait |
| Copying heap data | Implicit (GC manages sharing) | Explicit `.clone()` |
| Custom struct copy | Shallow copy via `{...with ...}` or GC-shared | Must derive `Copy` (all fields must be `Copy`) |
| Visibility of cost | Hidden by GC | Visible — `.clone()` = allocation |
| Sharing by default | Yes (GC refs) | No — ownership is exclusive |
