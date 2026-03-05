# JSON Query by Path — Comparison

## Core Insight
Recursive path traversal is the same algorithm in both languages. The critical difference: OCaml's GC makes returning values trivial (`Some v`), while Rust must track where the returned data lives using lifetime annotations (`Option<&'a JsonValue>`). The borrow is more efficient (no copy) but requires explicit lifetime reasoning.

## OCaml Approach
- `List.assoc_opt key pairs` finds a key in association list, returning `Option`
- `List.nth items i` indexes into a list (O(n) — fine for small arrays)
- `int_of_string_opt` safely parses array indices
- Recursive `match path, json with` cleanly handles all combinations
- Returns `Some j` — a GC-managed copy (or shared immutable value)

## Rust Approach
- `pairs.iter().find(|(k, _)| k == key)` searches Vec of pairs
- `items.get(idx)` bounds-checked index returning `Option<&T>`
- `key.parse::<usize>().ok()` for index parsing
- Slice pattern `[key, rest @ ..]` for head/tail deconstruction
- Returns `Option<&'a JsonValue>` — a borrowed reference, zero-copy
- `'a` lifetime links output reference to input reference

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Return type | `json option` | `Option<&'a JsonValue>` |
| Memory model | GC, shared immutable | Borrow, zero-copy, explicit lifetime |
| Assoc list lookup | `List.assoc_opt key pairs` | `pairs.iter().find(\|(k,_)\| k==key)` |
| Array index | `List.nth items i` | `items.get(idx)` |
| Index parsing | `int_of_string_opt` | `key.parse::<usize>().ok()` |
| Path deconstruction | `key :: rest` | `[key, rest @ ..]` slice pattern |
| Chaining options | `match ... with \| Some v -> get rest v` | `.and_then(\|v\| get(rest, v))` |
