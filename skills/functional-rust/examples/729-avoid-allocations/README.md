# 729: Avoid Allocations

**Difficulty:** 3  **Level:** Advanced

Keep hot paths allocation-free by reusing buffers, using iterators, and choosing stack-friendly types.

## The Problem This Solves

Every heap allocation has overhead: the allocator must find a free block, maintain metadata, potentially zero memory, and eventually free it. For code that runs millions of times — a web server handling requests, a game loop processing entities, a parser tokenising input — repeated small allocations add measurable latency and unpredictable GC pauses (in GC languages) or fragmentation (in manual-memory languages).

Rust gives you precise control: you can format a string into a pre-allocated buffer, process a sequence through iterator chains without creating intermediate `Vec`s, reuse a `Vec` by clearing it instead of dropping it, and choose stack-allocated types (`[u8; 64]`, `SmallVec`, `ArrayVec`) for small collections that never need to grow large. None of this requires `unsafe` — it's disciplined use of Rust's standard APIs.

## The Intuition

The key question for any hot path: "Does this expression allocate?" A few patterns that always allocate: `String::new()`, `to_string()`, `format!()`, `Vec::new()`, `.collect::<Vec<_>>()`, `.to_owned()`, `Box::new()`, `.clone()` on non-Copy types. A few patterns that never allocate: iterator chains (`.map()`, `.filter()`, `.sum()`), `&str` and `&[u8]` borrows, `write!` into an existing `String`, `.clear()` on a `Vec` or `String`.

The goal is not to eliminate all allocation — that's premature optimisation. The goal is to identify the hot path (profiler-measured), understand which allocations are inside it, and eliminate the unnecessary ones by reusing, borrowing, or using stack-local types.

## How It Works in Rust

```rust
// ── Technique 1: Write into a pre-allocated buffer ────────────────────────
fn format_record_into(buf: &mut String, name: &str, score: u32) {
    buf.clear();        // resets content, keeps heap capacity — no allocation
    buf.push_str(name);
    buf.push(':');
    // Integer without temporary String:
    let mut tmp = [0u8; 20];
    buf.push_str(u32_to_str(score, &mut tmp));
}
// Caller allocates once:
let mut buf = String::with_capacity(64);
for record in records {
    format_record_into(&mut buf, record.name, record.score);
    send(&buf);  // borrow — no clone
}

// ── Technique 2: Iterator chains — zero intermediate collections ──────────
fn sum_squares(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum()  // no Vec — lazy computation, one pass
}

fn hot_filter_sum(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .sum()           // single pass, zero intermediate allocations
}

// ── Technique 3: Reuse a Vec by clearing, not dropping ───────────────────
struct Pipeline { scratch: Vec<i32> }
impl Pipeline {
    fn new() -> Self { Pipeline { scratch: Vec::with_capacity(1024) } }
    fn process(&mut self, input: &[i32]) -> &[i32] {
        self.scratch.clear();          // O(1) length reset, capacity preserved
        for &x in input {
            if x.rem_euclid(2) == 0 { self.scratch.push(x * 3); }
        }
        &self.scratch
    }
}

// ── Technique 4: Borrow instead of clone ─────────────────────────────────
fn print_info(name: &str) { println!("{}", name); }  // &str, not String
```

Measure with `cargo flamegraph`, `heaptrack`, or `valgrind --tool=massif` before optimising. Allocation costs are workload-dependent — what's bottleneck in one context is noise in another.

## What This Unlocks

- **Web server request handling** — pre-allocate response buffers per connection, reuse across requests; eliminates allocation on every HTTP response header write.
- **Embedded and no-std** — stack-only computation is mandatory when there is no heap; iterator chains and stack buffers are the only option.
- **Latency-sensitive systems** — trading systems, audio processing, and real-time games require predictable latency; heap allocation introduces jitter from the allocator.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Buffer reuse | `Buffer.clear buf` | `buf.clear()` on `Vec<u8>` or `String` |
| Iterator without intermediate | `Seq` (lazy) | `Iterator` chains — lazy, zero-allocation |
| Stack buffer for integer | `string_of_int n` (allocates) | Manual `[u8; 20]` + write-in-place |
| Clone vs borrow | Pattern matching usually copies | `&str` / `&[T]` borrows — explicit no-copy |
| Allocation profiling | `spacetime` profiler | `heaptrack`, `cargo-instruments`, `valgrind massif` |
