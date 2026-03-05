# Persistent HashMap — Comparison

## Core Insight
Persistent data structures allow "time travel" — old versions remain valid after updates. OCaml's `Map` achieves this natively through structural sharing (balanced trees share unchanged subtrees). Rust must clone or use specialized crates.

## OCaml Approach
- `Map` IS persistent — `add` returns new map, old map unchanged
- Structural sharing: O(log n) nodes copied per update, rest shared
- Version history: just keep references to old maps
- Undo/redo: stack of map values (cheap — they share structure)
- GC handles cleanup of unreachable versions

## Rust Approach
- `HashMap` is mutable — no built-in persistence
- `clone()` simulates persistence but is O(n) per update
- Version history via `Vec<Rc<Map>>` for cheap reference sharing
- Undo/redo: `std::mem::replace` for efficient state swapping
- Real persistence: `im` crate provides HAMT (Hash Array Mapped Trie)

## Comparison Table

| Feature | OCaml (`Map`) | Rust (clone-based) |
|---|---|---|
| Persistence | Native | Simulated via clone |
| Update cost | O(log n) shared | O(n) full clone |
| Structural sharing | Yes (tree) | No (full copy) |
| Version access | Free | Stored explicitly |
| Undo cost | O(1) (keep ref) | O(1) (swap) but O(n) creation |
| Real persistent | stdlib | `im` crate (HAMT) |
| Memory per version | O(log n) delta | O(n) full copy |
