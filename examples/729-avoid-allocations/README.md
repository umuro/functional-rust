📖 **[View on hightechmind.io →](https://hightechmind.io/rust/729-avoid-allocations)**

---

# 729-avoid-allocations — Avoid Allocations

## Problem Statement

Heap allocation is not free. Every `Box::new`, `Vec::push`, or `String::from` call invokes the global allocator, which acquires a lock, searches a free list, and may trigger a system call. In latency-sensitive code — game loops, network packet handlers, audio DSP — even a single allocation per frame causes jitter. The solution is to pre-allocate once and reuse, use iterator chains that produce zero intermediate collections, and prefer stack-local buffers for short-lived data.

## Learning Outcomes

- Reuse a `String` or `Vec` buffer by calling `clear()` instead of dropping and reallocating
- Write integer-to-string conversion using a stack buffer (`[u8; 20]`) with no heap
- Build zero-allocation iterator pipelines with `filter`, `map`, and `sum`
- Use `Vec::with_capacity` to avoid repeated reallocations when the size is known
- Understand the trade-off between pre-allocation cost and per-call allocation cost

## Rust Application

`format_record_into` takes a `&mut String`, calls `buf.clear()` (which preserves heap capacity), then pushes fields directly. `u32_to_str` writes digits into a `[u8; 20]` stack array and returns a `&str` slice — zero heap. `sum_squares` and `hot_filter_sum` use lazy iterators: no `Vec` is ever created. The `Pipeline` struct holds a `Vec<i32>` with a large pre-allocated capacity and clears it each call.

## OCaml Approach

OCaml uses a generational GC so short-lived allocations are cheap (minor heap bump pointer). Avoiding allocations is less critical in OCaml than in Rust. However, for C-interop and FFI-heavy code, OCaml uses `Bytes.t` buffers passed by reference, and `Buffer.t` (similar to Rust's reusable `String`) for building strings incrementally without intermediate copies.

## Key Differences

1. **Cost model**: In Rust, every allocation is explicit and potentially blocking; in OCaml, minor-heap allocation is nearly free (bump pointer, ~2 ns), making avoidance less urgent.
2. **Buffer reuse**: Rust's `buf.clear()` is idiomatic for string reuse; OCaml's `Buffer.clear` serves the same purpose but the GC would collect the old buffer anyway.
3. **Stack strings**: Rust can use `[u8; N]` + `str::from_utf8` for zero-heap text; OCaml has no direct stack allocation (all values go through the GC).
4. **Iterator laziness**: Both languages have lazy iterators (`Seq` in OCaml, `Iterator` in Rust); Rust's borrow checker enforces that no intermediate `Vec` is created accidentally.

## Exercises

1. Modify `format_record_into` to write a CSV line (name, score, timestamp) into the buffer without allocating any intermediate strings.
2. Implement a `hot_path_process` function that filters, transforms, and sums a `&[f64]` slice using only iterator combinators — verify with `cargo bench` that it allocates zero bytes.
3. Build a `ScratchPool` that pre-allocates 16 `String` buffers and hands them out round-robin, measuring throughput vs. a naïve `String::new()` approach.
