📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1022-sentinel-vs-result)**

---

# 1022-sentinel-vs-result — Sentinel Values vs Result
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Sentinel values — magic numbers like `-1` for "not found" or empty strings for "missing" — are a C-era pattern for encoding failure in a type that is otherwise used for success. They require callers to check the return value against the sentinel manually, and forgetting to do so compiles without error. The `strlen` convention of returning `-1` on failure, `strtol` returning 0 with `errno` set, and many POSIX APIs use this pattern.

Rust's `Option<T>` and `Result<T, E>` make absence and failure explicit in the type, forcing callers to handle both cases at the type-checking stage. This example contrasts both approaches on equivalent problems.

## Learning Outcomes

- Recognize the failure modes of sentinel-value APIs
- Convert sentinel-based functions to `Option`-returning equivalents
- Convert `Option` to `Result` when the absence reason matters
- Understand the compile-time safety guarantee of `Option` and `Result`
- Know which one to choose for a given situation

## Rust Application

`src/lib.rs` pairs each sentinel-based function with an `Option` or `Result` equivalent. `find_index_sentinel` returns `-1` on failure; `find_index` returns `Option<usize>`. `get_config_sentinel` returns `""` for missing keys; `get_config` returns `Option<&str>`. `find_index_result` returns `Result<usize, String>` when the reason for absence matters to the caller.

The sentinel versions compile even when you forget to check the return value. The `Option` versions force a `match` or `.unwrap_or` call.

## OCaml Approach

OCaml eliminated sentinel values early. The standard library uses `option` types throughout: `List.find_opt`, `String.index_opt`, `Hashtbl.find_opt`. Exceptions play a role for unexpected failures, but `option` is idiomatic for "might not be present":

```ocaml
let find_index xs x =
  let rec go i = function
    | [] -> None
    | h :: t -> if h = x then Some i else go (i + 1) t
  in
  go 0 xs
```

## Key Differences

1. **Type safety**: Rust's `Option<usize>` makes the compiler enforce the check; a sentinel `i32` does not — you can accidentally use `-1` as an index.
2. **`?` operator**: `Option<T>` propagates with `?` just like `Result<T, E>`; sentinel values require manual checks at every call site.
3. **Expressive power**: `Result` carries a reason for failure; `Option` does not. Sentinel values can encode multiple failure modes (different magic numbers) but are error-prone.
4. **Standard library consistency**: OCaml's and Rust's standard libraries both use `option`/`Option` uniformly; C APIs are inconsistent with their sentinel conventions.

## Exercises

1. Write a wrapper around a hypothetical C-style API function `c_find(haystack: &[i32], needle: i32) -> i32` that returns `-1` on failure. Convert it to return `Option<usize>`.
2. Chain `find_index` and `get_config` together: find the index of "port" in a list of known keys, then look up the config value by that index.
3. Design a `LookupTable` struct that internally uses a `HashMap` but exposes sentinel-free methods returning `Option` and `Result`.
