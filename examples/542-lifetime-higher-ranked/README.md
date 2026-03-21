📖 **[View on hightechmind.io →](https://hightechmind.io/rust/542-lifetime-higher-ranked)**

---

# Higher-Ranked Trait Bounds (for<'a>)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Standard lifetime parameters on functions are monomorphic: `F: Fn(&'a str) -> &'a str` means `F` works for one specific lifetime `'a` chosen by the caller. But some abstractions need functions that work for any lifetime — a callback that processes strings of any duration, not just one specific scope. Higher-Ranked Trait Bounds (HRTB) with `for<'a>` express universal quantification over lifetimes: `F: for<'a> Fn(&'a str) -> &'a str` means `F` must work for every possible lifetime. This is essential for trait objects, parser combinators, and middleware that receive arbitrarily-scoped references.

## Learning Outcomes

- What `for<'a>` means: the bound must hold for all lifetimes simultaneously
- How `apply_hrtb<F: for<'a> Fn(&'a str) -> &'a str>` differs from a fixed-lifetime version
- How HRTBs appear implicitly in common Rust patterns like `F: Fn(&T) -> &T`
- How to write traits whose methods have lifetime parameters (`trait Processor`)
- Where HRTBs are essential: trait objects storing callbacks, `Iterator::for_each`, `serde` deserializers

## Rust Application

`apply_fixed<'a, F: Fn(&'a str) -> &'a str>` fixes `'a` at the call site — `F` only needs to work for that one lifetime. `apply_hrtb<F: for<'a> Fn(&'a str) -> &'a str>` requires `F` to work for any `'a` — a stricter bound. `transform_all<F: for<'a> Fn(&'a str) -> &'a str>` maps over `&[String]` using the HRTB callback. `identity` and `trim_str` are concrete functions satisfying the HRTB because they handle any lifetime.

Key patterns:
- `for<'a> Fn(&'a str) -> &'a str` — universally quantified over all lifetimes
- HRTBs are often inferred when you write `Fn(&T) -> &T` — `for<'a>` is implicit
- Trait methods with own lifetimes: `fn process<'a>(&self, input: &'a str) -> &'a str`

## OCaml Approach

OCaml's HM type system achieves similar genericity through polymorphism. A function that processes strings of any kind is simply:

```ocaml
let apply f s = f s   (* works for any 'a -> 'b *)
let transform_all f items = List.map f items
```

OCaml's rank-2 polymorphism (for functions that must be polymorphic in their arguments) requires explicit type annotations with `forall` using the module system or record wrapping.

## Key Differences

1. **Implicit vs explicit**: In Rust, `Fn(&T) -> &T` implicitly introduces `for<'a>` — it is commonly written without explicit `for<'a>`; the syntax only appears when necessary for clarity.
2. **Rank-2 types**: OCaml needs record wrapping for rank-2 polymorphic functions (functions that take polymorphic functions); Rust's `for<'a>` achieves this for lifetime polymorphism.
3. **Common implicit HRTB**: Many Rust programs use HRTBs without knowing it — `impl Fn(&str)` implicitly means `impl for<'a> Fn(&'a str)`.
4. **Error messages**: HRTB errors in Rust can be cryptic — the compiler reports lifetime bound violations that reference `for<'a>` without explaining it well.

## Exercises

1. **HRTB callback**: Write a function `fn apply_twice<F: for<'a> Fn(&'a str) -> &'a str>(f: F, s: &str) -> String` that applies `f` to `s` twice, returning the result of the second application.
2. **Trait with HRTB**: Implement the `Processor` trait for a `TrimProcessor` struct and use it in `transform_all` — verify it works with strings of any lifetime.
3. **Box<dyn ...> HRTB**: Store a `Box<dyn for<'a> Fn(&'a str) -> &'a str>` in a struct and call it with references of different lifetimes — show the boxed closure satisfies the HRTB.
