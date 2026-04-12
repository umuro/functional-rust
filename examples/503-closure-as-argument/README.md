📖 **[View on hightechmind.io →](https://hightechmind.io/rust/503-closure-as-argument)**

---

# Closure as Argument
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



Passing closures as function arguments is the foundation of higher-order programming: `map`, `filter`, `fold`, `sort_by`, and custom adapters all accept closures that specialise their behaviour at the call site.

## Problem Statement

A function that hardcodes its operation is inflexible. `Vec::sort()` only sorts in ascending order; `sort_by(|a, b| b.cmp(a))` sorts in descending order — same algorithm, different comparator. The ability to pass behaviour as an argument (higher-order functions) is the core of functional programming: it separates the structure of a computation (iterate, fold, filter) from the policy (what to do at each step). Rust's `impl Fn` bound enables this with zero runtime overhead through monomorphisation.

## Learning Outcomes

- Accept a closure with `F: Fn(A) -> B` bound parameters
- Implement `apply`, `apply_twice`, and `compose` as higher-order functions
- Build `filter_with`, `map_with`, and `reduce_with` wrappers around iterator methods
- Understand that `impl Fn` in argument position monomorphises (static dispatch, zero overhead)
- Compose two closures into a new closure using captured ownership

## Rust Application

`apply_twice` demonstrates the pattern — the function is generic over any `Fn(i32) -> i32`:

```rust
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}
// apply_twice(|x| x * 2, 5)  // 20
```

`compose` captures both functions in a `move` closure:

```rust
pub fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
    move |x| f(g(x))
}
```

The result is a new closure that applies `g` then `f` — `f ∘ g` in mathematical notation.

## OCaml Approach

OCaml's functions are first-class without any trait declaration:

```ocaml
let apply f x = f x
let apply_twice f x = f (f x)
let compose f g x = f (g x)

let filter_with pred items = List.filter pred items
let map_with f items = List.map f items
let reduce_with f init items = List.fold_left f init items
```

OCaml's `|>` pipe operator and `@@` application operator complement function composition:

```ocaml
5 |> (fun x -> x * 2) |> (fun x -> x + 1)  (* 11 *)
```

## Key Differences

1. **Trait bounds**: Rust requires explicit `F: Fn(A) -> B` bounds; OCaml infers the function type automatically.
2. **Monomorphisation**: Rust's `impl Fn` argument generates a separate specialised function per call site; OCaml's functions are polymorphic at runtime via uniform representation.
3. **Composition ownership**: Rust's `compose` requires `move` to capture `f` and `g` by value; OCaml captures by reference automatically.
4. **`filter_with` vs. `List.filter`**: Rust builds these as wrappers around `Iterator` methods; OCaml's `List.filter` is already the higher-order version.

## Exercises

1. **`apply_n`**: Write `fn apply_n<F: Fn(i32) -> i32>(f: F, x: i32, n: usize) -> i32` that applies `f` exactly `n` times.
2. **Predicate combinators**: Write `and_pred`, `or_pred`, and `not_pred` that take `Fn(&T) -> bool` closures and return new closures combining them with `&&`, `||`, and `!`.
3. **Iterator adaptor**: Write `fn window_map<F: Fn(i32, i32) -> i32>(data: &[i32], f: F) -> Vec<i32>` that applies `f` to each consecutive pair `(data[i], data[i+1])`.
