# Graph with Arena Allocation — Comparison

## Core Insight
Graphs have shared, cyclic references — the borrow checker's nightmare. The arena pattern sidesteps this entirely: store all nodes in a `Vec`, use `usize` indices instead of pointers. Both languages can use this pattern, but it's especially important in Rust where `Rc<RefCell>` graphs are verbose and slow.

## OCaml Approach
- Array-based arena: `node array` with index-based edges
- Mutable arrays for building incrementally
- GC means pointer-based graphs also work fine
- Queue module for BFS

## Rust Approach
- `Vec<Node>` arena with `usize` indices as node handles
- `add_node` returns index, `add_edge` connects indices
- No lifetime issues — indices are just numbers
- Cache-friendly: nodes stored contiguously
- BFS/DFS use `vec![false; n]` for visited tracking

## Comparison Table

| Feature | OCaml (array arena) | Rust (`Vec` arena) |
|---|---|---|
| Node storage | `node array` | `Vec<Node>` |
| Edge references | `int` indices | `usize` indices |
| Add node | Array mutation | `push` returns index |
| Cycles | Free (GC or indices) | Free (indices only) |
| Cache locality | Array = contiguous | Vec = contiguous |
| Alternative | Pointer graph (GC safe) | `Rc<RefCell>` (verbose) |
| Recommendation | Either works | Arena strongly preferred |
