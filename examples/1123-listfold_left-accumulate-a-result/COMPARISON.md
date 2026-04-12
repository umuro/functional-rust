# OCaml vs Rust: List.fold_left — Accumulate a Result

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5]
let sum = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers

let rec fold_left_rec f acc = function
  | [] -> acc
  | x :: rest -> fold_left_rec f (f acc x) rest
```

### Rust (idiomatic — specialized adapters)
```rust
pub fn sum_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().copied().sum()
}

pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(i64::max)
}
```

### Rust (functional/recursive fold)
```rust
pub fn fold_left_recursive<T, U, F>(items: &[T], acc: U, f: &F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => acc,
        [head, rest @ ..] => {
            let new_acc = f(acc, head);
            fold_left_recursive(rest, new_acc, f)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| fold_left | `('a -> 'b -> 'a) -> 'a -> 'b list -> 'a` | `fn fold_left<T, U, F>(items: &[T], init: U, f: F) -> U` |
| sum | `List.fold_left ( + ) 0 xs` | `xs.iter().copied().sum()` |
| product | `List.fold_left ( * ) 1 xs` | `xs.iter().copied().product()` |
| max | `List.fold_left max min_int xs` | `xs.iter().copied().reduce(i64::max)` |
| empty result | implicit (returns `init`) | `None` for `reduce`, `0` for `sum` |

## Key Insights

1. **Curried operators**: OCaml's `( + )` is a curried function you pass directly to `fold_left`; Rust needs a closure `|acc, &x| acc + x` or uses the specialized `.sum()` adapter.
2. **Specialized vs. generic**: Rust's `Iterator` provides `.sum()` and `.product()` as specializations that clippy prefers over manual folds; OCaml expresses both uniformly via `fold_left`.
3. **Empty list**: OCaml `fold_left f init []` returns `init`; Rust's `Iterator::reduce` returns `None` for empty input (no initial value), while `fold` with an initial value behaves like OCaml.
4. **Argument order**: OCaml `fold_left f init list`; Rust `iter.fold(init, f)` — the method receiver is the iterator, not a standalone function.
5. **Tail recursion**: OCaml `fold_left` is tail-recursive in the standard library; Rust's `fold` is also iterative. The manual recursive Rust version risks stack overflow on very long inputs.

## When to Use Each Style

**Use `.sum()` / `.product()` when:** computing simple numeric aggregations — Rust's specialized adapters are clearer and potentially more efficient.
**Use `.fold()` when:** the accumulation logic is custom and doesn't fit a standard adapter — e.g., building a `HashMap` or filtering while accumulating.
**Use recursive fold when:** demonstrating the OCaml parallel or building educational examples that show the structural recursion explicitly.
