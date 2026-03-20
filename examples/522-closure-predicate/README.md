📖 **[View on hightechmind.io →](https://hightechmind.io/rust/522-closure-predicate)**

---

# Predicate Functions Pattern

## Problem Statement

Predicates — boolean-valued functions — appear everywhere in data processing: filtering collections, validating inputs, routing events, enforcing business rules. When predicates are composed rather than inlined, code becomes self-documenting and reusable. The predicate combinator pattern (AND, OR, NOT, `all_of`, `any_of`) treats predicates as first-class values, enabling query languages, rule engines, and access-control systems to be built from small, testable pieces.

## Learning Outcomes

- How to implement composable predicates using `impl Fn(&T) -> bool` return types
- How `pred_and`, `pred_or`, and `pred_not` build new predicates from existing ones
- How `all_of` and `any_of` generalize to dynamic lists of predicates
- How to combine predicate composition with `Iterator::filter` for expressive queries
- How the pattern maps to rule engines and validation frameworks in production systems

## Rust Application

`pred_and<T, P1, P2>(p1, p2)` returns `impl Fn(&T) -> bool` that short-circuits on the first false. `pred_not<T, P>` wraps `P` and negates its result. `all_of(preds: Vec<Box<dyn Fn(&T) -> bool>>)` accepts a dynamic list and returns a closure checking all. Common predicates `is_positive()`, `is_even()`, and `in_range(min, max)` are factory functions returning closures that can be stored and passed around. The code shows how to chain combinators: `pred_and(is_positive(), is_even())` composes two factories.

Key patterns:
- `impl Fn(&T) -> bool` return — opaque closure type, zero heap allocation
- `move |x| p1(x) && p2(x)` — capturing `p1`/`p2` by move into combined closure
- `Vec<Box<dyn Fn(&T) -> bool>>` — dynamic predicate list for runtime composition

## OCaml Approach

OCaml predicates are plain functions `'a -> bool`. Composition is idiomatic with higher-order functions and no special combinators are needed — the language's native `&&`/`||` can be lifted:

```ocaml
let pred_and p1 p2 x = p1 x && p2 x
let pred_or  p1 p2 x = p1 x || p2 x
let pred_not p x = not (p x)
let all_of preds x = List.for_all (fun p -> p x) preds
```

## Key Differences

1. **Return type opaqueness**: Rust `impl Fn(&T) -> bool` hides the concrete closure type; OCaml predicates have transparent function types like `int -> bool` visible to the type checker.
2. **Dynamic dispatch cost**: Rust's `Vec<Box<dyn Fn(&T) -> bool>>` allocates each predicate on the heap; OCaml's `list` of predicates also heap-allocates but through GC-managed closures.
3. **Ownership of captured state**: Rust predicates capturing data must use `move`; OCaml closures capture by reference to the GC heap automatically.
4. **Type genericity**: Rust's `pred_and<T, P1, P2>` works for any `T` via generics; OCaml's `pred_and` is polymorphic via HM inference with no explicit type parameters.

## Exercises

1. **String predicate library**: Build `is_non_empty()`, `has_prefix(prefix: String)`, and `has_length_between(min, max)` as predicate factories and compose them to validate email-like strings.
2. **Predicate from regex**: Implement `matches_pattern(pat: &str) -> impl Fn(&str) -> bool` using `std::str::contains` or a simple substring check, and compose it with `pred_not` to reject certain strings.
3. **Weighted all_of**: Extend `all_of` to `all_of_weighted(preds: Vec<(Box<dyn Fn(&T) -> bool>, f64)>)` that returns the sum of weights of satisfied predicates, enabling scoring rather than binary pass/fail.
