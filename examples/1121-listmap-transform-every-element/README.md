📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1121-listmap-transform-every-element)**

---

# 1121-listmap-transform-every-element — List.map: Transform Every Element
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`map` is the fundamental operation of functional programming: apply a function to every element of a list, producing a new list of the same length with transformed values. It is the backbone of data transformation pipelines, API response formatting, and mathematical vector operations.

OCaml's `List.map` and Rust's `Iterator::map` both express this transformation, but with different evaluation strategies: OCaml's list is strict and builds a new linked list, while Rust's iterator is lazy and computes values on demand.

## Learning Outcomes

- Use `Iterator::map` to transform a slice into a new `Vec`
- Implement recursive `map` mirroring OCaml's `List.map` structure
- Understand the difference between strict list map and lazy iterator map
- Recognize `map` as the functor operation for lists
- Apply map in data transformation pipelines with `.collect()`

## Rust Application

`src/lib.rs` provides two implementations. `map_idiomatic` uses `list.iter().map(f).collect()` — the standard Rust idiom, leveraging the full iterator adapter chain. `map_recursive` uses slice pattern matching `[head, tail @ ..]` to mirror OCaml's `x :: rest` pattern — demonstrating the structural equivalence.

The idiomatic version is O(n) and cache-efficient. The recursive version illustrates structural recursion but is less efficient due to repeated `Vec` extension operations.

## OCaml Approach

```ocaml
let rec map f = function
  | [] -> []
  | x :: rest -> f x :: map f rest
```

OCaml's built-in `List.map` uses this exact structure. It builds the new list from right to left via recursion (not tail-recursive) or from left to right in tail-recursive form with `List.rev`.

## Key Differences

1. **Laziness**: Rust's `Iterator::map` is lazy — the function runs only when elements are consumed; OCaml's `List.map` is strict — all elements are transformed immediately.
2. **Allocation**: Rust's `.collect()` allocates the result `Vec` in one pass; OCaml's `List.map` allocates one cons cell per element.
3. **Slice vs list**: Rust operates on contiguous slices with random access; OCaml's `List.map` traverses a linked list sequentially.
4. **Type inference**: Both infer the type of `f` from usage — `map(|x| x * 2)` works without type annotations in both.

## Exercises

1. Implement `map_with_index<F: Fn(usize, &T) -> U>(list: &[T], f: F) -> Vec<U>` that passes the index along with each element.
2. Write `flat_map<F: Fn(&T) -> Vec<U>>(list: &[T], f: F) -> Vec<U>` that maps and flattens (equivalent to OCaml's `List.concat_map`).
3. Implement `map_in_place<T, F: Fn(&mut T)>(list: &mut [T], f: F)` that transforms elements without allocating a new collection.
