# OCaml vs Rust: List Filter — Select Elements by Predicate

## Side-by-Side Code

### OCaml

```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers
```

### Rust (idiomatic — borrowed refs)

```rust
pub fn filter<T, F>(list: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).collect()
}
```

### Rust (idiomatic — owned, mirrors OCaml output)

```rust
pub fn filter_cloned<T: Clone, F>(list: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).cloned().collect()
}
```

### Rust (functional/recursive)

```rust
pub fn filter_recursive<T: Clone, F>(list: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    fn go<T: Clone>(list: &[T], predicate: &dyn Fn(&T) -> bool) -> Vec<T> {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, predicate);
                if predicate(head) {
                    let mut result = vec![head.clone()];
                    result.append(&mut rest);
                    result
                } else {
                    rest
                }
            }
        }
    }
    go(list, &predicate)
}
```

### OCaml (recursive)

```ocaml
let rec filter_rec pred = function
  | [] -> []
  | x :: rest ->
    if pred x then x :: filter_rec pred rest
    else filter_rec pred rest
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Filter function | `val filter : ('a -> bool) -> 'a list -> 'a list` | `fn filter<T, F>(list: &[T], predicate: F) -> Vec<&T>` |
| List type | `'a list` | `&[T]` (slice) or `Vec<T>` |
| Predicate type | `'a -> bool` | `Fn(&T) -> bool` |
| Owned output | implicit (all values) | requires `T: Clone`, use `.cloned()` |

## Key Insights

1. **`iter().filter().collect()` is the idiomatic translation of `List.filter`** — the combinator chain mirrors OCaml's functional style directly without a visible loop.

2. **Borrowed vs owned output is a Rust-specific decision** — OCaml's `List.filter` always returns a new list of the same type. In Rust you choose: `Vec<&T>` avoids allocation but ties lifetimes to the input slice; `Vec<T>` requires `Clone` but is independent. Neither is "wrong" — the right choice depends on how the caller uses the result.

3. **Recursive Rust requires a `&dyn Fn` helper to avoid infinite monomorphization** — if you write `filter_recursive(tail, &predicate)` where `predicate: F` is generic, each recursive call instantiates `F = &F = &&F = ...` and the compiler hits its recursion limit. The fix is a non-generic inner function `fn go(..., predicate: &dyn Fn(...))` that uses dynamic dispatch for the predicate.

4. **Slice patterns (`[head, tail @ ..]`) mirror OCaml cons patterns** — OCaml's `x :: rest` destructures a cons cell; Rust's `[head, tail @ ..]` destructures a slice. Both express "take the head, recurse on the tail" without indexing.

5. **Performance: iterator chains are lazy, recursive is eager** — Rust's `filter().collect()` builds the `Vec` in a single pass through the iterator. The recursive version builds it in reverse (appending to the front via `vec![head]; result.append(rest)`) which is O(n²) due to `Vec` front-insertion. For production code, always prefer the iterator version.

## When to Use Each Style

**Use idiomatic iterator Rust when:** filtering any slice or collection in production code — it's O(n), composable, and the most readable approach.

**Use recursive Rust when:** demonstrating the structural correspondence to OCaml, teaching pattern matching on slices, or when the problem is inherently recursive and filter is just a component.
