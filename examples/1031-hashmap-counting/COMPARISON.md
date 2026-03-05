# Count Frequencies — Comparison

## Core Insight
Frequency counting is the "hello world" of the Entry API. Both languages use a fold/loop to accumulate counts, but Rust's entry pattern compresses the check-then-update into a single dereference-and-increment.

## OCaml Approach
- `find_opt` + default 0 + `add` with incremented value
- Pattern: `let n = match find_opt k m with Some n -> n | None -> 0 in add k (n+1) m`
- `fold` to find max element
- Each update creates a new map node (immutable)

## Rust Approach
- `*entry(k).or_insert(0) += 1` — one line
- Alternative: `entry(k).and_modify(|c| *c += 1).or_insert(1)` — more explicit
- `max_by_key` on iterator to find most frequent
- Functional style via `iter().fold()` also works

## Comparison Table

| Aspect | OCaml | Rust |
|---|---|---|
| Count idiom | `find_opt` + match + `add` | `*entry(k).or_insert(0) += 1` |
| Lines per count | 4 | 1 |
| Max element | `fold` with comparison | `iter().max_by_key()` |
| Allocation | New map node per update | In-place mutation |
| Style | Always functional | Imperative or functional |
