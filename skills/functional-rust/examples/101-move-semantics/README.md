# 101: Move Semantics

**Difficulty:** 2  **Level:** Intermediate

When you pass a value to a function in Rust, ownership transfers — the original is gone.

## The Problem This Solves

In C, you can pass a pointer to a function, free the memory inside, and then use the original pointer again. This is a **use-after-free** bug — one of the most dangerous vulnerabilities in systems programming. The compiler says nothing. Your program corrupts memory silently, or crashes unpredictably, or opens a security hole.

Python and most other languages sidestep this with garbage collection: every value is reference-counted or traced, so the memory is never freed while someone still holds a pointer to it. But GC has a cost: pauses, overhead, and the runtime can't know at compile time whether you'll use a value again.

Rust takes a third path. It tracks ownership statically — at compile time. Every value has exactly one owner. When that owner goes out of scope, the value is dropped. When you pass a value to a function, ownership transfers: the original binding becomes invalid. The compiler proves — without a GC, without runtime checks — that use-after-free is impossible.

## The Intuition

Every value has exactly one owner; passing a value to a function *gives away* that ownership, making the original variable invalid — like handing someone your only key.

This isn't arbitrary. If there's only ever one owner, the compiler knows exactly when to free memory: when the owner's scope ends. No GC needed. No runtime. No surprises.

## How It Works in Rust

```rust
// ERROR: value used after move
fn broken() {
    let name = String::from("Alice");
    let len = use_string(name); // ownership moves here
    println!("{}", name);       // ERROR: borrow of moved value: `name`
}

// FIX 1: clone before passing (explicit copy)
fn with_clone() {
    let name = String::from("Alice");
    let len = use_string(name.clone()); // clone stays, original moves
    let len2 = use_string(name);        // now name moves here — fine
}

// FIX 2: pass a reference instead (borrow, don't move)
fn use_string_ref(s: &String) -> usize {
    s.len() // s is borrowed, not owned — caller keeps it
}

fn with_borrow() {
    let name = String::from("Alice");
    let len = use_string_ref(&name); // borrow: name is still valid
    println!("{} has {} chars", name, len); // works!
}

// Copy types don't move — they're silently duplicated
fn copy_demo() {
    let x: i32 = 42;
    let y = x;        // Copy, not move — both remain valid
    println!("{} {}", x, y); // fine
}
```

The rule: heap-allocated types (`String`, `Vec`, structs with those fields) **move**. Stack-only types (`i32`, `bool`, `f64`, `char`) **copy** silently.

## What This Unlocks

- **No garbage collector** — memory is freed exactly when the owner's scope ends, with zero runtime overhead.
- **No use-after-free bugs** — the compiler rejects them before your code ever runs.
- **Fearless refactoring** — if it compiles, ownership is sound; you can restructure code without introducing memory bugs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Memory management | GC (reference counting + tracing) | Ownership (compile-time) |
| Passing a value | Shares a GC pointer — original stays valid | Moves ownership — original invalid |
| Reuse after pass | Always works | Only if `Copy` or explicitly `.clone()`d |
| "Use after move" | Impossible concept | Compile error |
| Runtime cost | GC pauses and overhead | Zero-cost (no GC) |
