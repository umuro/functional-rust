# 363: Arena / Bump Allocation Pattern

**Difficulty:** 4  **Level:** Expert

Allocate many objects into a single memory region and free them all at once — bypassing individual drop overhead.

## The Problem This Solves

Rust's ownership model is precise: every value is dropped exactly when its owner goes out of scope. For long-lived programs with millions of small allocations, this is usually fine. But for batch-processing workloads — parsing a file, compiling source code, running a game tick — paying per-object allocation and deallocation overhead adds up.

Compilers are the canonical example. During a parse pass, you allocate thousands of AST nodes. All of them share the same lifetime: the lifetime of the parse. When parsing is done, you want to free all of them at once, not walk a tree and drop each node individually. An arena (also called a "bump allocator" or "region allocator") does exactly this.

The pattern appears everywhere performance-critical batch allocation is needed: game ECS systems that reset each frame, query planners that build and discard execution trees, web servers that allocate per-request and reset at response time.

## The Intuition

An arena is a big memory slab with a bump pointer. To allocate an object: check if it fits, advance the pointer, return a reference. That's it. Deallocation is free — you just reset the pointer to the start. No per-object bookkeeping, no linked free lists, no GC pressure.

The constraint: everything allocated in the arena has the same lifetime. You can't free individual objects — you free the whole arena. This sounds limiting but maps perfectly onto "build a thing, use it, throw it all away."

Rust makes this safe via lifetime annotations. The `typed-arena` and `bumpalo` crates attach the arena's lifetime to every returned reference, so the borrow checker enforces that you can't use arena memory after the arena is dropped.

## How It Works in Rust

```rust
use typed_arena::Arena;

struct Node<'a> {
    value: i32,
    next: Option<&'a Node<'a>>,
}

let arena = Arena::new();

// Allocate — returns &'arena Node, not Box<Node>
let a = arena.alloc(Node { value: 1, next: None });
let b = arena.alloc(Node { value: 2, next: Some(a) });

// Use the linked structure
println!("{}", b.value); // 2
println!("{}", b.next.unwrap().value); // 1

// Drop arena → all nodes freed at once, no individual drops
```

For untyped bump allocation with `bumpalo`:
```rust
let bump = bumpalo::Bump::new();
let x: &mut i32 = bump.alloc(42);
let s: &str = bump.alloc_str("hello");
```

## What This Unlocks

- **Compiler/parser design** — AST nodes allocated into an arena, freed in one shot after code generation.
- **Game ECS frames** — allocate all frame state into a bump arena, reset at frame boundary.
- **Zero-fragmentation allocation** — linear packing means no heap fragmentation ever.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Memory management | GC (mark-and-sweep or minor/major heap) | Ownership + drop, or arena for batch |
| Arena equivalent | Minor heap (GC manages short-lived objects) | `typed_arena::Arena` or `bumpalo::Bump` |
| Object lifetime | GC-determined | Tied to arena's Rust lifetime (`'arena`) |
| Free all at once | GC major collection | `drop(arena)` — O(1) |
