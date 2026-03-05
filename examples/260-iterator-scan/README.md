📖 **[View on hightechmind.io →](https://hightechmind.io/rust/260-iterator-scan)**

---

# 260: Stateful Accumulation with scan()

**Difficulty:** 2  **Level:** Intermediate

Like `fold`, but yields each intermediate accumulator value as part of the output sequence.

## The Problem This Solves

`fold` collapses a sequence to one final value — useful for totals, but it discards all the intermediate states. When you need the *running* total — a prefix sum, a cumulative product, a balance history — `fold` alone can't help. You'd need a mutable accumulator and a manual push into a results vector.

Running totals appear everywhere: bank statement balances, time-series prefix sums, running maximum in a streaming algorithm, state machine output. Each step's result depends on the previous step's state. Without `scan`, you write an imperative loop with two mutable variables — the state and the output vector — mixing concerns that should be separate.

`scan` threads a mutable state through the iteration (like `fold`) while also emitting each updated state as an iterator element. You get both: the evolving state *and* a lazy sequence of every intermediate value. Returning `None` from the closure also gives you early termination — fold a prefix of the sequence until a condition is met.

## The Intuition

`scan(initial_state, |state, item| -> Option<output>)` is `fold` that yields each step. The closure receives `&mut state` (mutate it in place) and the current item, then returns `Some(value)` to emit a value and continue, or `None` to stop the iterator early.

```rust
let running_sum: Vec<i32> = [1, 2, 3, 4, 5].iter()
    .scan(0i32, |state, &x| { *state += x; Some(*state) })
    .collect();
// → [1, 3, 6, 10, 15]
```

The state persists across calls — each invocation of the closure sees the state left by the previous call. The initial value sets the state before any item is processed.

## How It Works in Rust

```rust
let nums = [1i64, 2, 3, 4, 5];

// Running sum — state is mutated, current state is emitted
let running_sum: Vec<i64> = nums.iter()
    .scan(0i64, |state, &x| {
        *state += x;     // state is &mut i64 — mutate through dereference
        Some(*state)     // emit the new state value
    })
    .collect();
// → [1, 3, 6, 10, 15]

// Early termination — return None to stop the iterator
let partial: Vec<i64> = nums.iter()
    .scan(0i64, |state, &x| {
        *state += x;
        if *state > 6 { None } else { Some(*state) }  // stop when sum exceeds 6
    })
    .collect();
// → [1, 3, 6]   (stops before adding 4, which would give 10)

// Bank balances — same pattern, real-world framing
let transactions = [100i64, -30, 50, -80, 200];
let balances: Vec<i64> = transactions.iter()
    .scan(0i64, |balance, &tx| {
        *balance += tx;
        Some(*balance)
    })
    .collect();
// → [100, 70, 120, 40, 240]
```

The closure signature `|state, item|` — `state` is `&mut S`, so you mutate through dereference. The emitted value does not have to equal the state — you can emit a transformation of the state.

## What This Unlocks

- **Prefix sums and running totals** — cumulative sums for range queries, balance histories, score progressions.
- **Streaming state machines** — emit the current state at each transition; stop on a terminal state by returning `None`.
- **Early-exit aggregation** — fold that stops as soon as a threshold is crossed, without consuming the rest of the iterator.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Running accumulation | Manual `fold_left` with list append | `iter.scan(init, \|state, x\| ...)` |
| Early termination | Raise exception or use lazy `Seq` | Return `None` from closure |
| State mutability | New value returned each step | `state` is `&mut S`, mutated in place |
| Laziness | No (strict lists) | Yes — elements produced on demand |
| Emit vs state | Always emits the accumulator | Can emit any transformation of state |
