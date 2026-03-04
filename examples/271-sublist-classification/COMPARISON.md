# OCaml vs Rust: Sublist Classification

## Side-by-Side Code

### OCaml
```ocaml
type relation = Equal | Sublist | Superlist | Unequal

let rec starts_with lst prefix = match lst, prefix with
  | _, [] -> true
  | [], _ -> false
  | h1 :: t1, h2 :: t2 -> h1 = h2 && starts_with t1 t2

let rec is_sublist sub lst = match lst with
  | [] -> sub = []
  | _ :: t -> starts_with lst sub || is_sublist sub t

let classify a b =
  if a = b then Equal
  else if is_sublist a b then Sublist
  else if is_sublist b a then Superlist
  else Unequal
```

### Rust (idiomatic)
```rust
pub fn classify_idiomatic<T: PartialEq>(a: &[T], b: &[T]) -> Relation {
    if a == b {
        Relation::Equal
    } else if is_sublist_idiomatic(a, b) {
        Relation::Sublist
    } else if is_sublist_idiomatic(b, a) {
        Relation::Superlist
    } else {
        Relation::Unequal
    }
}

fn is_sublist_idiomatic<T: PartialEq>(sub: &[T], lst: &[T]) -> bool {
    if sub.is_empty() { return true; }
    lst.windows(sub.len()).any(|w| w == sub)
}
```

### Rust (functional/recursive)
```rust
fn starts_with<T: PartialEq>(lst: &[T], prefix: &[T]) -> bool {
    match (lst, prefix) {
        (_, [])                      => true,
        ([], _)                      => false,
        ([h1, t1 @ ..], [h2, t2 @ ..]) => h1 == h2 && starts_with(t1, t2),
    }
}

fn is_sublist_recursive<T: PartialEq>(sub: &[T], lst: &[T]) -> bool {
    match lst {
        []             => sub.is_empty(),
        [_, rest @ ..] => starts_with(lst, sub) || is_sublist_recursive(sub, rest),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Relation type | `type relation = Equal \| Sublist \| Superlist \| Unequal` | `enum Relation { Equal, Sublist, Superlist, Unequal }` |
| Classify signature | `val classify : 'a list -> 'a list -> relation` | `fn classify<T: PartialEq>(a: &[T], b: &[T]) -> Relation` |
| List/slice type | `'a list` | `&[T]` (borrowed slice) |
| Equality constraint | implicit (structural `=`) | explicit `T: PartialEq` bound |

## Key Insights

1. **`slice::windows`** replaces the recursive prefix-checking loop entirely — it generates all overlapping sub-slices of a given length, enabling a clean `.any()` check.
2. **Slice patterns** (`[h, rest @ ..]`) in Rust match OCaml's `h :: t` perfectly, so the recursive solution is nearly a line-for-line translation.
3. **Generic bounds**: OCaml's structural equality is built-in; Rust requires an explicit `T: PartialEq` trait bound so the compiler knows `==` is valid for the element type.
4. **Borrowing**: both functions take `&[T]` (borrowed slices), avoiding unnecessary allocation — Rust guarantees no copies are made of the list data.
5. **Performance**: the idiomatic `.windows()` approach is O(n·m) but expressed in a single iterator chain with no heap allocation, while the recursive version has the same complexity with stack frames.

## When to Use Each Style

**Use idiomatic Rust when:** you want concise, expressive code that leverages `std` slice methods — `windows` communicates the contiguous-subsequence intent directly.  
**Use recursive Rust when:** you are translating OCaml algorithms for educational purposes or when the recursive structure reflects the problem's natural induction principle more clearly.
