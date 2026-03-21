📖 **[View on hightechmind.io →](https://hightechmind.io/rust/258-iterator-enumerate)**

---

# 258: Index-Value Pairs with enumerate()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Knowing the position of each element while iterating is a recurring need: numbering output lines, finding the index of the first matching element, or computing index-dependent transformations. The traditional C-style `for (int i = 0; i < n; i++)` loop provides this but loses the iterator abstraction. The `enumerate()` adapter solves this by injecting a zero-based index alongside each element, keeping the pipeline composable.

## Learning Outcomes

- Understand how `enumerate()` wraps each element with its zero-based index
- Use `(index, value)` destructuring patterns in `map()` and `for` loops
- Filter or transform elements based on their position
- Combine `enumerate()` with `filter()` to find the index of a matching element

## Rust Application

`Iterator::enumerate()` returns `Enumerate<I>`, yielding `(usize, Item)` pairs. The index starts at zero and increments by one for each element consumed:

```rust
let v = ["a", "b", "c"];
for (i, s) in v.iter().enumerate() {
    println!("{}: {}", i, s);
}
// 0: a, 1: b, 2: c

// Index-dependent transformation
let result: Vec<i32> = [10, 20, 30].iter()
    .enumerate()
    .map(|(i, &val)| val + i as i32)
    .collect();
// [10, 21, 32]
```

## OCaml Approach

OCaml's `List.mapi` applies a function `(index -> element -> result)` to each element, which is the direct equivalent for transformation. For filtering by position, one typically uses a manual `fold_left` with a counter accumulator:

```ocaml
let indexed = List.mapi (fun i x -> (i, x)) ["a"; "b"; "c"]
(* [(0,"a"); (1,"b"); (2,"c")] *)
```

OCaml lacks a direct `enumerate()` on `Seq`, but `Seq.zip (Seq.ints 0) seq` achieves the same lazily.

## Key Differences

1. **Built-in vs manual**: Rust provides `enumerate()` as a first-class adapter; OCaml requires `List.mapi` or a manual index counter for the same effect.
2. **Zero-based index**: Both languages use zero-based indexing for this operation.
3. **Type**: Rust yields `(usize, &T)`; OCaml's `mapi` yields the result of the applied function directly.
4. **Laziness**: `enumerate()` is lazy in Rust; `List.mapi` processes eagerly in OCaml.

## Exercises

1. Use `enumerate()` to find the index of the first element in a slice that satisfies a predicate, returning `Option<usize>`.
2. Build a function that takes a `Vec<String>` and returns a formatted numbered list like `["1. first", "2. second"]`.
3. Use `enumerate()` and `filter()` together to return only the even-indexed elements of a slice.
