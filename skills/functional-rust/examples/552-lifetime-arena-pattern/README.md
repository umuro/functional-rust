# 552: Arena with Lifetimes

**Difficulty:** 4  **Level:** Advanced

Allocate many objects with a single lifetime and free them all at once.

## The Problem This Solves

When you're building a compiler, game engine, or parser, you often create thousands of small objects (AST nodes, entities, tokens) that all live until a specific phase ends. Individually allocating and dropping each one is slow and complicates ownership. You'd also like to give out references to these objects that the compiler can verify are valid as long as the arena is alive.

An arena allocator solves this: bump-allocate into a backing store, hand out references with the arena's lifetime, and drop everything at once when the arena goes out of scope. There's no individual deallocation. The lifetime of every object in the arena is tied to the lifetime of the arena itself.

In Rust, the borrow checker makes this particularly elegant: references into the arena carry the arena's lifetime `'a`, so the compiler statically proves they can't outlive the arena. If you keep objects separate (e.g., store indices rather than references), you avoid the borrow-check complexity of mutating the arena while holding references into it.

## The Intuition

Think of an arena like a scratch pad. You write things on it. You can hand out addresses of things you wrote. When you're done with the whole session, you throw the pad away — in one move. You never erase individual entries. The address is valid as long as the pad exists.

The index-based approach (storing positions instead of raw references) sidesteps the Rust rule that you can't mutate a container while holding references into it. Allocate first, then read — or just use indices and never hold references across mutations.

## How It Works in Rust

1. **Index-based arena** — `alloc(&mut self, value: T) -> usize` pushes into a `Vec`, returns the index; `get(&self, idx: usize) -> Option<&T>` returns a reference tied to `&self`.
2. **Lifetime-tied references** — `fn alloc_and_get(&mut self, value: T) -> &T` borrows `&mut self` and returns `&T` with the arena's lifetime; valid as long as `&self` is not mutably borrowed again.
3. **AST slices from source** — `AstNode<'src>` borrows `&'src str` from the source string; the arena (a `Vec`) and the source can have independent lifetimes, both tracked by the compiler.
4. **Batch free** — when the arena drops, all its allocations drop simultaneously; no individual `drop` calls needed.
5. **Borrow discipline** — to read many items simultaneously after allocation, finish all mutations first, then take references: the borrow checker enforces this ordering.

## What This Unlocks

- Build compilers and parsers where all AST nodes share a phase lifetime without per-node allocation.
- Avoid the performance cost of many small heap allocations in tight inner loops.
- Get static proof that no object outlives the phase that owns the arena.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Object lifetimes | GC; individual objects freed lazily | Arenas: explicit phase lifetime; freed in bulk |
| References into collections | Safe; GC prevents dangling | Must borrow arena immutably to get refs; can't mutate while holding them |
| AST node lifetimes | GC-backed; no annotation | Lifetime parameter `'src` ties node slices to source string |
| Bump allocation | Libraries exist; GC still manages | Index-based arena avoids borrow conflicts; `typed-arena` crate for ref-based |
