📖 **[View on hightechmind.io →](https://hightechmind.io/rust/552-lifetime-arena-pattern)**

---

# Arena Allocation Pattern

## Problem Statement

Arena allocation (also called region-based memory management) is a technique where all allocations live in a single region and are freed all at once when the region is dropped. This is dramatically faster than individual heap allocations: no per-object malloc overhead, excellent cache locality, and zero fragmentation. Compilers (LLVM, GCC), game engines, and web browsers use arenas for AST nodes, parse results, and per-frame allocations. In Rust, arenas elegantly tie allocated objects to the arena's lifetime, preventing use-after-arena-drop at compile time.

## Learning Outcomes

- How `StringArena` stores all strings in a `Vec<String>` and returns `&str` references tied to the arena's lifetime
- Why arena-allocated references cannot outlive the arena (lifetime enforcement)
- How arenas eliminate per-object allocation overhead for batch workloads
- How the `bumpalo` crate implements a production arena allocator
- Where arenas are used: compilers (LLVM), parsers (nom ASTs), game engines, HTTP request processing

## Rust Application

`StringArena` stores strings in `storage: Vec<String>` and `alloc(&mut self, s: &str) -> &str` pushes a new `String` and returns a `&str` reference to it — the reference is valid as long as the arena exists and no more strings are pushed (which would reallocate the `Vec`). In practice, production arenas use `typed_arena::Arena<T>` or `bumpalo::Bump` which use bump allocation into pre-allocated slabs to avoid reallocation invalidating references.

Key patterns:
- `alloc(&mut self, ...) -> &str` — arena-tied lifetime via `&self` lifetime
- `Vec<String>` as backing store — simple implementation, push-only
- Production: `bumpalo::Bump` allocator, `typed_arena::Arena<T>`

## OCaml Approach

OCaml's GC naturally acts as an arena — objects allocated during processing are collected together when they become unreachable. For performance-sensitive arenas, OCaml uses `Bigarray` or manual memory management via `Bigarray.Array1`:

```ocaml
(* OCaml GC is effectively an automatic arena *)
let parse_and_process input =
  let ast = parse input in  (* AST nodes live until parse_and_process returns *)
  analyze ast
  (* ast and all its nodes are GC-collected after this *)
```

Explicit arenas in OCaml typically use `Buffer.t` for strings or custom allocators for performance.

## Key Differences

1. **Lifetime enforcement**: Rust arena references are tied to the arena's lifetime at compile time — use-after-free is impossible; OCaml relies on the GC to prevent this at runtime.
2. **Batch free**: Rust arenas (like `bumpalo`) support O(1) batch deallocation by dropping the arena; OCaml's GC defers collection until pressure warrants it.
3. **Cache performance**: Bumpalo's bump allocation keeps objects contiguous in memory; OCaml's copying GC (in its minor heap) achieves similar cache locality for young objects.
4. **Vec reallocation hazard**: The simple `StringArena` implementation can invalidate references if the `Vec` reallocates — production Rust arenas use slab allocation to prevent this.

## Exercises

1. **Fixed-capacity arena**: Implement an arena that pre-allocates a `Vec<String>` with a fixed capacity and returns `Err` when full, ensuring no reallocation can invalidate references.
2. **AST arena**: Build a simple arena for AST nodes using `Vec<Box<AstNode>>` where each node is a heap allocation whose reference is valid for the arena's lifetime.
3. **Benchmark comparison**: Compare allocating 10,000 strings with (a) individual `String::from`, (b) a simple arena, and (c) direct `Vec<String>` — measure and explain the performance differences.
