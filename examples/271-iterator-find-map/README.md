📖 **[View on hightechmind.io →](https://hightechmind.io/rust/271-iterator-find-map)**

---

# 271: Transform-and-Find with find_map()

## Problem Statement

A very common pattern is: try to convert each element into a useful form, stop at the first success, and return it. For example, parse numbers from strings until one succeeds, look up each key in several registries until a hit, or try multiple fallback strategies until one works. The naive approach chains `map()` and `find()`, but this creates intermediate `Option` values. The `find_map(f)` adapter fuses these into a single lazy operation: find the first `Some(...)` result from applying `f`.

## Learning Outcomes

- Understand `find_map(f)` as fused `map(f).flatten().next()` — find the first `Some` from `f`
- Recognize the "try each, return first success" pattern that `find_map` optimally handles
- Use `find_map()` to parse heterogeneous data, looking for the first valid interpretation
- Distinguish from `filter_map()`: `find_map` stops at first success, `filter_map` collects all

## Rust Application

`Iterator::find_map(f)` calls `f` on each element and returns the first `Some(...)` value, stopping immediately. Returns `None` if all calls return `None`:

```rust
let strings = ["foo", "bar", "42", "baz"];
let result = strings.iter().find_map(|s| s.parse::<i32>().ok());
// Some(42) — stops after finding "42", doesn't process "baz"

// Find first number larger than 10
let nums = [1i32, 2, 3, 15, 4, 20];
let result = nums.iter().find_map(|&x| if x > 10 { Some(x * 2) } else { None });
// Some(30) — transforms and returns first match
```

## OCaml Approach

OCaml's `List.find_map` (standard since OCaml 4.10) is exactly equivalent:

```ocaml
let result = List.find_map (fun s ->
  match int_of_string_opt s with
  | Some n -> Some n
  | None -> None
) ["foo"; "bar"; "42"; "baz"]
(* Some 42 *)
```

This is one of the cleanest analogies between the two languages — both provide `find_map` as a standard library function.

## Key Differences

1. **Standard library parity**: Both Rust and OCaml (4.10+) provide `find_map` as a standard function with identical semantics.
2. **Lazy**: Both implementations stop at the first `Some` — crucial for expensive operations like network lookups or file parsing.
3. **vs filter_map**: `find_map` returns `Option<B>` (first success); `filter_map` returns `Iterator<Item=B>` (all successes).
4. **Combinatorial search**: Useful in plugin systems, format auto-detection, and fallback chains where you try until one works.

## Exercises

1. Given a list of user IDs, use `find_map()` to return the first user that exists in a `HashMap` lookup.
2. Try to parse a string first as `i64`, then as `f64`, then as `bool`, using `find_map` over a list of parsing functions.
3. Implement `find_map` from scratch using only `map()`, `filter()`, and `next()`, then verify it produces identical results.
