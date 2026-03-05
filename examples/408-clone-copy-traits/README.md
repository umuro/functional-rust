# 408: Clone vs Copy Semantics

**Difficulty:** 2  **Level:** Intermediate

Two duplication models — implicit bitwise copy for small value types, explicit deep clone for everything else.

## The Problem This Solves

When you assign a value in Rust, ownership moves by default. After `let b = a;`, `a` is gone — moved into `b`. This is correct for heap-owning types like `String` and `Vec` where silent copying would be expensive and surprising. But for simple integers and booleans, moving doesn't make sense: they're tiny, they contain no heap resources, and copying them is what you always want.

`Copy` marks types that are safe to duplicate by bit-copying — integers, `bool`, `char`, `f64`, tuples of `Copy` types, references. After `let b = a;` on a `Copy` type, both `a` and `b` are valid. No heap involved, no destructor to worry about, just bits.

`Clone` is the explicit version. Call `.clone()` to duplicate anything that implements it. For `String`, clone allocates a new heap buffer and copies the bytes. For complex nested types, it recurses. It's always opt-in, always visible in code, and potentially expensive — which is exactly the design intent.

## The Intuition

`Copy` = assign and the original still works (implicit bitwise copy); `Clone` = call `.clone()` explicitly when you need a deliberate duplicate that may allocate.

## How It Works in Rust

```rust
// Copy types — assignment copies, original still valid
let x: i32 = 42;
let y = x;       // x is COPIED, not moved
println!("{}", x); // still valid

// Tuple of Copy types is also Copy
let point = (1.0f64, 2.0f64);
let p2 = point;  // copied
println!("{:?}", point); // still valid

// Clone — explicit, potentially expensive
#[derive(Debug, Clone)]
struct Config { name: String, values: Vec<i32> }

let cfg = Config { name: "main".to_string(), values: vec![1, 2, 3] };
let cfg2 = cfg.clone();  // allocates new String + new Vec
// cfg is still valid, cfg2 is an independent copy

// Can't be Copy if it owns heap memory
// #[derive(Copy)] on Config would FAIL — String doesn't implement Copy

// Custom Copy type — must also implement Clone
#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }

let p = Point { x: 1.0, y: 2.0 };
let p2 = p;  // copied — no .clone() needed
process(p);  // p still valid here
```

1. `Copy` is a marker trait — no methods, just signals "bitwise copy is safe."
2. `Clone` has the `.clone()` method — implement it for deep duplication.
3. `Copy` requires `Clone` (all Copy types must also be Clone).
4. Types with `Drop` implementations cannot be `Copy` (they manage resources).

## What This Unlocks

- **Ergonomic numeric code**: Pass `i32`, `f64`, `bool` freely without `clone()` noise.
- **Explicit allocation awareness**: Seeing `.clone()` in code immediately signals "potential allocation here."
- **Newtype copy semantics**: Wrap an `f64` in a newtype, derive `Copy`, and get full value semantics with type safety.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Assignment | Always copies (persistent data structures) or aliases | Moves by default, copies only for `Copy` types |
| Explicit duplication | Structural sharing (immutable), `copy` module | `.clone()` — always explicit, always visible |
| Primitive types | All immutable values, copy-on-use | `i32`, `f64`, `bool`, `char` — `Copy` |
| String | `string` is immutable, assignment is fine | `String` moves; `&str` is `Copy` |
| Resource management | GC handles it | `Copy` explicitly forbidden for types with `Drop` |
