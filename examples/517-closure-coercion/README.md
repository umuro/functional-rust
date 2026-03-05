📖 **[View on hightechmind.io →](https://hightechmind.io/rust/517-closure-coercion)**

---

# 517: Closure-to-fn-pointer Coercion

**Difficulty:** 3  **Level:** Intermediate

Non-capturing closures coerce to `fn` pointers; capturing ones cannot.

## The Problem This Solves

Rust has two kinds of callable values: function pointers (`fn(T) -> U`) and closures (`impl Fn`). Function pointers are a single machine-word — just an address. Closures are fat: they carry their captured environment. This size difference matters when you're building dispatch tables, FFI callbacks, or data structures that store many callbacks of the same type.

If you use `Box<dyn Fn>` for everything, you pay for heap allocation and indirection even when the closure doesn't capture anything. If you want a `[fn(i32) -> i32; N]` array (all same size, stack-allocated), you need function pointers — not `dyn Fn`. The coercion rule gives you this for free: any closure that captures nothing is automatically coercible to a `fn` pointer of the matching signature.

Understanding this boundary — what can and cannot coerce — helps you choose the right type in APIs, avoid unnecessary allocations, and write ergonomic callback registration without forcing callers to use `Box`.

## The Intuition

A non-capturing closure is just a function with an anonymous name. It has no environment to carry. So its type *is* a function pointer — the compiler will let it coerce to one. A capturing closure, on the other hand, is a struct with a `call` method. It has no function-pointer representation because its environment goes with it everywhere. You must use `Box<dyn Fn>` or `impl Fn` for those.

Think of it like this: if the closure needs a backpack (captured variables), it can't pretend to be a bare pointer. If it travels light, it can.

## How It Works in Rust

1. **Non-capturing closure** — `let f: fn(i32) -> i32 = |x| x + 1;` compiles; the closure captures nothing, so it coerces.
2. **Named function** — `let f: fn(i32) -> i32 = double;` always works; named functions *are* `fn` pointers.
3. **Array of fn pointers** — `[double, triple, |x| x * x]` works if all are non-capturing; uniform size, stack-allocated.
4. **Capturing closure** — `let offset = 42; let f: fn(i32) -> i32 = |x| x + offset;` **fails**; must use `Box<dyn Fn(i32) -> i32>` instead.
5. **FFI-style callbacks** — register a `fn(i32) -> i32` type alias as a callback; non-capturing closures and named functions both satisfy it without boxing.

## What This Unlocks

- Build dispatch tables and transform arrays with `fn` pointers — no heap allocation, uniform size.
- Design FFI-compatible callback APIs that accept `fn` pointers directly.
- Choose between `fn`, `impl Fn`, and `Box<dyn Fn>` with confidence based on whether capture is needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Function pointers | All functions/closures are values; no distinction | `fn` pointer (no capture) vs closure (`impl Fn`/`Box<dyn Fn>`) |
| Closure coercion | No equivalent; all closures are uniform | Non-capturing closures coerce to `fn T -> U`; capturing ones cannot |
| Array of functions | `(int -> int) array` naturally | `[fn(i32) -> i32; N]` requires non-capturing closures |
| Callback type | `'a -> 'b` function type | Choose `fn`, `impl Fn`, or `Box<dyn Fn>` based on capture needs |
