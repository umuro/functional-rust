# OCaml vs Rust: Set.Make — Immutable Sorted Set

## Side-by-Side Code

### OCaml

```ocaml
module IntSet = Set.Make(Int)

let s1 = IntSet.of_list [1; 3; 5; 7; 9]
let s2 = IntSet.of_list [2; 3; 5; 7; 11]

let union = IntSet.union s1 s2
let inter = IntSet.inter s1 s2
let diff  = IntSet.diff  s1 s2

let print_set s =
  IntSet.elements s
  |> List.map string_of_int
  |> String.concat ", "
  |> Printf.printf "{%s}\n"

let () = print_set union; print_set inter; print_set diff
```

### Rust (idiomatic)

```rust
use std::collections::BTreeSet;

fn set_of_slice<T: Ord + Clone>(items: &[T]) -> BTreeSet<T> {
    items.iter().cloned().collect()
}

fn union<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.union(b).cloned().collect()
}

fn inter<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.intersection(b).cloned().collect()
}

fn diff<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.difference(b).cloned().collect()
}

fn elements<T: Ord + Clone>(s: &BTreeSet<T>) -> Vec<T> {
    s.iter().cloned().collect()
}
```

### Rust (functional / persistent-style add and remove)

```rust
// "Immutable" add: clone the set, insert, return new binding
fn add<T: Ord + Clone>(x: T, s: &BTreeSet<T>) -> BTreeSet<T> {
    let mut next = s.clone(); // cost of simulating persistence
    next.insert(x);
    next
}

// "Immutable" remove: same pattern
fn remove<T: Ord + Clone>(x: &T, s: &BTreeSet<T>) -> BTreeSet<T> {
    let mut next = s.clone();
    next.remove(x);
    next
}

// Filter, map, fold via iterators
fn filter<T: Ord + Clone, F: Fn(&T) -> bool>(pred: F, s: &BTreeSet<T>) -> BTreeSet<T> {
    s.iter().filter(|x| pred(x)).cloned().collect()
}

fn map_set<T: Ord + Clone, U: Ord, F: Fn(&T) -> U>(f: F, s: &BTreeSet<T>) -> BTreeSet<U> {
    s.iter().map(f).collect()
}

fn fold_set<T: Ord, A, F: Fn(A, &T) -> A>(f: F, s: &BTreeSet<T>, init: A) -> A {
    s.iter().fold(init, f)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Set type | `IntSet.t` (module-specific) | `BTreeSet<i32>` |
| Generic set type | `'a Set.t` (via functor) | `BTreeSet<T> where T: Ord` |
| Build from list | `Set.of_list : 'a list -> 'a Set.t` | `set_of_slice<T: Ord+Clone>(&[T]) -> BTreeSet<T>` |
| Union | `Set.union : t -> t -> t` | `fn union<T: Ord+Clone>(&BTreeSet<T>, &BTreeSet<T>) -> BTreeSet<T>` |
| Membership | `Set.mem : elt -> t -> bool` | `BTreeSet::contains(&T) -> bool` |
| Sorted elements | `Set.elements : t -> elt list` | `BTreeSet::iter()` (already sorted) |
| Fold | `Set.fold : (elt -> 'a -> 'a) -> t -> 'a -> 'a` | `Iterator::fold(init, f)` |

## Key Insights

1. **`BTreeSet` is OCaml's `Set.Make` in Rust.** Both are ordered balanced trees
   with O(log n) lookup, insert, and delete; both guarantee sorted iteration;
   both deduplicate on insert.

2. **Persistence costs a clone.** OCaml's AVL sets share structure between
   versions (path copying), so `Set.add` is O(log n) in space. Rust's
   `BTreeSet` is fully mutable — simulating the immutable API requires `clone()`
   which is O(n). For large sets, prefer Rust's in-place mutation style instead.

3. **Functor vs trait bounds.** `Set.Make(Ord)` is a compile-time functor
   application that produces a named module (`IntSet`). Rust achieves the same
   static dispatch with `T: Ord` generics — no separate module is needed.

4. **`union`/`intersection`/`difference` return iterators, not sets.** The Rust
   stdlib methods take `&BTreeSet<T>` and yield `&T` references; you must
   `.cloned().collect()` to materialise a new owned set. This lazy design avoids
   unnecessary allocation if you only need to iterate the result.

5. **`Set.fold` is argument-order reversed in OCaml.** OCaml's signature is
   `fold : (elt -> 'a -> 'a) -> t -> 'a -> 'a` (element first, accumulator
   second). Rust's `Iterator::fold(init, |acc, x| ...)` has accumulator first.
   The functional meaning is identical; only the lambda argument order differs.

## When to Use Each Style

**Use idiomatic Rust (`BTreeSet` in-place mutation) when:** you own the set and
performance matters — avoid the clone overhead of persistent-style wrappers.

**Use persistent-style wrappers (`add`/`remove` returning new sets) when:**
translating OCaml algorithms directly, or when the call site needs to keep both
the old and new set alive simultaneously (e.g., undo/redo, version tracking).
