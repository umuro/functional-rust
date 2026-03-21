📖 **[View on hightechmind.io →](https://hightechmind.io/rust/861-writer-monad)**

---

# Writer Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Functions that need to accumulate a log, collect diagnostics, or build an audit trail alongside their computation result face a choice: return a tuple `(result, log)` or use a mutable global. The Writer monad encapsulates the log accumulation: `Writer<A>` represents a value `A` paired with a log `Vec<String>`. Computations are composed and their logs automatically concatenated. This separates concerns: the core logic doesn't know about logging; the monad handles it. Use cases: compiler diagnostics (warnings alongside the compiled output), query plan logging, audit trails, and trace accumulation in distributed tracing.

## Learning Outcomes

- Understand `Writer<A>` as `(A, Vec<String>)` — a value paired with an accumulated log
- Implement `tell(msg)` to emit a log entry, `pure(x)` to lift a value with empty log
- Implement monadic bind: concatenate the two computations' logs
- Recognize the constraint: log must form a monoid (empty log + concatenation)
- Apply to: audit logging, compiler warnings, distributed trace accumulation

## Rust Application

```rust
#[derive(Debug, Clone)]
pub struct Writer<A> {
    pub value: A,
    pub log: Vec<String>,
}
impl<A: Clone + 'static> Writer<A> {
    pub fn pure(a: A) -> Self { Writer { value: a, log: vec![] } }
    pub fn tell(msg: impl Into<String>) -> Writer<()> {
        Writer { value: (), log: vec![msg.into()] }
    }
    pub fn bind<B: Clone + 'static>(self, f: impl FnOnce(A) -> Writer<B>) -> Writer<B> {
        let Writer { value: a, log: log1 } = self;
        let Writer { value: b, log: log2 } = f(a);
        Writer { value: b, log: [log1, log2].concat() }
    }
}
```

The `bind` (flatMap) destructures both Writer values and concatenates their logs before returning the final value. This is the key operation: the logs from both computations are combined in order. `tell` produces a `Writer<()>` — it has no meaningful result value, only a log entry. The `[log1, log2].concat()` is equivalent to `log1.into_iter().chain(log2).collect()` but more readable. The `pure` creates a writer with an empty log — the neutral element.

## OCaml Approach

OCaml represents Writer as `type 'a writer = { value: 'a; log: string list }`. `pure a = { value = a; log = [] }`. `tell msg = { value = (); log = [msg] }`. Bind: `let bind w f = let w2 = f w.value in { value = w2.value; log = w.log @ w2.log }`. OCaml's `@` operator appends lists. The Writer monad requires the log type to be a monoid (`[]` for empty, `@` for combine). For performance, `Buffer.t` or `Queue.t` replaces immutable list append. OCaml's `let%bind` with ppx_writer provides do-notation.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Log type | `Vec<String>` | `string list` |
| Log concatenation | `[log1, log2].concat()` | `log1 @ log2` |
| `pure` | `Writer::pure(a)` | `{ value = a; log = [] }` |
| `tell` | `Writer::tell(msg)` | `{ value = (); log = [msg] }` |
| Monoid constraint | `Vec<String>` (implicit) | `string list` or explicit |
| Performance | O(n) per append | O(n) per `@` |

## Exercises

1. Implement a computation that factors a number and logs each factor found using the Writer monad.
2. Replace `Vec<String>` with a `String` buffer and implement a writer that builds a formatted log string.
3. Generalize Writer to `Writer<A, W>` where W must implement a `Monoid` trait with `empty()` and `combine`.
4. Use the Writer monad to collect performance timings alongside computation results.
5. Implement `sequence(writers: Vec<Writer<A>>) -> Writer<Vec<A>>` that runs all writers and concatenates their logs.
