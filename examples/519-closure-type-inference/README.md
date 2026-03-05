# 519: Closure Type Inference

**Difficulty:** 2  **Level:** Beginner-Intermediate

Rust infers closure types from context — but each closure has a unique anonymous type.

## The Problem This Solves

Closures in Rust don't have a written type — the compiler generates a unique, anonymous type for each one, even if two closures have identical signatures and bodies. This matters when you try to store multiple closures in a `Vec`, return them from functions, or pass them across type boundaries. Understanding how inference works — and where it breaks down — prevents confusing errors.

It also matters for performance. When Rust can infer a concrete closure type at the call site, it monomorphizes the code: no vtable, no indirection, full inlining potential. When you use `dyn Fn`, you pay for dynamic dispatch. The compiler does the right thing automatically, but you need to understand what "the right thing" is.

The first-use rule is a subtle footgun: a closure's input types are fixed by the first call. If you call `add(1i32, 2i32)`, the closure is now `fn(i32, i32) -> i32` forever. Trying to call it with `f64` later is a type error. This is different from generics, which can be instantiated multiple times.

## The Intuition

Think of closure type inference as the compiler watching how you use the closure and then making a permanent decision. The first time you call it, the types are locked in. Every use after that must match. This is monomorphism by default — great for performance, occasionally surprising.

The unique-type rule means `let f = |x: i32| x+1; let g = |x: i32| x+1;` gives `f` and `g` different types even though they look identical. You can't put them in a `[_; 2]` array unless you annotate both as `fn(i32) -> i32`.

## How It Works in Rust

1. **Context inference** — `nums.iter().map(|&x| x * 2)` infers `x: i32` from `Vec<i32>`; no annotation needed.
2. **First-use fixation** — `let add = |x, y| x + y; add(1i32, 2i32);` locks the type as `(i32, i32) -> i32`.
3. **Explicit annotations** — annotate the parameter or return type when context is insufficient: `|s: &str| -> i64 { s.parse().unwrap_or(0) }`.
4. **Unique anonymous types** — `f` and `g` defined separately have distinct types; use `impl Fn(i32) -> i32` or `fn(i32) -> i32` when you need a common type.
5. **Generic functions** — `fn apply<F: Fn(T) -> U>(f: F, x: T) -> U` accepts any closure matching the signature; monomorphizes per call site.

## What This Unlocks

- Write closures in iterator chains without annotating every type — inference handles it.
- Understand why `Vec<impl Fn>` doesn't work and why `Vec<Box<dyn Fn>>` does.
- Confidently choose between monomorphized generics and dynamic dispatch based on your use case.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Closure type | Inferred structurally; closures with same sig are compatible | Each closure has unique anonymous type; `impl Fn` or `Box<dyn Fn>` for abstraction |
| Polymorphic closures | Natural; `let f x = x + 1` is polymorphic | Closures are monomorphic; type fixed on first use |
| Type inference | Bidirectional; Hindley-Milner | Unidirectional; inferred from context and first use |
| Collections of closures | `(int -> int) list` directly | `Vec<Box<dyn Fn(i32) -> i32>>` needed for heterogeneous closures |
