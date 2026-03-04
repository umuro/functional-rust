# 062: Writer Monad

**Difficulty:** 3  **Level:** Advanced

Accumulate a log or audit trail alongside a computation without passing a log buffer to every function.

## The Problem This Solves

You're building a computation pipeline. You want to log what happened at each step — for debugging, for auditing, for observability. The simplest approach: add `log: &mut Vec<String>` to every function signature.

```rust
fn add(x: i32, y: i32, log: &mut Vec<String>) -> i32 {
    log.push(format!("Adding {} + {} = {}", x, y, x + y));
    x + y
}

fn multiply(x: i32, y: i32, log: &mut Vec<String>) -> i32 {
    log.push(format!("Multiplying {} * {} = {}", x, y, x * y));
    x * y
}

fn compute(a: i32, b: i32, c: i32, log: &mut Vec<String>) -> i32 {
    let sum     = add(a, b, log);
    let product = multiply(sum, c, log);
    product
}
```

This works. But now the log buffer is tangled into every function signature. Your `add` function is no longer a pure math operation — it has a side effect dependency. Testing `add` in isolation requires creating a log buffer even if you don't care about the log in that test. Refactoring from `Vec<String>` to a structured log type means touching every function signature.

The Writer monad decouples the accumulation from the computation. Each function returns `(value, log_entries)` — a pair. The Writer wrapper automatically merges the log entries as you chain functions, so you never thread `&mut log` through anything. The final result carries the complete log, and you read it off at the end.

Conceptually: instead of `fn f(x) -> A`, every step is `fn f(x) -> (A, Log)`. Writer makes chaining these pairs clean. This exists to solve exactly that pain.

## The Intuition

Imagine every step of your computation is a machine that produces two outputs: the result and a sticky note describing what it did. You chain the machines: the result of machine A flows into machine B, but the sticky notes from both get stacked together. At the end, you have the final result *and* a complete stack of sticky notes describing everything that happened.

```
[Machine A: add 3+4] → value: 7,  note: "Adding 3 + 4 = 7"
[Machine B: mul 7×2] → value: 14, note: "Multiplying 7 * 2 = 14"

Combined result: value=14, log=["Adding 3 + 4 = 7", "Multiplying 7 * 2 = 14"]
```

You never had to carry the sticky note stack manually — the Writer chain handled it.

**Jargon decoded:**
- *Writer monad* — a wrapper around `(value, log)` pairs with a `bind` that automatically merges logs
- *`pure(x)`* — wrap `x` with an empty log
- *`tell(msg)`* — produce no value (`()`), but emit a log entry
- *`and_then`* — chain two `Writer` values: result flows forward, logs get merged
- *Monoid* — anything you can "append" to itself with an identity (empty) element. `Vec<String>` is a monoid: you can extend it with more entries, and empty is `vec![]`. The Writer monad works with any monoid as the log type.

## How It Works in Rust

```rust
// Writer carries both a value and an accumulated log
#[derive(Debug, Clone)]
struct Writer<A> {
    value: A,
    log: Vec<String>,
}

impl<A> Writer<A> {
    // Wrap a plain value — log starts empty
    fn pure(a: A) -> Self {
        Writer { value: a, log: vec![] }
    }

    // Emit a log entry — value is ()
    fn tell(msg: String) -> Writer<()> {
        Writer { value: (), log: vec![msg] }
    }

    // Chain: run f on value, merge both logs together
    fn and_then<B>(self, f: impl FnOnce(A) -> Writer<B>) -> Writer<B> {
        let Writer { value: b, log: log2 } = f(self.value);
        let mut log = self.log;
        log.extend(log2);          // merge: self's log + f's log
        Writer { value: b, log }
    }

    // Transform value without changing the log
    fn map<B>(self, f: impl FnOnce(A) -> B) -> Writer<B> {
        Writer { value: f(self.value), log: self.log }
    }
}
```

```rust
// Building a logged computation:
fn add_with_log(x: i32, y: i32) -> Writer<i32> {
    Writer::tell(format!("Adding {} + {}", x, y))
        .and_then(move |()| {
            let sum = x + y;
            Writer::tell(format!("Result: {}", sum))
                .map(move |()| sum)      // attach the value after logging
        })
}

fn multiply_with_log(x: i32, y: i32) -> Writer<i32> {
    Writer::tell(format!("Multiplying {} * {}", x, y))
        .map(move |()| x * y)            // log first, then produce value
}
```

```rust
// Chain them: logs accumulate automatically
fn computation() -> Writer<i32> {
    add_with_log(3, 4)
        .and_then(|sum| multiply_with_log(sum, 2))
        .and_then(|product| {
            Writer::tell("Done!".to_string()).map(move |()| product)
        })
}

let w = computation();
println!("Result: {}", w.value);  // 14
println!("Log: {:?}", w.log);
// ["Adding 3 + 4", "Result: 7", "Multiplying 7 * 2", "Done!"]
// All steps logged, nothing passed manually
```

```rust
// Writer as a general accumulator — collect filtered values:
fn gather_evens(xs: &[i32]) -> Writer<()> {
    xs.iter().fold(Writer::pure(()), |acc, &x| {
        acc.and_then(move |()| {
            if x % 2 == 0 {
                Writer { value: (), log: vec![format!("{}", x)] }
            } else {
                Writer::pure(())  // odd numbers contribute nothing
            }
        })
    })
}

let evens = gather_evens(&[1, 2, 3, 4, 5, 6]);
println!("{:?}", evens.log);  // ["2", "4", "6"]
// Writer used as a collector, not just a logger
```

## What This Unlocks

- **Audit trails:** Wrap business logic steps in Writer. The returned log is a complete record of every decision made, ready for serialization or display — without a logging framework or global logger.
- **Computation explanation:** An interpreter or calculator can return `(result, Vec<StepExplanation>)` via Writer, letting the caller show the work (like a calculator's history display) without the interpreter knowing anything about UI.
- **Filtered collection:** Use Writer's log as an output channel for values that *might* be emitted at each step — like a stream-to-batch collector. Each step either emits to the log or doesn't, and the chain collects everything.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type | `type ('a, 'w) writer = 'a * 'w` (tuple, no wrapper needed) | `struct Writer<A> { value: A, log: Vec<String> }` |
| Log appending | `@` operator (list concatenation, immutable) | `Vec::extend` (mutate-in-place, more efficient) |
| Generic log type | `('a, 'w) writer` where `'w` is any monoid | Typically specialized to `Vec<String>`; generic version needs a `Monoid` trait (not in stdlib) |
| `tell` | `tell msg = ((), [msg])` | `Writer::tell(msg)` — creates `Writer { value: (), log: vec![msg] }` |
| Ownership of log | Immutable lists shared freely via GC | Log vector is *moved* through the chain — each `extend` consumes the old log |
| Stdlib support? | No | No — must implement; `tracing` crate solves this in production |
