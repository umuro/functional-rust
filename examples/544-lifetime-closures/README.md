📖 **[View on hightechmind.io →](https://hightechmind.io/rust/544-lifetime-closures)**

---

# Lifetimes in Closures

## Problem Statement

Closures that capture references introduce lifetime constraints that must appear in the closure's type. A closure that borrows a `&str` prefix can only be called while that prefix is alive — the closure's lifetime is bounded by its captured borrows. When returning such closures from functions, the `+ 'a` lifetime bound must appear in the return type to tell callers how long they can use the closure. This is distinct from closures that capture owned data — those have no borrowed lifetime and can be `'static`.

## Learning Outcomes

- How `impl Fn(&str) -> String + 'a` expresses a closure tied to its captured reference's lifetime
- How computing and capturing an owned value (`let sum = data.iter().sum()`) avoids a lifetime constraint
- How `impl FnMut() -> i32` with mutable state (counter) works with no lifetime annotation
- When `Box<dyn Fn(&str) -> String>` is needed vs `impl Fn` for returning closures
- The difference between a closure capturing `&'a str` (tied) and capturing `String` (not tied)

## Rust Application

`make_prefixer<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a` captures `prefix` by reference — the returned closure cannot outlive `prefix`. `make_sum_adder(data: &[i32]) -> impl Fn(i32) -> i32` computes `sum` first and captures the `i32` value — no lifetime annotation needed because the closure owns its state. `make_checker<'a>(valid: &'a [&str])` captures a slice reference. `make_counter()` returns `impl FnMut() -> i32` with mutable state — no borrowed data. `make_formatter(width: usize)` must use `Box<dyn Fn(&str) -> String>` because `impl Fn` return types cannot be stored in a struct field directly.

Key patterns:
- `impl Fn(&str) -> String + 'a` — closure bounded by captured reference
- Pre-compute + capture owned: `let sum = ...; move |x| x + sum` — avoid lifetime
- `Box<dyn Fn>` when the concrete closure type must be hidden behind a field

## OCaml Approach

OCaml closures capture by reference to the GC heap — no lifetime annotations are needed:

```ocaml
let make_prefixer prefix = fun s -> prefix ^ s
let make_sum_adder data = let sum = List.fold_left (+) 0 data in fun x -> x + sum
let make_counter () = let count = ref 0 in fun () -> incr count; !count
```

All captured values are GC-managed — there is no concept of a closure's lifetime being bounded by its captures.

## Key Differences

1. **Lifetime bound in return type**: Rust requires `+ 'a` when a returned closure captures a reference; OCaml requires no annotation since the GC keeps captured values alive.
2. **Owned vs borrowed captures**: Rust distinguishes closures capturing `&str` (lifetime-bounded) from those capturing `String` (lifetime-free); OCaml treats all captures uniformly.
3. **Mutable counter**: Rust `FnMut() -> i32` with `let mut count = 0` captures by move — no lifetime needed; OCaml uses `ref` cells captured by the GC closure.
4. **Box vs impl**: Rust sometimes requires `Box<dyn Fn>` for closures returned from structs or stored in heterogeneous collections; OCaml's uniform value representation avoids this distinction.

## Exercises

1. **Lifetimed combinator**: Write `fn make_both<'a>(f: impl Fn(&str) -> bool + 'a, g: impl Fn(&str) -> bool + 'a) -> impl Fn(&str) -> bool + 'a` that returns a closure checking both.
2. **Owned capture optimization**: Rewrite `make_checker` to clone the `valid` slice data into an owned `Vec<String>` inside the closure so no lifetime annotation is needed on the return type.
3. **Counter with reset**: Extend `make_counter` to return a pair `(impl FnMut() -> i32, impl FnMut())` where the second closure resets the counter — verify both closures share the same mutable state.
