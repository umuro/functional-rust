# Circular Buffer — Comparison

## Core Insight
A ring buffer uses modular arithmetic (`i = (i + 1) % capacity`) to wrap around array indices, creating the illusion of a circular structure from a linear array. Three indices (`head`, `tail`, `count`) fully describe state. Both OCaml and Rust implement this identically; the main difference is OCaml uses `mutable` record fields vs Rust's `let mut` struct fields.

## OCaml Approach
- `mutable head/tail/count` in a record
- `Array.make capacity default` — requires a default value
- `r.head <- (r.head + 1) mod r.capacity` — modular advance
- Handles full/empty via `count` field
- Overwrite policy: advance both tail and head when full

## Rust Approach
- Struct with `head: usize`, `tail: usize`, `count: usize`
- `vec![T::default(); capacity]` — requires `T: Default + Clone`
- `self.tail = (self.tail + 1) % self.capacity` — same modular advance
- Check `is_full()` before incrementing count (slightly different branch order)
- `to_vec()` for snapshot: `(0..count).map(|i| data[(head+i)%cap])` 

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| State | Mutable record fields | `&mut self` struct |
| Array init | `Array.make cap default` | `vec![T::default(); cap]` |
| Modular advance | `(i + 1) mod capacity` | `(i + 1) % capacity` |
| Full check | `count = capacity` | `count == capacity` |
| Overwrite | Advance both head and tail | Advance head after tail write |
| Snapshot | Manual fold | `.map(|i| data[(head+i)%cap])` |
| Generic | `'a ring` | `RingBuffer<T>` with `T: Clone` |
