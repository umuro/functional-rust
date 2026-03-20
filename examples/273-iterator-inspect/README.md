📖 **[View on hightechmind.io →](https://hightechmind.io/rust/273-iterator-inspect)**

---

# 273: Debugging Iterators with inspect()

## Problem Statement

Iterator pipelines are often opaque: when a `filter().map().fold()` chain produces an unexpected result, there is no obvious place to insert a print statement without breaking the pipeline. The `inspect()` adapter solves this by injecting a side-effect function at any point in the pipeline — the values pass through unchanged, but the closure can log, count, assert, or monitor them. It is the functional equivalent of `console.log` placed between transformations.

## Learning Outcomes

- Understand `inspect(f)` as a transparent pass-through that applies a side-effect to each element
- Use `inspect()` to debug intermediate values inside a lazy iterator pipeline
- Recognize that `inspect()` does not modify values — it only observes them
- Apply `inspect()` to count elements processed at different pipeline stages for profiling

## Rust Application

`Iterator::inspect(f)` returns an `Inspect<I, F>` adapter that calls `f(&item)` on each element, then yields the item unchanged. Since iterators are lazy, `inspect()` only runs when the pipeline is consumed:

```rust
let result: Vec<i32> = [1, 2, 3, 4, 5]
    .iter()
    .copied()
    .inspect(|x| println!("before filter: {}", x))
    .filter(|&x| x % 2 == 0)
    .inspect(|x| println!("after filter: {}", x))
    .collect();
// Prints: before 1, before 2, after 2, before 3, before 4, after 4, before 5
// result = [2, 4]
```

## OCaml Approach

OCaml lacks a built-in `inspect` equivalent. The idiomatic approach wraps a function with a side-effect using `|>` and a `tap`-like helper:

```ocaml
let tap f x = f x; x  (* apply side-effect, return value unchanged *)

let result =
  [1;2;3;4;5]
  |> List.map (tap (Printf.printf "before filter: %d\n"))
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (tap (Printf.printf "after filter: %d\n"))
```

The `tap` pattern is standard in functional languages for inserting side-effects into pipelines.

## Key Differences

1. **Built-in vs manual**: Rust provides `inspect()` as a standard adapter; OCaml requires a manual `tap` combinator.
2. **Laziness reveals timing**: Because Rust iterators are lazy, `inspect()` only fires when the pipeline is consumed — which reveals evaluation order and lazy behavior.
3. **Testing use**: `inspect()` can collect observed values into a `Vec` via closure capture, enabling assertion-based pipeline testing.
4. **Production use**: `inspect()` with logging frameworks (like `tracing`) is used in production pipelines to add observability without restructuring code.

## Exercises

1. Add `inspect()` calls to a multi-step iterator pipeline to print each stage's output, then count how many elements reach each stage.
2. Use `inspect()` with a mutable counter to count how many elements pass through each stage of a `filter().map().take_while()` pipeline.
3. Write a test that uses `inspect()` to capture intermediate values into a `Vec` via closure capture and assert their expected values.
