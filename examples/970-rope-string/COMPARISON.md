# Rope String — Comparison

## Core Insight
A rope represents a string as a binary tree of smaller strings (leaves). Concatenation creates a new Node in O(1). Indexing and splitting traverse the tree in O(log n). Both languages express this as a recursive algebraic type. The critical Rust difference: `Box<Rope>` is required because recursive enum variants must have statically known size, and a direct self-reference would be infinite.

## OCaml Approach
- `type rope = Leaf of string | Node of rope * rope * int` — recursive type, implicit
- No heap boxing needed — OCaml's GC handles pointer-sized references automatically
- `String.sub s 0 i` for splitting leaf strings
- `s.[i]` for character access (byte index)
- `to_string l ^ to_string r` — string concatenation (O(n) but readable)

## Rust Approach
- `enum Rope { Leaf(String), Node(Box<Rope>, Box<Rope>, usize) }` — explicit Box
- `Box::new(...)` allocates on the heap, providing the indirection needed for recursion
- `s[..i].to_string()` for splitting (byte-indexed slice)
- `s.chars().nth(i)` for Unicode-safe character access
- `#[derive(Clone)]` needed because `to_string_val` requires cloning for sub
- `l.to_string_val()` + `push_str` instead of `^` operator

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursive type | `type rope = ... Node of rope * rope * int` | `enum Rope { Node(Box<Rope>, Box<Rope>, usize) }` |
| Heap boxing | Implicit (GC) | Explicit `Box<T>` |
| String split | `String.sub s 0 i` | `s[..i].to_string()` |
| Char access | `s.[i]` (byte) | `s.chars().nth(i)` (Unicode) |
| Concat strings | `^` operator | `String::push_str` |
| Clone | Not needed (GC shares) | `#[derive(Clone)]` explicit |
| Length field | `int` | `usize` |
