📖 **[View on hightechmind.io →](https://hightechmind.io/rust/267-iterator-cycle)**

---

# 267: Infinite Cycling with cycle()

**Difficulty:** 2  **Level:** Intermediate

Repeat a finite iterator endlessly — round-robin scheduling, color cycling, repeating patterns.

## The Problem This Solves

You need to pair each item in a longer list with elements from a shorter repeating pattern: assign roles in a round-robin, alternate row colors, apply repeating weights for a checksum. Without `cycle()`, you'd use modular indexing — `pattern[i % pattern.len()]` — which requires the pattern to be indexable (arrays, vecs) and scatters the logic through your code.

Cycling is especially useful when `zip`-ping: pair each element of a long list with the corresponding element of a short list, wrapping around automatically. `cycle()` turns that into a single expression.

OCaml doesn't have a built-in cycle for lists, but `Seq` supports infinite sequences. In Rust, `cycle()` works on any iterator that implements `Clone` — the iterator is cloned at the start of each repetition.

## The Intuition

`cycle()` returns an iterator that, once it reaches the end of the underlying sequence, resets to the beginning and starts again — forever. Use `take(n)` or `zip()` to bound consumption.

```rust
let colors = ["red", "green", "blue"];
let cycled: Vec<_> = colors.iter().cycle().take(7).collect();
// → ["red", "green", "blue", "red", "green", "blue", "red"]
```

## How It Works in Rust

```rust
let colors = ["red", "green", "blue"];

// Take a bounded prefix of the infinite cycle
let cycled: Vec<_> = colors.iter().cycle().take(9).collect();
// → [red, green, blue, red, green, blue, red, green, blue]

// Round-robin role assignment: zip a long list with a cycling short list
let items = ["a", "b", "c", "d", "e"];
let roles = ["leader", "follower"];
let assigned: Vec<String> = items.iter()
    .zip(roles.iter().cycle())               // cycle roles to match items length
    .map(|(item, role)| format!("{}->{}", item, role))
    .collect();
// → ["a->leader", "b->follower", "c->leader", "d->follower", "e->leader"]

// Alternating boolean pattern
let alternating: Vec<bool> = [true, false].iter().copied()
    .cycle().take(8).collect();
// → [true, false, true, false, true, false, true, false]

// Repeating weights for a checksum
let weights = [1u32, 3, 7];
let data = [4u32, 5, 6, 7, 8, 9];
let checksum: u32 = data.iter()
    .zip(weights.iter().cycle())
    .map(|(d, w)| d * w)
    .sum();
```

The inner iterator must implement `Clone` — `cycle()` clones the iterator to replay it from the start each cycle.

## What This Unlocks

- **Round-robin scheduling** — assign tasks, colors, or labels from a short repeating list to a longer sequence.
- **Repeating test data** — generate patterns for property tests without manual modulo arithmetic.
- **Weighted cycling** — pair data with cyclically repeating weights for checksums or transforms.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Infinite repetition | `Seq.cycle` (OCaml 4.14+) / manual unfold | `iter.cycle()` |
| Bound consumption | `Seq.take n` | `.take(n)` or `.zip(finite_iter)` |
| Requires `Clone` | N/A | Yes — iterator cloned at each cycle boundary |
| Works on any iterator | `Seq` only | Any `Iterator + Clone` |
