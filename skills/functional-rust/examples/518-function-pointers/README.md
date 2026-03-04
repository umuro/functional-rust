# 518: Function Pointers vs Closures

**Difficulty:** 2  **Level:** Beginner-Intermediate

Two kinds of callable in Rust — know when to use each for maximum clarity and performance.

## The Problem This Solves

You write a function that accepts a callback and use `fn(i32) -> i32` as the parameter type. A user passes a closure that captures a variable — and gets a type error. Or you write `F: Fn(i32) -> i32` everywhere and then try to put function pointers in an array — and hit size issues.

Understanding when to use `fn` pointers versus `impl Fn`/`Box<dyn Fn>` closures is essential for writing ergonomic APIs. Getting it wrong either rejects valid inputs, adds unnecessary overhead, or prevents storing callables in arrays and const contexts.

There's also a performance angle: `fn` pointers are one machine word and can be stored in plain arrays without boxing. Closures may be zero-sized or large depending on what they capture, and have different inlining characteristics.

## The Intuition

A **function pointer** (`fn(i32) -> i32`) is just an address — a number pointing to a compiled function. Like a C function pointer. It can't capture state because there's nowhere to store it. But it's `Copy`, tiny (one word), and works in const contexts.

A **closure** is an anonymous struct that happens to implement `Fn`/`FnMut`/`FnOnce`. It can carry captured state (like an object with fields). Each closure is its own type. The size varies: a non-capturing closure may be zero bytes; a closure capturing a `Vec` might be 24 bytes.

The key insight: every `fn` pointer implements `Fn`, `FnMut`, and `FnOnce`. So APIs using `impl Fn` accept *both* function pointers and closures. APIs using `fn(...)` reject closures with captures.

In Python and JavaScript, all functions are objects on the heap — there's no distinction. Rust's separation gives you the zero-cost option (`fn` ptr) when you don't need capture, and the powerful option (`impl Fn`) when you do.

## How It Works in Rust

```rust
fn square(x: i32) -> i32 { x * x }
fn double(x: i32) -> i32 { x * 2 }

// fn pointer: one word, Copy, no captures
fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }
// Generic: accepts fn pointers AND closures (static dispatch)
fn apply_generic<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }

// fn pointer table: array of (name, function) pairs — no boxing needed
let ops: Vec<(&str, fn(i32) -> i32)> = vec![
    ("square", square),
    ("double", double),
    ("negate", |x| -x),   // non-capturing closure COERCES to fn ptr
];
// Non-capturing closures automatically coerce to fn pointers!

// fn pointer is Copy — can copy freely
let f: fn(i32) -> i32 = square;
let g = f;    // copied
println!("{} {}", f(3), g(3)); // both work

// Closure with capture: ONLY works with impl Fn, NOT fn ptr
let offset = 100;
let add_offset = move |x: i32| x + offset;  // captures offset
// apply_fn_ptr(add_offset, 5);  // ✗ ERROR: can't coerce capturing closure to fn ptr
println!("{}", apply_generic(add_offset, 5)); // ✓ 105

// Size comparison
println!("{}", std::mem::size_of::<fn(i32) -> i32>());  // 8 bytes (pointer)
let nc = |x: i32| x + 1;
println!("{}", std::mem::size_of_val(&nc));  // 0 bytes! (non-capturing = no state)
let cap = move |x: i32| x + offset;
println!("{}", std::mem::size_of_val(&cap)); // 4 bytes (captures one i32)
```

## What This Unlocks

- **Const-compatible callbacks** — `fn` pointers work in `const` contexts, static arrays, and FFI; closures don't.
- **FFI callbacks** — `extern "C" fn(...)` pointers are what C libraries expect; closures with captures can't be passed to C directly.
- **Table-driven programming** — arrays of `fn` pointers for dispatch tables, command parsers, and jump tables without boxing overhead.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Function pointer | `int -> int` (first class, boxed by GC) | `fn(i32) -> i32` — one word, stack |
| Closure | Same type as fn — no distinction | Anonymous type implementing `Fn*` |
| Can capture | Always | `fn` ptr: no; closure: yes |
| Size | 1 word (uniform representation) | `fn` ptr: 1 word; closure: 0..N bytes |
| Copy semantics | Yes (value semantics) | `fn` ptr: Copy; closure: depends on captures |
