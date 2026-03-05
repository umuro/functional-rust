# Small Vector Optimization — Comparison

## Core Insight
The small-vec optimization stores elements inline (on the stack) up to a fixed capacity N, then spills to the heap. This matters in Rust/C++ where heap allocation has measurable cost. In OCaml, the GC's minor heap makes small allocations nearly free, so this optimization is unnecessary.

## OCaml Approach
- Simulated with variant: `Inline of array * int | Heap of list`
- Not idiomatic — OCaml's GC handles this transparently
- Minor heap allocation is a pointer bump (~1ns)
- Short-lived objects collected in microseconds
- Programmers don't think about stack vs heap

## Rust Approach
- Enum with const generic: `SmallVec<T, const N: usize>`
- `Inline { data: [Option<T>; N], len }` for stack storage
- `Heap(Vec<T>)` after spill
- Real-world: use `smallvec` or `tinyvec` crate
- Measurable win for hot paths with small, temporary collections
- Const generics make capacity a compile-time parameter

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Heap alloc cost | ~1ns (GC bump) | ~20-50ns (system allocator) |
| Optimization needed | No | Yes, for hot paths |
| Stack storage | N/A (GC managed) | `[Option<T>; N]` or MaybeUninit |
| Spill point | N/A | Configurable via const generic |
| Real-world impl | Not used | `smallvec`, `tinyvec` crates |
| Key benefit | None (GC is fast) | Avoid allocator + better locality |
