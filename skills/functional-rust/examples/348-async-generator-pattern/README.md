# 348: Async Generator Pattern

**Difficulty:** 4  **Level:** Expert

Produce values lazily from a function body — like Python's `yield` or JavaScript's `function*`, using a channel.

## The Problem This Solves

Rust doesn't have native generator syntax (no `yield` keyword in stable Rust). But the pattern is extremely useful: compute values one at a time, yielding each to the consumer, without loading all values into memory. Think of an infinite Fibonacci sequence, a prime number stream, or a file parser that yields records as it reads.

The channel-based generator pattern simulates this: a background thread runs the generator body and sends values through a `SyncSender`; the consumer iterates over the `Receiver`. Backpressure comes for free — the `SyncSender` blocks when the buffer is full, so the generator never runs ahead of the consumer. The consumer controls pacing with `.take(n)`.

In async Rust, this maps to `async-stream` or the `Stream` trait — the generator `yield`s items and the consumer polls the stream.

## The Intuition

Python generators:
```python
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

for n in itertools.islice(fibonacci(), 10):
    print(n)
```

This example builds the identical pattern in Rust: `fibonacci_gen()` returns an `impl Iterator<Item=u64>` backed by a channel. The generator runs in a thread, the consumer drives it via iteration.

JavaScript's `async function*` is the async version — Rust's `async-stream` crate provides the equivalent.

## How It Works in Rust

```rust
fn generator<T: Send + 'static>(
    body: impl FnOnce(mpsc::SyncSender<T>) + Send + 'static,
    buffer: usize,
) -> impl Iterator<Item = T> {
    let (tx, rx) = mpsc::sync_channel(buffer);
    thread::spawn(move || body(tx));  // generator runs in background
    rx.into_iter()  // consumer iterates here
}

fn fibonacci_gen() -> impl Iterator<Item = u64> {
    generator(|tx| {
        let (mut a, mut b) = (0u64, 1u64);
        loop {
            if tx.send(a).is_err() { break; }  // consumer dropped — stop
            (a, b) = (b, a.wrapping_add(b));
        }
    }, 8)  // buffer 8 values ahead
}

// Use: infinite sequence, take only what you need
let first_10: Vec<u64> = fibonacci_gen().take(10).collect();
```

When the consumer calls `.take(10).collect()` and drops the iterator, the `Receiver` is dropped. The generator's next `tx.send(...)` returns `Err(SendError)` and `break`s the loop — no leak, clean shutdown.

## What This Unlocks

- **Infinite sequences** — primes, Fibonacci, random numbers, timestamps — without allocating the whole sequence.
- **Streaming parsers** — yield parsed records one at a time as a file is read, processing in constant memory.
- **Test data generators** — generate test cases lazily; take as many as needed per test.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Generator / lazy sequence | `type 'a stream = Cons of 'a * (unit -> 'a stream)` | Channel-based thread + `impl Iterator` |
| Native yield | `Seq.t` (lazy sequences in 4.07+) | No `yield` in stable; use channel or `async-stream` |
| Backpressure | Thunk delays evaluation | `SyncSender` buffer blocks producer |
| Consumer control | `Seq.take n` | `.take(n)` on iterator |
