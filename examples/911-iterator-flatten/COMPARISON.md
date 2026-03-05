# OCaml vs Rust: One-Level Flattening with flatten()

## Side-by-Side Code

### OCaml
```ocaml
(* Flatten list of lists — one level *)
let flat = List.concat [[1; 2]; [3; 4]; [5; 6]]
(* → [1; 2; 3; 4; 5; 6] *)

(* Filter out None, keep Some values *)
let values = List.filter_map Fun.id [Some 1; None; Some 3; None; Some 5]
(* → [1; 3; 5] *)

(* Characters from words via concat_map *)
let chars = List.concat_map
  (fun w -> List.init (String.length w) (fun i -> w.[i]))
  ["hello"; "world"]
```

### Rust (idiomatic — iterator flatten)
```rust
// Flatten Vec<Vec<T>> — one level removed
let flat: Vec<i32> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]
    .into_iter()
    .flatten()
    .collect();
// → [1, 2, 3, 4, 5, 6]

// Flatten Vec<Option<T>> — Nones discarded
let values: Vec<i32> = vec![Some(1), None, Some(3), None, Some(5)]
    .into_iter()
    .flatten()
    .collect();
// → [1, 3, 5]
```

### Rust (Option flattening — monadic join)
```rust
// flatten() on Option<Option<T>> — not just iterators
let a: Option<Option<i32>> = Some(Some(42));
assert_eq!(a.flatten(), Some(42));

let b: Option<Option<i32>> = Some(None);
assert_eq!(b.flatten(), None);

let c: Option<Option<i32>> = None;
assert_eq!(c.flatten(), None);
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Flatten list of lists | `List.concat : 'a list list -> 'a list` | `Iterator::flatten` → `Flatten<I>` |
| Filter-keep Somes | `List.filter_map Fun.id` | `.into_iter().flatten()` on `Vec<Option<T>>` |
| Collapse nested option | `Option.join : 'a option option -> 'a option` (≥ 4.08) | `Option::flatten` |
| Chars from strings | `List.concat_map ... String.get` | `.flat_map(\|w\| w.chars())` |

## Key Insights

1. **`flatten` = monadic `join`**: In both languages, flattening one level of nesting is the categorical `join` operation (`μ` in monad theory). OCaml's `List.concat` and Rust's `.flatten()` are the same idea — remove exactly one constructor layer.

2. **`Option` is iterable in Rust**: `Option<T>` implements `IntoIterator` (yielding 0 or 1 items), so `.flatten()` on an iterator of `Option<T>` automatically discards `None` and unwraps `Some`. OCaml needs the dedicated `List.filter_map Fun.id` idiom instead.

3. **One level only — intentionally**: Unlike `List.flatten` applied recursively, `.flatten()` in Rust removes precisely one level. A `Vec<Vec<Vec<T>>>` becomes `Vec<Vec<T>>`, not `Vec<T>`. This mirrors OCaml's `List.concat` which also flattens only one level.

4. **`flat_map(|x| x)` vs `flatten()`**: In Rust, `iter.flat_map(|x| x)` and `iter.flatten()` are equivalent; `flatten()` is the preferred spelling because it removes the tautological closure and signals intent directly.

5. **Zero allocation overhead**: Both OCaml and Rust implementations process items lazily through the inner iterables without allocating an intermediate container — the outer list/iterator is consumed element by element.

## When to Use Each Style

**Use `.flatten()` when:** you already have an iterator of iterables (e.g., after `map()` returns a `Vec` or `Option`) and want to concatenate them without an intermediate allocation or a redundant `|x| x` closure.

**Use `.flat_map(f)` when:** you need to transform *and* flatten in one step — it is more composable and avoids an intermediate iterator adapter layer.
