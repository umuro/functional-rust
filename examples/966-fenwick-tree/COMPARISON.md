# Fenwick Tree — Comparison

## Core Insight
The Fenwick tree (BIT) is an implicit tree encoded in a 1-indexed array. The `lowbit(i) = i & (-i)` operation extracts the lowest set bit and drives both update (walk up: `i += lowbit(i)`) and query (walk down: `i -= lowbit(i)`). Simpler than a segment tree but more limited (prefix sums only, harder to generalize). Both languages implement exactly this bit trick.

## OCaml Approach
- `let lowbit i = i land (-i)` — OCaml's bitwise AND
- `i := !i + lowbit !i` — mutable ref for loop counter
- `let i = ref (i + 1)` — convert 0-indexed to 1-indexed
- `while !i <= fw.n do ... done` — imperative loop
- Separate `update` (add delta) and `prefix_sum` functions

## Rust Approach
- `idx & (-idx)` — Rust requires `i64` for negation (usize can't be negative)
- `let mut idx = (i + 1) as i64` — convert and use signed for arithmetic
- `while idx > 0 { ... idx -= idx & (-idx) }` — loop with mutable `idx`
- `self.set()` adds a convenience "set to value" operation
- `from_slice` constructor for batch initialization

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Lowbit | `i land (-i)` | `idx & (-idx)` (i64) |
| Index type | `int` (signed, 63-bit) | `i64` (for negation) then `usize` |
| Update loop | `i := !i + lowbit !i` | `idx += idx & (-idx)` |
| Query loop | `i := !i - lowbit !i` | `idx -= idx & (-idx)` |
| 0→1 index | `let i = ref (i + 1)` | `let mut idx = (i + 1) as i64` |
| Delta update | `add delta` only | `update(delta)` + `set(value)` |
| Array access | `fw.tree.(!i)` | `self.tree[idx as usize]` |
