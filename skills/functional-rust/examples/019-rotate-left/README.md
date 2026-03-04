# 019: Rotate Left

**Difficulty:** 1  **Level:** Foundations

Rotate a list left by N positions — elements that fall off the front wrap around to the back.

## The Problem This Solves

Rotating a list is everywhere: circular buffers in embedded systems, round-robin scheduling, carousel UI components, shift-register simulations. Given `['a','b','c','d','e','f','g','h']` and `n=3`, the result is `['d','e','f','g','h','a','b','c']` — the first three elements moved to the end.

In Python you'd write `lst[n:] + lst[:n]` for a positive rotation, or use `collections.deque` for efficient in-place rotation. This is clean, but you still need to handle negative `n` and `n > len(lst)` manually.

Rust makes both cases explicit and efficient. The modular arithmetic normalizes any `n` — positive, negative, or larger than the list — into a single canonical shift, then a slice operation does the rotation in two array copies with no extra logic.

## The Intuition

In Python: `lst[n:] + lst[:n]`

In JavaScript: `[...lst.slice(n), ...lst.slice(0, n)]`

A left rotation by `n` is really just: "start reading at position `n`, wrap around when you reach the end." Rust's slice concatenation (`lst[shift..]` + `lst[..shift]`) does exactly that.

The tricky part is normalizing `n`. If the list has 8 elements and you rotate by 11, that's the same as rotating by 3 (11 % 8 = 3). If you rotate by -2, that's the same as rotating left by 6 (or right by 2). The formula `((n % len) + len) % len` handles all cases in one line.

## How It Works in Rust

```rust
fn rotate<T: Clone>(lst: &[T], n: i64) -> Vec<T> {
    if lst.is_empty() {
        return vec![];
    }
    let len = lst.len() as i64;
    // Normalize n: handles negative, zero, and n > len
    let shift = ((n % len) + len) as usize % lst.len();
    let mut result = lst[shift..].to_vec();   // elements from shift onward
    result.extend_from_slice(&lst[..shift]);  // wrap-around elements
    result
}
```

- `n % len` — reduce to range `(-len, len)`
- `+ len` — make non-negative (shift negatives into positive range)
- `% lst.len()` — final clamp to `[0, len)`
- `lst[shift..]` + `lst[..shift]` — two slices joined into one Vec

There's also a `rotate_cycle` version that uses `.cycle().skip(shift).take(len)` — the iterator chain reads almost like an English sentence: "cycle the list endlessly, skip the first `shift` items, take exactly `len` items."

## What This Unlocks

- **Circular buffers** — advance the read head by rotating the logical start position.
- **Round-robin scheduling** — rotate the task queue after each cycle.
- **Shift registers** — simulate hardware shift registers with arbitrary step sizes.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Rotation | Split + `@` (list concat) | Slice concat: `extend_from_slice` |
| Negative rotation | Separate case or `mod` trick | Same formula: `((n % len) + len) % len` |
| Cyclic iteration | `List.init` with modular index | `.cycle().skip(n).take(len)` |
| Signed index | `int` | `i64` (explicit sign), then cast to `usize` |
| Empty list guard | Pattern match on `[]` | `.is_empty()` early return |
