📖 **[View on hightechmind.io →](https://hightechmind.io/rust/886-scan-accumulate)**

---

# 886-scan-accumulate — Scan / Accumulate

## Problem Statement

Fold collapses a sequence to a single value. Scan is fold's sibling: it emits every intermediate accumulator value as an element of a new sequence. Running sums, cumulative maxima, and prefix products are all applications of scan. In financial applications, scan computes balance history from a list of transactions. In signal processing, it computes moving statistics. Haskell has `scanl`, OCaml has `Seq.scan`, and Python's `itertools.accumulate` serves the same role. Rust's `Iterator::scan` adapter is the idiomatic approach — stateful, lazy, and composable with the rest of the iterator ecosystem.

## Learning Outcomes

- Use `.scan(init, |state, item| ...)` to compute running accumulations
- Build running sum, running product, running max, and running min in one pass
- Implement balance history from a sequence of transactions using scan
- Understand the difference between fold (single result) and scan (sequence of results)
- Compare with OCaml's `Seq.scan` and Python's `itertools.accumulate`

## Rust Application

`running_sum` uses `data.iter().scan(0, |acc, &x| { *acc += x; Some(*acc) }).collect()`. The closure receives `&mut state` and the current element, returns `Some(value)` to emit a value. `running_max` uses `scan(i32::MIN, |max, &x| { *max = (*max).max(x); Some(*max) })`. `balance_history` prepends the initial balance to transactions and scans them to produce a balance timeline. `running_zscore` computes a normalized running deviation in a single scan pass.

## OCaml Approach

OCaml's `Seq.scan f init xs` produces a lazy sequence of intermediate values. `List.fold_left` is used for the eager version, collecting intermediate values with a `ref` accumulator. `scan_left` in the example uses a helper that returns `init :: (fold values)` — the list of all accumulator states including the initial. OCaml's `Seq.scan` is lazy like Rust's; `List`-based scan is eager.

## Key Differences

1. **Mutability**: Rust scan uses `&mut state` — the state is explicitly mutable inside the closure; OCaml uses functional state threading.
2. **Early termination**: Rust scan can return `None` to stop early (like a conditional scan-while); OCaml requires explicit `Seq.take_while` wrapping.
3. **Initial value**: Rust `scan` starts emitting from the first transformed value (not the initial state); OCaml's `scan_left` typically includes the initial state.
4. **Laziness**: Both are lazy when using `Seq`/iterator form; both require explicit collection for eager evaluation.

## Exercises

1. Implement `running_variance` using scan to track count, sum, and sum-of-squares in a single pass.
2. Write `scan_until_zero` that computes running products but stops (returns `None`) when a zero is encountered.
3. Implement `cumulative_max_index` that tracks both the running maximum and the index where it was last updated.
