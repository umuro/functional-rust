# 370: SmallVec / Inline Storage Optimization

**Difficulty:** 3  **Level:** Advanced

Store small collections on the stack; spill to the heap only when they grow beyond a threshold.

## The Problem This Solves

`Vec<T>` always heap-allocates, even for one element. If your typical collection has 0–4 items and only rarely grows larger — function argument lists, SSA use-def chains, adjacency lists for sparse graphs, small sets of tags — you're paying a heap allocation on every single instance. At scale, this hammers the allocator and kills cache performance.

`SmallVec<[T; N]>` from the `smallvec` crate stores up to N elements inline (on the stack or embedded in a larger struct), with no heap allocation. When the collection grows beyond N, it transparently spills to a heap-allocated buffer, just like `Vec`. Most callers never see the difference — the API is nearly identical to `Vec`.

The compiler itself uses this pattern heavily: rustc's `SmallVec` is used for lists that are almost always short, like the list of where-clause predicates on a generic parameter.

## The Intuition

`SmallVec<[T; N]>` contains an enum internally: either an inline array of N elements with a length counter, or a heap-allocated `Vec<T>` when full. The size of a `SmallVec<[T; 4]>` is `4 * size_of::<T>() + 8 bytes` — versus `Vec<T>` which is always 24 bytes of pointer + length + capacity regardless of how many elements you store.

The sweet spot: N should be your p90 collection size. Too small and you always spill. Too large and you waste stack space. Profile first, then tune.

## How It Works in Rust

```rust
use smallvec::{SmallVec, smallvec};

// Stores up to 4 elements on stack; spills to heap if more
let mut args: SmallVec<[&str; 4]> = smallvec!["--verbose", "--output"];

// Works like Vec
args.push("file.txt");
args.push("--color");

println!("args: {:?}", args);          // still on stack
println!("spilled: {}", args.spilled()); // false

args.push("--extra");
println!("spilled: {}", args.spilled()); // true — now on heap

// Nearly identical API to Vec
for arg in &args {
    print!("{} ", arg);
}
```

Add to `Cargo.toml`: `smallvec = "1"`

## What This Unlocks

- **Compiler/interpreter hot paths** — argument lists, IR operands, and scope chains are nearly always small.
- **Graph adjacency lists** — most nodes in real-world graphs have few neighbors; avoid per-node heap allocation.
- **Parser combinators** — accumulated match lists stay on the stack for typical inputs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Small list | `list` (heap cons cells always) | `SmallVec<[T; N]>` (stack until overflow) |
| Array vs list | `Array.t` (fixed-size heap) | `[T; N]` (stack, fixed) or `SmallVec` (stack, dynamic) |
| Inline storage | No equivalent — GC handles it | `SmallVec` — explicit stack/heap tradeoff |
| API compatibility | N/A | `SmallVec` implements `Deref<Target=[T]>` — works with slice APIs |
