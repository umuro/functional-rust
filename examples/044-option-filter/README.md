📖 **[View on hightechmind.io →](https://hightechmind.io/rust/044-option-filter)**

---

# 044 — Option Filter
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

`Option::filter` applies a predicate to the value inside a `Some`, turning it into `None` if the predicate fails. It is the conditional guard operation for `Option`: "keep this value only if it satisfies this condition". Combined with `map` and `and_then`, `filter` completes the basic Option toolkit.

This pattern appears in validation pipelines: `parse_int(s).filter(|&x| x >= 0).filter(|&x| x < 100)` — parse a string, then validate the range, producing `None` at any failure point. It is a declarative alternative to nested `if` statements on unwrapped values.

## Learning Outcomes

- Use `opt.filter(|x| predicate(x))` to discard values not meeting a condition
- Chain `filter` with `map` and `and_then` for validation pipelines
- Understand that `filter` never introduces `Some` — it can only remove it
- Use `filter` as a guard in option chains to enforce preconditions
- Recognize filter as the "zero of the monad" — it can collapse a chain to None

## Rust Application

`opt.filter(|&x| x > 0)` returns `None` if `opt` is `None` or if `x <= 0`. The predicate receives a reference `&T`. Multiple filters: `opt.filter(|&x| x >= 18).filter(|&x| x <= 65)` applies both bounds. Combining with map: `safe_parse(s).filter(|&n| n != 0).map(|n| 100 / n)` — parse, guard against zero, then divide. The `.ok()` method converts `Result` to `Option` for use with filter chains.

## OCaml Approach

OCaml's `Option.filter f opt`: `let filter f = function None -> None | Some x -> if f x then Some x else None`. Chaining with pipe: `opt |> Option.filter (fun x -> x > 0) |> Option.map (fun x -> x * 2)`. OCaml 4.08+ provides `Option.filter`. Without it: `opt |> Option.bind (fun x -> if pred x then Some x else None)` — filter is a special case of bind.

## Key Differences

1. **Predicate argument**: Rust's filter closure receives `&T` (reference to the inner value). OCaml's filter function receives `T` directly. Be careful with the reference in Rust: `filter(|&x| x > 0)` pattern-matches the reference.
2. **`filter` as `bind`**: `opt.filter(pred)` is equivalent to `opt.and_then(|x| if pred(&x) { Some(x) } else { None })`. Understanding this connection shows filter is not a primitive — it is derivable.
3. **`Option.filter` availability**: OCaml 4.08 added `Option.filter`. Earlier versions need `match` or the bind derivation. Rust has always had `Option::filter`.
4. **`retain` analogy**: `Option::filter` is analogous to `Vec::retain` — both remove elements that fail a predicate. `retain` is in-place; `filter` produces a new Option.

## Exercises

1. **Age validation**: Write `validate_age(age: Option<i32>) -> Option<i32>` that filters to `[0, 150]` using two chained `filter` calls.
2. **Non-empty string**: Write `non_empty(s: Option<String>) -> Option<String>` that returns `None` for `Some("")` and passes through non-empty strings. Use `filter(|s| !s.is_empty())`.
3. **Conditional transform**: Write `square_if_positive(opt: Option<i32>) -> Option<i32>` that squares the value only if it is positive, returning `None` otherwise. Combine `filter` and `map`.
