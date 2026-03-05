📖 **[View on hightechmind.io →](https://hightechmind.io/rust/597-scott-encoding-rust)**

---

# 597: Scott Encoding for Algebraic Types

**Difficulty:** 4  **Level:** Advanced

Represent algebraic data types as functions — the Church encoding family, but with O(1) pattern matching.

## The Problem This Solves

In lambda calculus and type theory, data types don't exist as primitives — everything is a function. Church encoding and Scott encoding both represent ADTs (Option, List, Bool) purely as functions. The difference: Church encoding folds over the entire structure, while Scott encoding matches on exactly *one* level, giving O(1) pattern match and O(n) recursion — the same as native `enum`.

Why study this in Rust? It illuminates what `enum` and `match` mean computationally. It shows that pattern matching is fundamentally *function application*. It also demonstrates the limits of Rust's type system when encoding recursive types (you'll reach for `Rc<dyn Fn...>` quickly), and it bridges directly to Church encoding in proof assistants and dependently typed languages.

It's also genuinely mind-bending: a Scott-encoded `Option<T>` is a function that takes "what to do if None" and "what to do if Some(T)" and applies the right one. That *is* a match expression, as a value.

## The Intuition

A vending machine as a data value. Instead of inspecting a coin to determine if it's a quarter, the coin *is* a function: give it a slot for each denomination, and it activates the right slot. `scott_some(42)` is a value that, when given a "nothing" handler and a "has value" handler, calls the latter with 42. The data and the dispatch are unified.

## How It Works in Rust

1. **Scott-encoded `Option<T, R>`** — a function that takes two handlers and calls the right one:
   ```rust
   use std::rc::Rc;
   type ScottOption<T, R> = Rc<dyn Fn(Box<dyn Fn() -> R>, Box<dyn Fn(T) -> R>) -> R>;

   fn scott_none<T: 'static, R: 'static>() -> ScottOption<T, R> {
       Rc::new(|on_none: Box<dyn Fn() -> R>, _| on_none())
   }

   fn scott_some<T: Clone + 'static, R: 'static>(v: T) -> ScottOption<T, R> {
       Rc::new(move |_, on_some: Box<dyn Fn(T) -> R>| on_some(v.clone()))
   }
   ```
2. **Pattern matching is function application**:
   ```rust
   fn scott_match<T, R>(m: &ScottOption<T, R>,
       on_none: impl Fn() -> R + 'static,
       on_some: impl Fn(T) -> R + 'static) -> R {
       m(Box::new(on_none), Box::new(on_some))
   }

   let result = scott_match(&scott_some(42),
       || "nothing".to_string(),
       |v| format!("got {}", v));  // "got 42"
   ```
3. **Scott-encoded Bool** — simpler, no `Box` needed with concrete types:
   ```rust
   type SBool<R> = Box<dyn Fn(R, R) -> R>;
   fn s_true<R: Clone + 'static>()  -> SBool<R> { Box::new(|t, _f| t) }
   fn s_false<R: Clone + 'static>() -> SBool<R> { Box::new(|_t, f| f) }
   fn s_if<R: Clone + 'static>(b: SBool<R>, t: R, f: R) -> R { b(t, f) }
   ```
4. **Recursive types require `Rc`** — `ScottList` would need `Rc<dyn Fn(...)>` to break the infinite type cycle that `Box<dyn Fn(...)>` hits.

## What This Unlocks

- **Denotational clarity** — you see exactly what `match` means: apply a function indexed by constructor.
- **Lambda calculus bridge** — Scott encoding is the standard ADT representation in type theory; understanding it prepares you for Coq, Agda, and Lean.
- **Introspect your type system** — the contortions Rust needs (`Rc<dyn Fn>`, `'static` bounds, `Clone` constraints) reveal what a GC buys you in a functional language.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Scott encoding | Clean with polymorphic types | Requires `Rc<dyn Fn>` and `'static` bounds |
| Pattern match | `match` (native) | Function application (encoded) |
| Recursive type | `type 'a list = Nil \| Cons of 'a * 'a list` | Needs `Rc` to break type cycle |
| Performance | GC overhead | `Rc` + `Box` allocations per constructor |
