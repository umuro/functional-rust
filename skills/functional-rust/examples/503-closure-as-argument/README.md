# 503: Closure as Argument

**Difficulty:** 2  **Level:** Beginner-Intermediate

Pass behavior into functions — the foundation of every iterator adapter, callback system, and functional API in Rust.

## The Problem This Solves

Without higher-order functions, you write the same loop structure over and over with a tiny variation in the body: filter-and-collect, transform-and-sum, find-by-condition. Every new requirement means a new function. The logic structure is duplicated; only the behavior inside changes.

The alternative in C is function pointers — but they can't capture state without passing an explicit `void* userdata` parameter. In older Java you'd create an anonymous class implementing a single-method interface. Both approaches are verbose and error-prone.

Rust's `F: Fn(T) -> U` generic bound gives you behavior-as-parameter with zero overhead: each call site gets its own monomorphized function, inlined at compile time. The closure *carries its own state* (captured variables) without needing a separate userdata parameter.

## The Intuition

Passing a closure as an argument is like passing a recipe to a chef: the chef handles the kitchen infrastructure (iteration, error handling, threading), and you provide just the transformation to apply to each ingredient.

In Python, `map(lambda x: x*2, items)` passes a lambda as a behavior. JavaScript uses arrow functions: `items.filter(x => x > 5)`. Rust's version is `items.iter().filter(|&x| x > 5)` — identical concept, but the closure's type is checked at compile time and the call is inlined.

The key choice is **static dispatch** (`F: Fn(...)` — generic, zero-cost) versus **dynamic dispatch** (`&dyn Fn(...)` — vtable, works for heterogeneous collections).

## How It Works in Rust

```rust
// Static dispatch: compiler generates one version per closure type
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)  // inlined at compile time — zero overhead
}

// Works with any closure:
apply(|x| x * 2, 5);           // 10 — captures nothing
let offset = 3;
apply(|x| x + offset, 5);      // 8 — captures offset by ref

// Filter using a closure predicate
fn my_filter<T, F: Fn(&T) -> bool>(items: &[T], pred: F) -> Vec<&T> {
    items.iter().filter(|x| pred(x)).collect()
}
let nums = [1, 2, 3, 4, 5, 6];
let evens = my_filter(&nums, |x| *x % 2 == 0);  // [2, 4, 6]

// Dynamic dispatch: one function pointer, many closure types at runtime
fn apply_dyn(f: &dyn Fn(i32) -> i32, x: i32) -> i32 { f(x) }
// Use when storing different closures in a Vec or struct:
let ops: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
];
let result = ops.iter().fold(10, |acc, f| apply_dyn(f.as_ref(), acc));
// (10+1)*2 = 22
```

## What This Unlocks

- **Iterator adapters** — `map`, `filter`, `fold`, `sort_by` all accept closures as arguments; understanding this lets you write your own.
- **Callback systems** — event handlers, hooks, and plugin architectures pass behavior into a framework as closures.
- **Generic algorithms** — a single `sort_by(|a, b| ...)` replaces a family of typed comparison functions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| HOF parameter | `f : 'a -> 'b` | `f: F` where `F: Fn(A) -> B` |
| Static dispatch | Default (monomorphized functors) | Generic `<F: Fn>` — zero cost |
| Dynamic dispatch | First-class functions (always boxed) | `&dyn Fn` — explicit vtable |
| Closure with state | Captures env automatically | Captures env, compiler infers mode |
| Predicate in filter | `List.filter pred xs` | `iter.filter(\|x\| pred(x))` |
