# OCaml vs Rust: List Fold Left (accumulate a result)

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5]

let sum     = List.fold_left (+) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let concat  =
  List.fold_left
    (fun acc x -> acc ^ " " ^ string_of_int x)
    "Numbers:"
    numbers

(* Explicit recursive definition — what the stdlib does internally *)
let rec fold_left f acc = function
  | [] -> acc
  | x :: xs -> fold_left f (f acc x) xs
```

### Rust (idiomatic)
```rust
pub fn fold_left<T, A, F>(f: F, init: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    items.iter().fold(init, f)
}

// usage
let numbers = [1, 2, 3, 4, 5];
let sum     = fold_left(|acc, x| acc + x, 0, &numbers);
let product = fold_left(|acc, x| acc * x, 1, &numbers);
let concat  = fold_left(
    |acc: String, x| acc + " " + &x.to_string(),
    String::from("Numbers:"),
    &numbers,
);
```

### Rust (functional / recursive)
```rust
pub fn fold_left_recursive<T, A, F>(f: &F, acc: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    match items {
        [] => acc,
        [h, rest @ ..] => fold_left_recursive(f, f(acc, h), rest),
    }
}
```

## Type Signatures

| Concept                     | OCaml                                                    | Rust                                                                   |
|-----------------------------|----------------------------------------------------------|------------------------------------------------------------------------|
| `fold_left` signature       | `val fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a` | `fn fold_left<T, A, F: Fn(A, &T) -> A>(F, A, &[T]) -> A`               |
| Combiner                    | `'a -> 'b -> 'a`                                         | `F: Fn(A, &T) -> A`                                                    |
| Initial accumulator         | `'a`                                                     | `A` (owned, passed and returned by value)                              |
| List                        | `'b list`                                                | `&[T]`                                                                 |
| Operator literal            | `(+)`, `( * )`                                           | closure: `|acc, x| acc + x`                                            |

## Key Insights

1. **`List.fold_left` ↔ `Iterator::fold`** — identical semantics: threads an accumulator left-to-right and returns the final value. The stdlib implementation of `Iterator::fold` is already a tight `while let` loop.
2. **Accumulator passed by value** — both languages treat the accumulator as a value (`A`), not a reference. This is why `fold` composes cleanly with immutable data: each step returns the new accumulator rather than mutating one in place.
3. **Operators-as-functions** — OCaml can write `(+)` and pass it directly; Rust has no "operator as value" syntax, so you wrap it in `|a, b| a + b`. Named helpers like `i32::saturating_add` can be passed by path, but arithmetic operators cannot.
4. **Direction matters** — `fold_left` vs `fold_right` gives different results for non-associative operations. `fold_left (-) 0 [1;2;3;4;5]` is `-15`; switching to `fold_right` yields `3`. Rust only exposes `fold` (left); right-fold requires `.rev().fold(...)` on a `DoubleEndedIterator`.
5. **Borrowing elements** — OCaml passes list elements by value (they are boxed/immutable); Rust's `.iter()` yields `&T`, so the closure takes `(A, &T) -> A`. When `T: Copy`, the closure can dereference; otherwise, work with the reference.

## When to Use Each Style

**Use idiomatic Rust when:** you want constant stack use on long slices, or when composing with other iterator adapters (`filter`, `map`, `take_while`). `Iterator::fold` inlines aggressively and is the right default.

**Use recursive Rust when:** you're teaching the OCaml → Rust correspondence, or the input is a genuinely recursive structure (binary trees, cons-lists) where slice indexing doesn't apply. Beware: Rust does not guarantee TCO, so recursive folds can blow the stack on long lists.
