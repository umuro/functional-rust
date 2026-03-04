# 190: Effect Handler — Logging and Nondeterminism

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Two effect handler patterns: a logging handler that collects messages without interrupting computation, and a nondeterminism handler that runs all branches and returns every possible result.

## The Problem This Solves

Logging is normally a side effect woven through code — `log::info!("...")` statements scattered everywhere, writing to a global logger. You can't easily capture logs in tests without configuring logging infrastructure. You can't count log entries without parsing output. The logs are effects that escape the computation, and reigning them back in requires mocking at the infrastructure level.

Nondeterminism is worse — most languages have no native support for "run this code for each possible choice and collect all results." You'd write explicit loops, maintain intermediate lists, propagate results manually. The code that generates results and the code that collects them are tangled together.

Effect handlers give you a clean separation for both. A `Log` effect says "record this message." The *handler* decides what to do with it: collect to a vec, print to stdout, count occurrences, discard. A `Choose` effect says "pick one of these values." The *handler* decides to run the continuation once per option and combine results. The computation stays pure; the handler owns the policy.

## The Intuition

**Logging:** A writer in a recording studio wears noise-cancelling headphones and writes lyrics, occasionally calling out "this line is temporary" to their assistant. The assistant writes it down without interrupting the lyrics session. Later, they review the notes. The logging effect is like calling out — the writer (computation) doesn't change behavior, but the assistant (handler) captures every call.

**Nondeterminism:** A story with branching paths: "You find a fork in the road. Take the left path, or the right path?" An exhaustive reader reads the story once for each possible choice and collects all endings. That's the nondeterminism handler — for each `Choose [1, 2, 3]`, run the rest of the computation three times and gather all results. This is exactly the **list monad**: `bind` for lists is `flat_map`.

## How It Works in Rust

```rust
// ── Logging Effect ─────────────────────────────────────────────────────

enum LogStep<A> {
    Done(A),
    Log(String, Box<dyn FnOnce() -> LogStep<A>>),  // log this message, then continue
}

fn log_msg<A: 'static>(msg: impl Into<String>, next: LogStep<A>) -> LogStep<A> {
    LogStep::Log(msg.into(), Box::new(move || next))
}

// Handler: collect all log messages
fn collect_logs<A>(mut step: LogStep<A>) -> (A, Vec<String>) {
    let mut logs = Vec::new();
    loop {
        match step {
            LogStep::Done(x) => return (x, logs),
            LogStep::Log(msg, cont) => {
                logs.push(msg);    // intercept the log message
                step = cont();     // resume the computation
            }
        }
    }
}

// Usage:
let program = log_bind(
    log_msg("starting", log_done(())),
    |_| log_bind(
        log_msg("x = 42", log_done(())),
        |_| log_done(84_i32)
    )
);
let (result, logs) = collect_logs(program);
// result = 84, logs = ["starting", "x = 42"]

// ── Nondeterminism Effect: the list monad ──────────────────────────────

// "Choose" doesn't need a tree structure — it maps directly to the list monad
// bind for lists = flat_map: run f for every element, collect all results
fn nd_bind<A, B, F: Fn(A) -> Vec<B>>(xs: Vec<A>, f: F) -> Vec<B> {
    xs.into_iter().flat_map(f).collect()
}

fn nd_choose<A: Clone>(xs: Vec<A>) -> Vec<A> { xs }  // "perform Choose xs"
fn nd_guard(cond: bool) -> Vec<()> { if cond { vec![()] } else { vec![] } }

// Find all pairs (a,b) where a+b == 5, a,b in [1..4]:
let pairs = nd_bind(nd_choose(vec![1, 2, 3, 4]), |a| {
    nd_bind(nd_choose(vec![1, 2, 3, 4]), move |b| {
        nd_bind(nd_guard(a + b == 5), move |_| vec![(a, b)])
    })
});
// [(1,4), (2,3), (3,2), (4,1)] — all combinations, automatically

// Pythagorean triples — same pattern, different predicate:
let triples = nd_bind(nd_choose((1..=10).collect()), |a| {
    nd_bind(nd_choose((a..=10).collect()), move |b| {
        nd_bind(nd_choose((b..=10).collect()), move |c| {
            nd_bind(nd_guard(a*a + b*b == c*c), move |_| vec![(a,b,c)])
        })
    })
});
```

The nondeterminism handler reveals a deep truth: **the list monad _is_ the handler for a Choose effect**. In OCaml, `continue k x` runs the rest of the program with choice `x`; collecting all results means calling `List.concat_map (fun x -> continue k x) xs`. That's exactly `flat_map`.

## What This Unlocks

- **Testable logging** — capture logs as a `Vec<String>` in tests without any global logger; assert on exact log contents without string parsing.
- **Backtracking search** — express constraint satisfaction (Sudoku solvers, regex matching, type unification) as nondeterministic programs; the list monad enumerates all solutions automatically.
- **Alternative effect interpretations** — run the same logging program with a "silent" handler (discard all logs), a "panic on error" handler, or a "first N only" handler without changing the program.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Logging handler | `match program () with \| effect (Log msg) k -> log := msg :: !log; continue k ()` | `loop { match step { LogStep::Log(msg, cont) => { logs.push(msg); step = cont(); } } }` |
| Nondeterminism handler | `match program () with \| effect (Choose xs) k -> List.concat_map (fun x -> continue k x) xs` | `nd_bind(nd_choose(xs), f)` — list monad; no native continuation capture needed |
| Resuming | `continue k value` — built-in, can resume multiple times (for nondeterminism) | Closure called once per branch; for nondeterminism, `flat_map` handles branching implicitly |
| Effect composition | Same `match` handles both Log and Choose — naturally composable | Separate `LogStep` and list monad — composing requires nesting or a combined effect type |
| Purity | OCaml 5 effects are truly first-class; can mix Log and Choose in one program | Rust simulation separates them; mixing requires a unified effect enum |
