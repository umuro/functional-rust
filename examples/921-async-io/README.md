📖 **[View on hightechmind.io →](https://hightechmind.io/rust/921-async-io)**

---

# 921-async-io — Async I/O
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Blocking I/O operations — reading files, making network requests, querying databases — pause the calling thread for potentially seconds. In a single-threaded server, this means all other requests wait. The solution is asynchronous I/O: initiate the operation, yield control while waiting, resume when the result is ready. OCaml's `Lwt` library implements this with promises; Python uses `asyncio`; JavaScript uses `Promise`. Rust's `async/await` syntax desugars to state machines (Futures) with no runtime overhead. This example shows thread-based async I/O as a foundation before introducing the `async` keyword.

## Learning Outcomes

- Offload blocking I/O to threads using `std::sync::mpsc` channels
- Understand `spawn_io_task` as a manual implementation of async I/O
- Process text statistics (lines, words, chars) from asynchronously fetched data
- Use `BufRead` and `Write` traits for buffered I/O
- Compare with OCaml's `Lwt_unix` and `Lwt_io` for non-blocking I/O

## Rust Application

`spawn_io_task<T>` spawns a thread and returns an `mpsc::Receiver<T>` — the caller can do other work until `.recv()` is called. `read_string_async` simulates I/O latency with a sleep then returns the content. `process_text` computes (lines, words, chars) statistics synchronously — this is the CPU-bound part that could run concurrently with I/O. `write_to_buf` uses the `Write` trait for buffered output. The pattern: spawn I/O tasks, do CPU work, then collect results.

## OCaml Approach

OCaml's `Lwt_unix.read` and `Lwt_io.read_line` are the async I/O primitives. `Lwt.bind (Lwt_io.read_file path) (fun content -> Lwt.return (process content))` is the promise-chaining equivalent. OCaml 5's `Eio` library uses structured concurrency with fibers. For synchronous I/O in OCaml, `In_channel.input_all` reads a whole file. The big difference: OCaml's async model is cooperative (green threads), Rust's standard library uses OS threads for async, with `tokio`/`async-std` for true async I/O.

## Key Differences

1. **Thread vs future**: Rust std uses OS threads for async-like behavior; true `async/await` requires a runtime (tokio, async-std); OCaml's `Lwt` uses cooperative green threads.
2. **Channel as future**: Rust `mpsc::Receiver<T>` serves as a manual "future" — `.recv()` blocks until ready; OCaml `Lwt.t` is a typed promise.
3. **Backpressure**: Thread-based approach has no built-in backpressure; `tokio` and `Lwt` both support backpressure via buffer bounds.
4. **Error handling**: Both languages surface I/O errors as `Result`/`option`; Rust `?` operator is more ergonomic than OCaml's explicit error threading.

## Exercises

1. Implement `read_multiple_files(paths: &[&str]) -> Vec<String>` using parallel threads and mpsc to read all files concurrently.
2. Write a pipeline that reads text, processes it in a worker thread, and streams formatted output back via a channel.
3. Implement a rate-limited I/O function that spawns tasks but sleeps between spawns to avoid overwhelming the I/O subsystem.
