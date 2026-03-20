📖 **[View on hightechmind.io →](https://hightechmind.io/rust/329-async-stream)**

---

# 329: Async Streams
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Database cursor results, paginated API responses, file lines, and network byte streams all produce data incrementally. Loading everything into memory before processing is impractical for large datasets. The `Stream` trait (async equivalent of `Iterator`) yields values one at a time, allowing the consumer to process each before the next is produced. Async streams are the foundation of streaming ETL, real-time data processing, and lazy I/O pipelines.

## Learning Outcomes

- Understand `Stream` as the async equivalent of `Iterator` — lazy, sequential, potentially infinite
- Implement a `RangeStream` using Rust's `Iterator` trait as a synchronous analogy
- Use `map()`, `filter()`, and `collect()` on synchronous streams as preparation for async streams
- Recognize that `tokio_stream` and `futures::Stream` provide the full async streaming API

## Rust Application

The `RangeStream` implements `Iterator` to demonstrate streaming patterns:

```rust
pub struct RangeStream { current: i64, end: i64 }

impl Iterator for RangeStream {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current >= self.end { return None; }
        let v = self.current;
        self.current += 1;
        Some(v)
    }
}

// Process lazily — only compute what's consumed
let sum: i64 = RangeStream::new(0, 1_000_000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .take(100)
    .sum();
// Never materializes the full range
```

## OCaml Approach

OCaml's `Seq` module is the synchronous lazy stream type, directly analogous to this iterator-based stream:

```ocaml
let range start end_ =
  Seq.unfold (fun n -> if n >= end_ then None else Some (n, n+1)) start

(* Lazy processing: *)
let sum = Seq.fold_left (+) 0
  (Seq.take 100 (Seq.filter_map
    (fun x -> if x mod 2 = 0 then Some (x*x) else None)
    (range 0 1_000_000)))
```

## Key Differences

1. **Async streams**: Rust's `futures::Stream` / `tokio_stream::Stream` is the async version with `poll_next()` instead of `next()`; OCaml's `Lwt_stream` is the equivalent.
2. **Backpressure**: Async streams naturally implement backpressure — the producer only generates the next item when the consumer polls for it.
3. **Real-world use**: `tokio_stream::wrappers::ReceiverStream` wraps a channel as a stream; `tokio_stream::StreamExt::timeout_repeating` adds retry logic.
4. **Generator syntax**: The `async-stream` crate provides `stream! { yield value; }` syntax for ergonomic async stream creation.

## Exercises

1. Implement a `FibonacciStream` that yields Fibonacci numbers indefinitely.
2. Build a pipeline using stream adapters: take a `RangeStream`, filter even numbers, square them, and collect only the first 10 results.
3. Simulate an async data source by implementing a stream that yields values with configurable delays between items.
