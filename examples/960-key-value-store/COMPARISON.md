# Key-Value Store — Comparison

## Core Insight
Both `Hashtbl` (OCaml) and `HashMap` (Rust) are hash-based mutable dictionaries. The API is nearly identical. The key difference: Rust wraps state in a `struct` with `&mut self` methods (enforcing ownership), while OCaml passes the table as a function argument. Both languages also support functional association lists for immutable KV storage.

## OCaml Approach
- `Hashtbl.create 16` creates a mutable table with initial capacity
- `Hashtbl.replace` — set (handles both insert and update)
- `Hashtbl.find_opt` — get returning `option`
- `Hashtbl.remove` — delete key
- `Hashtbl.mem` — contains check
- `Hashtbl.fold` — iterate over all entries
- Functional alternative: `(string * 'a) list` with `List.assoc_opt`

## Rust Approach
- `HashMap::new()` wrapped in a struct for encapsulation
- `insert` (returns old value), `get`, `remove`, `contains_key`
- `&self` for reads, `&mut self` for writes — ownership enforced by compiler
- `impl Into<String>` for flexible key/value input
- Functional alternative: `Vec<(String, V)>` with filter + find

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Hash table | `Hashtbl.t` | `HashMap<K, V>` |
| Create | `Hashtbl.create 16` | `HashMap::new()` |
| Insert/update | `Hashtbl.replace t k v` | `map.insert(k, v)` |
| Lookup | `Hashtbl.find_opt t k` | `map.get(k)` → `Option<&V>` |
| Delete | `Hashtbl.remove t k` | `map.remove(k)` |
| Contains | `Hashtbl.mem t k` | `map.contains_key(k)` |
| Iteration | `Hashtbl.fold` / `Hashtbl.iter` | `.iter()` / `.keys()` / `.values()` |
| Encapsulation | Module functions + table arg | `struct` with `impl` block |
| Mutation | Passed by value (mutable) | `&mut self` methods |
