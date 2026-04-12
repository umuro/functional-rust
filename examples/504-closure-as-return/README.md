📖 **[View on hightechmind.io →](https://hightechmind.io/rust/504-closure-as-return)**

---

# Closure as Return
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



Returning closures from functions — factory functions — enables partial application, stateful generators, and configurable behaviour. Rust uses `impl Fn` for static dispatch or `Box<dyn Fn>` when the concrete type must be erased.

## Problem Statement

A function that returns a specialised function is a closure factory. `make_adder(5)` returns a function that adds 5 to its argument — a specific case of partial application. `make_counter()` returns a stateful function that increments a counter on each call. These patterns are everywhere: middleware builders, parser combinators, event handlers, configuration-driven pipelines. Rust requires the return type to be concrete: `impl Fn(i32) -> i32` for static dispatch (inlined by the compiler) or `Box<dyn Fn(i32) -> i32>` when type erasure is needed for heterogeneous collections.

## Learning Outcomes

- Return closures with `impl Fn(A) -> B` for zero-cost static dispatch
- Return stateful closures with `impl FnMut() -> T` for generator patterns
- Use `Box<dyn Fn>` when the concrete type must be hidden or stored heterogeneously
- Capture multiple values in a returned closure with `move`
- Understand that `impl Trait` in return position infers one concrete type per call site

## Rust Application

`make_adder` captures `n` by move and returns a static-dispatch closure:

```rust
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

`make_counter` captures mutable state — the returned closure must be called with `mut`:

```rust
pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || { count += 1; count }
}
```

`make_multiplier_factory` returns a factory that itself returns `Box<dyn Fn>`:

```rust
pub fn make_multiplier_factory() -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    |factor| Box::new(move |x| x * factor)
}
```

## OCaml Approach

OCaml functions naturally return closures — no special syntax required:

```ocaml
let make_adder n = fun x -> x + n
let make_linear slope intercept = fun x -> slope *. x +. intercept
let make_counter () =
  let count = ref 0 in
  fun () -> incr count; !count
```

OCaml's `ref` provides the mutable state for stateful closures; there is no `mut` annotation on the returned function.

## Key Differences

1. **`impl Fn` vs. `Box<dyn Fn>`**: Rust's `impl Fn` is zero-cost (monomorphised); `Box<dyn Fn>` adds a heap allocation and vtable dispatch. OCaml has uniform representation — no such distinction.
2. **`FnMut` return**: Rust's `make_counter` returns `impl FnMut()` — callers must declare `mut counter`. OCaml's counter closure mutates via `ref` without any declaration.
3. **Lifetime of captures**: Rust's `impl Fn` return's lifetime is tied to the closure's captures (the compiler infers this); OCaml's GC manages all lifetimes.
4. **Type inference**: Rust's compiler infers the concrete closure type behind `impl Fn`; OCaml infers the function type directly.

## Exercises

1. **Fibonacci generator**: Write `fn make_fib() -> impl FnMut() -> u64` that returns successive Fibonacci numbers on each call, maintaining `(a, b)` state.
2. **Rate limiter**: Write `fn make_rate_limiter(max_per_sec: u32) -> impl FnMut() -> bool` using `std::time::Instant` that returns `true` when the call is within the rate limit and `false` when exceeded.
3. **Typed pipeline builder**: Write `fn make_pipeline<A, B, C>(f: impl Fn(A)->B, g: impl Fn(B)->C) -> impl Fn(A)->C` that chains two transformations and verify it composes with a third.
