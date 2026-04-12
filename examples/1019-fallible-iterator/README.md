📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1019-fallible-iterator)**

---

# 1019-fallible-iterator — Fallible Iterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Standard iterators yield `T` values with no error channel. But many real-world data sources — file readers, database cursors, network streams — can fail mid-iteration. The `fallible-iterator` crate and manual implementations model this with iterators that yield `Option<Result<T, E>>`: `None` for end-of-sequence, `Some(Ok(v))` for a value, and `Some(Err(e))` for a failure.

This is the basis of `serde`'s streaming deserialisation, `std::io::Lines`, and async stream error handling in Tokio.

## Learning Outcomes

- Implement an `Iterator` with `Item = Result<T, E>` for fallible sequences
- Use `collect::<Result<Vec<T>, E>>()` to consume a fallible iterator with fail-fast behaviour
- Build an adaptor that stops at the first error versus one that accumulates both sides
- Understand how `std::io::Lines` uses the same pattern
- Connect this pattern to async `Stream` types in Tokio

## Rust Application

`src/lib.rs` implements `LineParser`, an iterator that yields `Result<i64, String>`. It wraps a `Vec<String>` and parses each line on `next()`. `take_while_ok` consumes any fallible iterator using `?` inside a `for` loop, stopping at the first error. `process_all` separates results without stopping. The tests show that `collect::<Result<Vec<i64>, String>>()` on the iterator short-circuits at the first bad line.

`std::io::BufReader::lines()` uses this exact pattern, yielding `Result<String, io::Error>` so read errors do not require a separate error callback.

## OCaml Approach

OCaml sequences (`Seq.t`) are lazy but not natively fallible. Fallible iteration requires wrapping:

```ocaml
type 'a result_seq = unit -> ('a, exn) result Seq.node

let take_while_ok seq =
  let rec go acc s =
    match s () with
    | Seq.Nil -> Ok (List.rev acc)
    | Seq.Cons (Ok v, rest) -> go (v :: acc) rest
    | Seq.Cons (Error e, _) -> Error e
  in
  go [] seq
```

Libraries like `Streaming` provide `Source.t` with built-in error handling, mirroring Rust's `fallible_iterator` crate.

## Key Differences

1. **Type in std**: Rust's `std::io::Lines` uses `Iterator<Item=Result<String, io::Error>>` in the standard library; OCaml's `Seq.t` is pure and requires manual wrapping.
2. **`collect` integration**: Rust's `collect::<Result<Vec<T>, E>>()` works out of the box; OCaml needs custom accumulation.
3. **Async extension**: Rust's `fallible_iterator` pattern maps directly to `tokio_stream::Stream`; OCaml's async equivalents (`Lwt_stream`) have similar but different APIs.
4. **Early termination**: Rust's `for` loop with `?` inside is a first-class early-return mechanism; OCaml requires explicit recursion or a custom combinator.

## Exercises

1. Add a `skip_errors` method to `LineParser` that filters out `Err` items and only yields `Ok` values as a new iterator type.
2. Implement a `FallibleZip` iterator that zips two fallible iterators and returns `Err` if either source fails.
3. Write a function that reads lines from a `BufReader<File>` using `Lines`, parses each as an `i64`, and collects them with the fallible collect pattern.
