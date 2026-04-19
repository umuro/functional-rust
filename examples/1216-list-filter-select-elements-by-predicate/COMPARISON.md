# OCaml vs Rust: List Filter (select elements by predicate)

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]

let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers
let pos   = List.filter (fun n -> n > 0)      (* partial application *)

let rec filter_rec p = function
  | [] -> []
  | x :: xs ->
    if p x then x :: filter_rec p xs
    else filter_rec p xs
```

### Rust (idiomatic)
```rust
pub fn filter<T, F>(predicate: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().copied().filter(|x| predicate(x)).collect()
}

// usage
let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
let evens = filter(|x: &i32| x % 2 == 0, &numbers);
let odds  = filter(|x: &i32| x % 2 != 0, &numbers);
```

### Rust (functional / recursive)
```rust
pub fn filter_recursive<T, F>(predicate: &F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    match items {
        [] => Vec::new(),
        [h, rest @ ..] => {
            let mut tail = filter_recursive(predicate, rest);
            if predicate(h) {
                let mut out = Vec::with_capacity(1 + tail.len());
                out.push(*h);
                out.append(&mut tail);
                out
            } else {
                tail
            }
        }
    }
}
```

## Type Signatures

| Concept                     | OCaml                                  | Rust                                                          |
|-----------------------------|----------------------------------------|---------------------------------------------------------------|
| `filter` signature          | `val filter : ('a -> bool) -> 'a list -> 'a list` | `fn filter<T: Copy, F: Fn(&T) -> bool>(F, &[T]) -> Vec<T>` |
| Predicate                   | `'a -> bool`                           | `F: Fn(&T) -> bool`                                           |
| List                        | `'a list`                              | `&[T]`                                                        |
| Returned collection         | new `'a list`                          | new `Vec<T>`                                                  |
| Partial application         | `let pos = List.filter p` (curried)    | closure: `let pos = |xs: &[i32]| filter(p, xs)`               |

## Key Insights

1. **`List.filter` ↔ `Iterator::filter`** — the named combinator exists in both languages; Rust chains it with `collect` because iterators are lazy.
2. **Predicate takes `&T`** — Rust predicates borrow their argument so filtering does not move or clone the element; only survivors are copied into the output.
3. **Curried partial application** — `List.filter p` in OCaml is already a function; in Rust the idiomatic translation is either a capturing closure or a generic function.  There is no "half-applied" function value without one.
4. **Pattern matching on slices** — `match items { [] => _, [h, rest @ ..] => _ }` is the slice-level equivalent of OCaml's `match lst with [] -> _ | x :: xs -> _`.
5. **Laziness** — OCaml's `List.filter` eagerly allocates; Rust's `.filter()` returns an iterator that only does work on demand, so chaining `.filter(...).take(3)` stops early for free.

## When to Use Each Style

**Use idiomatic Rust when:** you're composing with other iterator adapters (`map`, `take`, `sum`) or when you want to avoid the intermediate allocation by staying lazy until a final `collect`/`for_each`.

**Use recursive Rust when:** you're teaching the OCaml → Rust correspondence, or when the data structure is genuinely recursive (trees, cons-lists) and cannot be expressed as a slice.
