📖 **[View on hightechmind.io →](https://hightechmind.io/rust/881-iterator-adapters)**

---

# 881-iterator-adapters — Iterator Adapters
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Functional programming's core power comes from composing small transformations into pipelines. Haskell's `Data.List`, OCaml's `List` module, and Python's `itertools` all provide map, filter, flat_map, take, and skip as standalone functions or methods. Rust packages these as lazy iterator adapters — each adapter wraps the previous iterator and transforms elements on demand, with no intermediate allocation. This pipeline model replaces nested loops with declarative data transformations. The lazy evaluation means even pipelines over infinite iterators terminate correctly when capped by `.take()` or `.find()`.

## Learning Outcomes

- Build data processing pipelines by chaining map, filter, flat_map, take, and skip
- Understand that iterator adapters are lazy — no work happens until consumed
- Use `.chain()` to concatenate iterators without allocating a combined container
- Write complex multi-step transformations as readable single expressions
- Compare with OCaml's `List.map |> List.filter` pipeline style

## Rust Application

The `pipeline` function chains `.filter(positive) -> .map(square) -> .map(to_string) -> .collect()` in one expression. `flat_map_example` splits sentences into words using `.flat_map(|s| s.split_whitespace())`. `take_skip_demo` shows conditional filtering with `.filter`, `.map`, `.take(5)` in sequence. `chain_demo` concatenates two slices lazily with `.chain()`. The `indexed_evens` function uses `.enumerate().filter(|(i, _)| i % 2 == 0)` to combine positional and value filtering.

## OCaml Approach

OCaml uses chained function application with the pipe operator: `list |> List.filter (fun x -> x > 0) |> List.map (fun x -> x * x)`. Each function returns a new list (eager evaluation), unlike Rust's lazy adapters. For lazy pipelines, OCaml uses `Seq`: `Seq.of_list xs |> Seq.filter pred |> Seq.map f |> List.of_seq`. The `flat_map` equivalent is `List.concat_map`. Take and skip: `Seq.take`, `Seq.drop`.

## Key Differences

1. **Laziness**: Rust adapters are lazy — no work until consumed; OCaml `List` operations are eager and create intermediate lists.
2. **Allocation**: Rust iterator pipelines avoid intermediate heap allocations; OCaml pipelines allocate a new list at each step.
3. **Infinite sequences**: Rust can chain adapters on infinite iterators safely (`.take()` bounds them); OCaml needs `Seq` for the same safety.
4. **Syntax**: Rust uses method chaining (`.map().filter()`); OCaml uses the pipe operator (`|> List.map |> List.filter`).

## Exercises

1. Write a `word_frequency` pipeline that takes a `&str`, splits into words, lowercases, and counts occurrences using `.fold()` into a `HashMap`.
2. Implement `moving_average` using `.windows(n)` and `.map()` to compute overlapping window averages in a single iterator chain.
3. Write a `cross_product` function over two slices using `.flat_map()` and `.map()` that produces all `(a, b)` pairs.
