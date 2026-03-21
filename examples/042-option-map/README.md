📖 **[View on hightechmind.io →](https://hightechmind.io/rust/042-option-map)**

---

# 042 — Option Map
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`Option::map` applies a function to the value inside a `Some`, leaving `None` unchanged. This is the functor operation — it lifts a function `f: T -> U` into the option context to produce `Option<T> -> Option<U>`. The key property: `None.map(f) == None` for any `f`. This enables transformation of optional values without explicit null-checking.

`map` is the foundation of the option/maybe monad in functional languages. It enables fluent chains: `find_user(id).map(|u| u.name).map(|n| n.to_uppercase())` — each step transforms the value if present. This pattern eliminates deeply nested `if let Some(x) = ...` chains, making code that handles missing values as readable as code that does not.

## Learning Outcomes

- Use `opt.map(|x| transform(x))` to transform values inside `Option`
- Understand that `map` preserves `None` — it never introduces or removes None
- Chain multiple `map` calls for sequential transformations
- Distinguish `map` from `and_then`: map keeps the return type in `Option`, and_then allows the function to return `Option`
- Use `map` with method references: `opt.map(str::to_uppercase)`

## Rust Application

`opt.map(|x| x * 2)` doubles the inner value if present. `opt.map(|x| x.to_string())` converts an `Option<i32>` to `Option<String>`. Chaining: `safe_head(v).map(|x| x * 2).map(|x| x + 1)`. The closure type must match: `Option<T>::map<U, F: FnOnce(T) -> U>`. Note: `map` takes the inner value by move (for `Option<T>`, not `&Option<T>`). Use `as_ref()` to map by reference: `opt.as_ref().map(|x| x.len())`.

## OCaml Approach

OCaml's `Option.map f opt`: `let map f = function None -> None | Some x -> Some (f x)`. The `|>` pipe makes chains natural: `safe_head lst |> Option.map (fun x -> x * 2) |> Option.map string_of_int`. OCaml's `Option.map` is curried — `Option.map (fun x -> x + 1)` partially applies to create a function `option -> option`.

## Key Differences

1. **Method vs function**: Rust: `opt.map(f)` (method call). OCaml: `Option.map f opt` (module function, can be partially applied). The result is identical.
2. **Type inference**: Rust infers the closure type from context. OCaml infers both argument and return types. Both languages rarely need explicit type annotations on `map` calls.
3. **`as_ref` for reference**: Rust's `opt.as_ref().map(...)` maps over `Option<&T>` without consuming the option. OCaml's `Option.map` always borrows by the GC; no explicit `as_ref` needed.
4. **`map` vs `Option.iter`**: Rust's `map` produces a new Option. `Option::iter` produces an iterator of 0 or 1 elements. OCaml's `Option.iter f opt` calls `f` for side effects, returns unit.

## Exercises

1. **Map string**: Write `shorten(opt: Option<String>) -> Option<String>` that truncates the string to 5 characters if Some. Use `opt.map(|s| s.chars().take(5).collect())`.
2. **Nested option**: Given `Option<Option<i32>>`, write `flatten_opt(opt: Option<Option<i32>>) -> Option<i32>` using `.flatten()`. Understand why this is not a `map`.
3. **Map error message**: Write `validate_age(age: Option<i32>) -> Option<String>` that returns `Some("valid")` for ages 0-150 and `None` otherwise. Use `opt.filter(|&a| a >= 0 && a <= 150).map(|_| "valid".to_string())`.
