# 194: Coroutines and Generators via Effects

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Implement generators that lazily yield a sequence of values — like Python's `yield` — using Rust's stable Iterator trait, state-machine enums, and callback-based collection, all simulating OCaml 5's native effect-based approach.

## The Problem This Solves

Generating sequences lazily is a common need: range values, Fibonacci numbers, lines from a file, events from a stream. The naive approach is to collect everything into a `Vec` first, but this breaks for infinite sequences and wastes memory for large ones. You want to produce values one at a time, pausing between each — that's what Python's `yield` keyword does.

Python generators work by *suspending* execution: `yield x` pauses the function at that point, hands `x` to the caller, and resumes from the same point on the next `next()` call. This is a coroutine — a function that can pause and resume. Without language support, you'd have to manually encode this pause/resume logic as a state machine. Rust's nightly generators do this automatically; in stable Rust we do it ourselves.

OCaml 5 implements this with algebraic effects: a `Yield_val` effect pauses the generator and hands the value to the handler, which can resume the generator for the next value. Rust stable offers multiple equivalent patterns: the `Iterator` trait captures the exact same semantics for simple cases, while state-machine enums let you encode more complex coroutines explicitly.

## The Intuition

A coroutine is like a lazy letter-writer. Instead of writing the whole letter and sending it at once, they write one sentence, hand it to you, wait for you to read it, then write the next sentence. You control the pacing — you ask for the next sentence whenever you're ready. The letter-writer keeps their place between sentences.

In Rust, `Iterator` is exactly this protocol: `next()` asks for one more value, the iterator remembers where it left off, and it returns `None` when done. A state-machine coroutine encodes "where the function is paused" as an enum variant. A callback-based generator (`fn generator<F>(f: F) -> Vec<i64>`) collects all yields eagerly — sacrificing laziness but making the API simple.

## How It Works in Rust

```rust
// Approach 1: Iterator — the most idiomatic stable-Rust generator
struct RangeGenerator { current: i64, high: i64 }

impl Iterator for RangeGenerator {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current > self.high { return None; }
        let v = self.current;
        self.current += 1;   // advance state — this is the "resume"
        Some(v)
    }
}

// Usage:
let sum: i64 = RangeGenerator::new(1, 100).sum();  // lazy, no Vec allocation
let squares: Vec<i64> = RangeGenerator::new(1, 5).map(|x| x * x).collect();

// Approach 2: State-machine coroutine — for multi-step logic between yields
#[derive(Debug)]
enum CoroutineState { Start, AfterFirst, AfterSecond, Done }

struct TwoStepCoroutine { state: CoroutineState, name: &'static str }

impl TwoStepCoroutine {
    fn resume(&mut self) -> Option<String> {
        match self.state {
            CoroutineState::Start => {
                self.state = CoroutineState::AfterFirst;
                Some(format!("{}: step 1", self.name))  // yield
            }
            CoroutineState::AfterFirst => {
                self.state = CoroutineState::AfterSecond;
                Some(format!("{}: step 2", self.name))  // yield
            }
            CoroutineState::AfterSecond => {
                self.state = CoroutineState::Done;
                Some(format!("{}: done", self.name))   // final yield
            }
            CoroutineState::Done => None,              // finished
        }
    }
}

// Interleave two coroutines — cooperative scheduling:
let mut task_a = TwoStepCoroutine::new("A");
let mut task_b = TwoStepCoroutine::new("B");
loop {
    let a = task_a.resume();
    let b = task_b.resume();
    if a.is_none() && b.is_none() { break; }
    // prints: A step1, B step1, A step2, B step2, A done, B done
}

// Approach 3: Callback-based (closest to OCaml's effect model)
fn generator<F: Fn(&mut dyn FnMut(i64))>(f: F) -> Vec<i64> {
    let mut values = Vec::new();
    f(&mut |v| values.push(v));  // yield_val closure = the Yield_val effect
    values
}

fn fibonacci_gen(count: usize, yield_val: &mut dyn FnMut(i64)) {
    let (mut a, mut b) = (0_i64, 1_i64);
    for _ in 0..count {
        yield_val(a);   // "perform (Yield_val a)" in OCaml
        let next = a + b; a = b; b = next;
    }
}

let fibs = generator(|y| fibonacci_gen(8, y));
// [0, 1, 1, 2, 3, 5, 8, 13]
```

In OCaml 5, `perform (Yield_val n)` suspends the generator and runs the handler's `continue k ()` to resume it. Rust's `Iterator::next()` is the same protocol — the caller drives resumption. The state-machine enum is how Rust's `async/await` and nightly generators are compiled internally.

## What This Unlocks

- **Lazy sequences over large datasets** — process lines from a file, events from a socket, or tree nodes in a traversal without loading everything into memory first.
- **Cooperative multitasking** — interleave multiple coroutines by taking turns calling `resume()`; this is the foundation of async runtimes like Tokio before they had OS threads.
- **Infinite sequences** — a coroutine that generates primes, Fibonacci numbers, or random values has no natural end; `Iterator` adapters (`take(N)`, `take_while(...)`) terminate the infinite stream lazily.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Yielding a value | `perform (Yield_val n)` — suspends, gives `n` to handler | `yield_val(n)` callback (eager) or `Some(n)` from `Iterator::next` (lazy) |
| Resuming execution | `continue k ()` — native delimited continuation | `Iterator::next()` calls the `next()` method again; state machine advances |
| Suspend point encoding | Implicit — the effect captures the continuation | Explicit — enum variant records "where we are" |
| Infinite sequences | Natural — generator loops forever, handler pulls values | Iterator with `None` termination; infinite iterators via `.take()` |
| Nightly Rust generators | N/A | `#![feature(generators)]` gives Python-like `yield` syntax with compiler-generated state machines |
