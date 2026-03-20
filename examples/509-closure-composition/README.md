📖 **[View on hightechmind.io →](https://hightechmind.io/rust/509-closure-composition)**

---

# Closure Composition
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


Function composition combines two functions `f` and `g` into `f ∘ g` (apply `g` first, then `f`), and `pipe` provides left-to-right composition — both implemented as higher-order functions returning new closures.

## Problem Statement

Complex data transformations are best expressed as a sequence of simple steps: parse → validate → normalise → format. Manually nesting function calls `f(g(h(x)))` becomes unreadable for long chains. **Function composition** formalises this: `compose(f, g)` returns a new function that applies `g` then `f`. **Piping** (`|>` in F#, OCaml, and Elixir) applies left-to-right. The `Pipeline` builder pattern extends this to dynamic lists of transformations.

## Learning Outcomes

- Implement `compose(f, g)` returning `impl Fn(A) -> C` for mathematical `f ∘ g`
- Implement `pipe(f, g)` for left-to-right `f | g` composition
- Build `make_pipeline(Vec<Box<dyn Fn(T)->T>>)` for dynamic chain construction
- Use the `Pipeline` builder with fluent `.then(f).then(g).run()` API
- Understand the type constraints: `F: Fn(A)->B`, `G: Fn(B)->C` for `pipe(F, G)`

## Rust Application

`compose` is mathematical right-to-left (apply `g` first):

```rust
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(B) -> C, G: Fn(A) -> B {
    move |x| f(g(x))
}
```

`pipe` is left-to-right (apply `f` first — more intuitive for data pipelines):

```rust
pub fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(A) -> B, G: Fn(B) -> C {
    move |x| g(f(x))
}
```

`Pipeline` builder for dynamic composition:

```rust
let result = Pipeline::new()
    .then(|x: i32| x * 2)
    .then(|x| x + 1)
    .run()(5);  // (5*2)+1 = 11
```

## OCaml Approach

OCaml has `@@` (right-to-left application) and `|>` (left-to-right pipe) built in:

```ocaml
let compose f g x = f (g x)   (* right-to-left *)
let pipe f g x = g (f x)      (* left-to-right *)

(* Using built-in operators *)
let result = 5 |> (fun x -> x * 2) |> (fun x -> x + 1)  (* 11 *)

(* Dynamic pipeline *)
let make_pipeline transforms x =
  List.fold_left (fun acc f -> f acc) x transforms
```

OCaml 4.01 added `|>` and `@@` to the standard library; they are idiomatic for function pipelines.

## Key Differences

1. **Built-in operators**: OCaml has `|>` and `@@` in the standard library; Rust has no built-in composition operators — they are library functions.
2. **Type inference**: OCaml infers all types in `compose`/`pipe`; Rust requires explicit type parameters `<A, B, C, F, G>` for composition functions.
3. **Dynamic pipeline**: Rust's `make_pipeline(Vec<Box<dyn Fn(T)->T>>)` requires boxing (heap allocation); OCaml's `List.fold_left` over function lists uses uniform representation.
4. **Builder pattern**: Rust's `Pipeline::new().then(f).then(g).run()` consumes `self` at each step (move semantics); OCaml would use a mutable list `ref` or a functional accumulator.

## Exercises

1. **N-ary compose**: Write `fn compose_all<T>(fns: Vec<Box<dyn Fn(T)->T>>) -> impl Fn(T)->T` that composes a list right-to-left (last function applied first).
2. **Typed pipeline**: Design a type-safe pipeline where each step's output type must match the next step's input type — use a `struct Pipeline<A, B>` parameterised by input and output types.
3. **Lazy evaluation**: Wrap the `Pipeline` in a `struct LazyPipeline<T>` that stores the input alongside the transforms and evaluates lazily when `.evaluate()` is called.
