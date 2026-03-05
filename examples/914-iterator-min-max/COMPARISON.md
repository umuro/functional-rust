# OCaml vs Rust: Iterator min() and max()

## Side-by-Side Code

### OCaml

```ocaml
let list_min = function
  | [] -> None
  | lst -> Some (List.fold_left min max_int lst)

let list_max = function
  | [] -> None
  | lst -> Some (List.fold_left max min_int lst)

(* min by derived property *)
let shortest words =
  match words with
  | [] -> None
  | hd :: tl ->
    Some (List.fold_left (fun acc w ->
      if String.length w < String.length acc then w else acc
    ) hd tl)
```

### Rust (idiomatic)

```rust
// Built into Iterator — no manual fold needed
fn slice_min(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().min()
}

fn slice_max(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().max()
}

// min/max by derived property — no Ord required on the struct
fn shortest<'a>(words: &[&'a str]) -> Option<&'a str> {
    words.iter().copied().min_by_key(|w| w.len())
}
```

### Rust (functional / explicit fold)

```rust
fn slice_min_fold(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().reduce(|acc, x| if x < acc { x } else { acc })
}

fn slice_max_fold(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().reduce(|acc, x| if x > acc { x } else { acc })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Min of list | `val list_min : int list -> int option` | `fn slice_min(nums: &[i32]) -> Option<i32>` |
| Max by key | `List.fold_left` with manual comparator | `Iterator::max_by_key(fn)` |
| Float extreme | `List.fold_left Float.min` | `.reduce(f64::min)` |
| Empty case | pattern match on `[]` | Returns `None` automatically |

## Key Insights

1. **Built-in vs manual fold:** OCaml's `List` module lacks `min`/`max` functions, so idiomatic OCaml uses `List.fold_left min max_int lst`. Rust's `Iterator` trait provides `.min()` and `.max()` directly, making the intent explicit.

2. **`Ord` requirement and the float problem:** Rust's `.min()`/`.max()` require `T: Ord` — a *total* ordering. `f64` does not implement `Ord` because `NaN` breaks total order. The workaround is `.reduce(f64::min)` which uses `f64`'s built-in partial comparison. OCaml avoids this distinction by having a polymorphic `min` that uses structural comparison everywhere.

3. **`min_by_key` eliminates boilerplate:** OCaml requires a hand-written fold to find the minimum of a derived property. Rust's `.min_by_key(|x| x.score)` expresses this directly, without needing to implement `Ord` on the struct or build intermediate tuples.

4. **Ownership and `copied()`:** Iterating over `&[i32]` yields `&i32` references. `.copied()` converts them to owned `i32` values before calling `.min()`, keeping the return type `Option<i32>` rather than `Option<&i32>`. For non-`Copy` types, use `.cloned()` or work with references directly.

5. **Empty case handling:** Both languages wrap the result in `Option`/`option`, but OCaml requires an explicit pattern match on the empty list before the fold, while Rust's iterator methods handle the empty case internally and return `None` automatically.

## When to Use Each Style

**Use `.min()` / `.max()`** when your type implements `Ord` and you want the simplest possible expression.
**Use `.min_by_key()` / `.max_by_key()`** when comparing structs by a derived numeric or comparable field.
**Use `.reduce(f64::min)`** when working with floats, which lack a total ordering and cannot use `.min()` directly.
**Use explicit `fold` / `reduce`** when you need custom tie-breaking logic or are computing several aggregates in one pass.
