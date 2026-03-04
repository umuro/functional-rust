# 516: Complex Closure Environments

**Difficulty:** 3  **Level:** Intermediate

Closures that capture structs, collections, and other closures as their environment.

## The Problem This Solves

In simple examples, closures capture one or two primitives. Real-world closures often capture much richer state: a configuration struct, a `HashMap` for lookup, a `Vec` with an index for cycling, or another closure to pass along. Understanding how Rust handles these complex captures — including what gets moved versus borrowed, and how nested closures compose — is essential for building formatters, pipelines, factories, and middleware chains.

The question is always: what does this closure *own*? When you `move` a `Config` struct into a closure, the closure owns everything in that struct, including its `Box<dyn Fn>` field. When you capture a `HashMap` by move, the closure becomes a lookup function that owns its data. When a closure captures another closure, you get higher-order composition with no external state.

## The Intuition

A closure's environment is like a small struct the compiler generates for you. Every captured variable becomes a field in that struct. With `move`, the fields contain owned values. Without `move`, they contain references. The closure *is* that struct plus a `call` method.

When you write a closure that captures another closure, you're nesting structs — composition by value. The outer closure's environment holds the inner closure, and so on. This is how pipeline stages, middleware chains, and transformer factories work in Rust.

## How It Works in Rust

1. **`move` for ownership** — `move |...| { ... }` transfers ownership of all captured variables into the closure; the original binding is consumed.
2. **Complex struct capture** — capturing a `Config` (with a `Box<dyn Fn>` field) works fine; the closure owns the entire config, including its closure field.
3. **`Vec` + index cycler** — capture a `Vec<T>` by move and a mutable `index: usize`; the `FnMut` closure increments the index on each call, wrapping via `%`.
4. **Nested closure factory** — a closure returning a `Box<dyn Fn>` captures a local variable; the returned closure captures from the outer closure's environment.
5. **Pipeline via captured `next`** — each step captures an optional `Box<dyn Fn(i32) -> i32>` as `next`; calling it chains to the next stage naturally.

## What This Unlocks

- Build configurable formatters, validators, and transformers that own their configuration.
- Implement cyclic iterators and round-robin selectors with no external state.
- Compose multi-stage pipelines where each stage is a closure holding the next stage.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Closure capture | By reference (closures close over mutable refs) | `move` for ownership; implicit borrow otherwise |
| Capturing a struct with closures | Transparent; GC manages | Closure owns the struct; `Box<dyn Fn>` fields work fine |
| Higher-order closures | Natural; first-class functions | `Box<dyn Fn>` or `impl Fn` for returning closures |
| Mutable captured state | `ref` cells or mutable bindings | `FnMut` + `move` captured `mut` variables |
