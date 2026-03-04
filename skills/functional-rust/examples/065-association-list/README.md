# Example 065: Association List — Functional Key-Value Store

**Difficulty:** ⭐
**Category:** Data Structures
**Concept:** The simplest functional dictionary: a list of key-value pairs where insert prepends and lookup scans linearly. Shadowing (inserting a duplicate key) naturally works because lookup finds the first match.
**OCaml → Rust insight:** OCaml's list cons `(k,v) :: lst` is O(1) and shares the tail; Rust's `Vec` requires moving all elements, making the ownership cost of "prepend" explicit.
