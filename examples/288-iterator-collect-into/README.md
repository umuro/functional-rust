📖 **[View on hightechmind.io →](https://hightechmind.io/rust/288-iterator-collect-into)**

---

# 288: Materializing Iterators with collect()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Lazy iterators describe computations but produce no output until consumed. The `collect()` method is the primary way to materialize a lazy iterator pipeline into a concrete data structure. Its power lies in genericity: the same `collect()` call produces a `Vec`, `HashSet`, `HashMap`, `String`, `BTreeMap`, or any other `FromIterator`-implementing type, depending solely on the type annotation. This makes pipelines maximally composable — the output format is a separate decision from the transformation logic.

## Learning Outcomes

- Understand `collect()` as materializing a lazy iterator into any `FromIterator<T>` type
- Use type annotations (or turbofish `::<Vec<_>>()`) to specify the output collection type
- Collect into `HashSet` for deduplication, `HashMap` from pairs, `String` from chars
- Recognize `collect::<Result<Vec<T>, E>>()` as the short-circuit pattern for fallible collection

## Rust Application

`collect()` is generic over the output type, which is determined by type inference or explicit annotation:

```rust
// Vec: basic materialization
let squares: Vec<u32> = (0..5).map(|x| x * x).collect();

// HashSet: automatic deduplication
let unique: HashSet<i32> = vec![1, 2, 2, 3, 3].into_iter().collect();

// HashMap: from (key, value) pairs
let map: HashMap<&str, i32> = vec![("a", 1), ("b", 2)].into_iter().collect();

// String: from characters
let s: String = "hello".chars().map(|c| c.to_ascii_uppercase()).collect();

// Result<Vec>: fails on first error
let nums: Result<Vec<i32>, _> = vec!["1", "2", "x"].iter()
    .map(|s| s.parse::<i32>()).collect();
// Err(...) because "x" fails to parse
```

## OCaml Approach

OCaml does not have a unified `collect` function. Each collection type has its own conversion function: `List.of_seq`, `Array.of_seq`, `Hashtbl.of_seq`, or `String.concat ""` for strings:

```ocaml
(* OCaml: different function for each target type *)
let lst = List.of_seq (Seq.map (fun x -> x*x) (Seq.init 5 Fun.id))
let arr = Array.of_seq (Seq.map (fun x -> x*x) (Seq.init 5 Fun.id))
```

## Key Differences

1. **Unified API**: Rust's `collect()` works for all `FromIterator` types via one method; OCaml requires type-specific conversion functions.
2. **Type-driven dispatch**: The output type of `collect()` is selected by the compiler from type annotations alone — no conditional branching.
3. **Fallible collection**: `collect::<Result<Vec<T>, E>>()` aggregates results, short-circuiting on first error — OCaml requires explicit fold logic.
4. **Custom types**: Implementing `FromIterator` makes any user-defined collection participate in `collect()` — it is an extension point.

## Exercises

1. Collect a `Vec<(String, Vec<i32>)>` into a `HashMap<String, Vec<i32>>` using `collect()`.
2. Use `collect::<String>()` to join a vector of characters with an uppercase transformation.
3. Collect a `Vec<Result<i32, String>>` into `Result<Vec<i32>, String>`, then separately collect all errors using `partition()`.
