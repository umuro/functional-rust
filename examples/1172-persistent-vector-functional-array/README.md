# Example 1172: Persistent Vector (Functional Array)

**Difficulty:** ⭐⭐⭐
**Category:** Functional Data Structures | Trees | Persistent Structures
**OCaml Source:** Classic functional programming — persistent balanced binary tree

## Problem Statement

Implement a persistent (immutable) vector using a balanced binary tree where `get` retrieves an element by index and `set` returns a new version of the vector with one element replaced, leaving the original completely unchanged.

## Learning Outcomes

- `Rc<T>` enables structural sharing — shared subtrees avoid deep copies on update
- Persistent data structures: multiple versions coexist after each `set` operation
- Pattern matching on recursive enum types mirrors OCaml ADTs directly
- Using `Option` instead of panicking (`failwith`) for safe out-of-bounds handling

## OCaml Approach

OCaml's garbage collector and immutable-by-default values make persistent structures natural: `set` constructs new `Two` nodes along the path to the modified leaf, while unchanged subtrees are automatically shared (the GC tracks all references). The `failwith` calls handle errors by raising exceptions.

## Rust Approach

Rust uses `Rc<T>` (reference-counted pointer) to achieve the same structural sharing without a GC. `Rc::clone` increments the reference count in O(1) — no deep copy occurs. Out-of-bounds returns `Option<T>` rather than panicking. The `set` method takes `&self` (shared borrow) and returns an owned `Option<Self>`, reflecting the persistent semantics.

## Key Differences

1. **Memory management:** OCaml uses a tracing GC for shared subtrees; Rust uses `Rc<T>` for explicit reference counting with deterministic drop.
2. **Error handling:** OCaml raises exceptions (`failwith`); Rust returns `Option<T>` for safe, composable error propagation.
3. **Ownership:** OCaml values are implicitly shared; in Rust, `Rc::clone(r)` makes the sharing explicit — you can see exactly what is shared.
4. **Argument order:** OCaml convention is `get i v` (index before collection); Rust idiom uses method syntax `v.get(i)`, though free-function style mirrors OCaml.
