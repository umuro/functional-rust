## Core Insight

Scan is fold that emits every intermediate accumulator value. `scan_left [1;2;3] (+) 0` gives `[0;1;3;6]` — the running sum. It's the functional equivalent of a cumulative sum.

## OCaml Approach
- No built-in `scan_left` — implement with fold or recursion
- Returns list of all accumulator states
- `List.fold_left` adapted to collect intermediates

## Rust Approach
- `.scan(state, |st, x| ...)` — stateful iterator adapter
- Returns `Option` to allow early termination
- Also achievable with fold collecting into Vec

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Built-in | No | `.scan()` iterator adapter |
| Result | List of accumulators | Iterator of values |
| State | Accumulator in recursion | Mutable state in closure |
