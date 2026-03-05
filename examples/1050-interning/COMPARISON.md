# String Interning — Comparison

## Core Insight
String interning maps strings to unique integer IDs, enabling O(1) comparison and smaller memory footprint when strings repeat. Both languages implement it the same way — a bidirectional map between strings and IDs. The key difference is that Rust's `Symbol` can be `Copy`, making it as cheap as an integer.

## OCaml Approach
- `Hashtbl` for both directions: `(string, int)` and `(int, string)`
- Mutable counter for next ID
- Module-based encapsulation
- OCaml strings are already value-compared (structural equality)
- GC handles interned string lifetime

## Rust Approach
- `HashMap<String, Symbol>` for string→ID
- `Vec<String>` for ID→string (index = ID)
- `Symbol(usize)` newtype: derives `Copy, Clone, Eq, Hash`
- Symbols as HashMap keys are faster than String keys
- Must manage lifetimes — interner must outlive symbols

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| String→ID | `Hashtbl` | `HashMap<String, Symbol>` |
| ID→String | `Hashtbl` | `Vec<String>` (index) |
| ID type | `int` | `Symbol(usize)` newtype |
| Copy semantics | N/A (GC) | `Copy` trait — zero-cost |
| Comparison cost | O(1) int compare | O(1) usize compare |
| Use as key | int in any map | Symbol derives `Hash + Eq` |
| Memory | GC managed | Must outlive usage |
| Common in | Compilers, interpreters | Compilers, ECS, databases |
