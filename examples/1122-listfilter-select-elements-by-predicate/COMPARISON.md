# OCaml vs Rust: List Filter — Select Elements by Predicate

## Side-by-Side Code

### OCaml

```ocaml
(* Idiomatic: List.filter is built-in *)
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers

(* Recursive: explicit structural recursion *)
let rec filter_rec pred = function
  | [] -> []
  | x :: rest ->
    if pred x then x :: filter_rec pred rest
    else filter_rec pred rest
```

### Rust (idiomatic)

```rust
pub fn filter_idiomatic<T, F>(list: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).cloned().collect()
}

// Two halves in a single pass — more efficient than filtering twice
pub fn partition_by<T, F>(list: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let (yes, no): (Vec<&T>, Vec<&T>) = list.iter().partition(|x| predicate(x));
    (
        yes.into_iter().cloned().collect(),
        no.into_iter().cloned().collect(),
    )
}
```

### Rust (functional/recursive)

```rust
pub fn filter_recursive<T, F>(list: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    fn go<T, F>(list: &[T], pred: &F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> bool,
    {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, pred);
                if pred(head) {
                    rest.insert(0, head.clone()); // prepend: mirrors `x :: filter_rec pred rest`
                }
                rest
            }
        }
    }
    go(list, &predicate)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Filter function | `val filter : ('a -> bool) -> 'a list -> 'a list` | `fn filter_idiomatic<T, F>(list: &[T], predicate: F) -> Vec<T>` |
| Predicate type | `'a -> bool` | `F: Fn(&T) -> bool` |
| List type (input) | `'a list` | `&[T]` (borrowed slice) |
| List type (output) | `'a list` | `Vec<T>` (owned) |
| Partition | `val partition : ('a -> bool) -> 'a list -> 'a list * 'a list` | `fn partition_by<T, F>(list: &[T], predicate: F) -> (Vec<T>, Vec<T>)` |

## Key Insights

1. **`List.filter` → `.filter().collect()`:** The iterator chain is a direct translation — the predicate is passed as a closure and items satisfying it are collected into a new `Vec`.
2. **Borrow vs own:** OCaml predicates receive values; Rust predicates here receive `&T` (references) to avoid cloning the input elements before the final `.cloned().collect()` at the output.
3. **Partition eliminates double traversal:** The OCaml example calls `List.filter` twice (once for evens, once for odds), making two passes over the list. Rust's `partition` does both in a single pass — a meaningful optimization for large lists.
4. **Slice pattern matching:** `[head, tail @ ..]` in Rust is a close match to OCaml's `x :: rest` — both destructure the head from the tail, enabling the same recursive structure.
5. **GC vs Clone:** OCaml's GC allows sharing list nodes freely. In Rust, moving from borrowed `&T` to an owned `Vec<T>` requires `.clone()` — the cost is explicit and visible.

## When to Use Each Style

**Use idiomatic Rust (`.filter().collect()`) when:** you need a subset of a slice and want clear, concise code that mirrors `List.filter`.

**Use `partition_by` when:** you need both the matching and non-matching elements — it avoids a second traversal and is the Rust equivalent to OCaml's `List.partition`.

**Use recursive Rust when:** you are learning the OCaml→Rust translation or need to implement a custom traversal that processes elements in pairs or accumulates state beyond what `filter` supports.
