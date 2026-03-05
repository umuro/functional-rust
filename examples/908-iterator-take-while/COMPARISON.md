# OCaml vs Rust: Conditional Stopping with take_while()

## Side-by-Side Code

### OCaml

```ocaml
let rec take_while pred = function
  | [] -> []
  | x :: xs -> if pred x then x :: take_while pred xs else []

let () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8; 9] in
  let small = take_while (fun x -> x < 5) nums in
  Printf.printf "Less than 5: %s\n"
    (String.concat ", " (List.map string_of_int small));

  let data = [3; 1; 4; 1; -5; 9; -2; 6] in
  let positives = take_while (fun x -> x > 0) data in
  Printf.printf "Leading positives: %s\n"
    (String.concat ", " (List.map string_of_int positives))
```

### Rust (idiomatic)

```rust
pub fn take_while_less_than(slice: &[i32], threshold: i32) -> Vec<i32> {
    slice.iter().copied().take_while(|&x| x < threshold).collect()
}

pub fn leading_positives(slice: &[i32]) -> Vec<i32> {
    slice.iter().copied().take_while(|&x| x > 0).collect()
}

// Works on infinite iterators — OCaml lists cannot be infinite
pub fn triangular_indices_below(limit: u64) -> Vec<u64> {
    (1u64..).take_while(|&n| n * (n + 1) / 2 < limit).collect()
}
```

### Rust (functional/recursive — mirrors OCaml)

```rust
pub fn take_while_rec<T, F>(slice: &[T], pred: F) -> Vec<T>
where
    T: Copy,
    F: Fn(T) -> bool,
{
    match slice {
        [] => vec![],
        [x, rest @ ..] => {
            if pred(*x) {
                let mut result = vec![*x];
                result.extend(take_while_rec(rest, pred));
                result
            } else {
                vec![]
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val take_while : ('a -> bool) -> 'a list -> 'a list` | `fn take_while(pred: impl Fn(&T) -> bool) -> impl Iterator<Item=T>` |
| Sequence type | `'a list` (finite, eager) | `impl Iterator<Item=T>` (potentially infinite, lazy) |
| Predicate | `'a -> bool` | `Fn(&T) -> bool` or `Fn(T) -> bool` |
| Output | `'a list` | `Vec<T>` (after `.collect()`) |

## Key Insights

1. **Laziness is the killer feature.** Rust's `take_while` is lazy — it evaluates elements one at a time and stops immediately. OCaml's standard library operates on finite lists. Rust can apply `take_while` to `(0u64..)` (an infinite range) without issue; OCaml requires `Seq.take_while` for lazy sequences.

2. **Permanent stop, not filter.** Both OCaml and Rust `take_while` stop at the *first* predicate failure and never resume — this is the key distinction from `filter`. A trailing element that would match is silently excluded if it comes after a non-matching element.

3. **Ownership and the closure.** In Rust the predicate closure captures by reference and the double-dereference pattern `|&&x|` (or `.copied()` before `take_while`) handles the indirection introduced by `.iter()`. OCaml's GC avoids this entirely — values are freely passed without ownership concerns.

4. **Slice patterns in the recursive version.** Rust's `[x, rest @ ..]` slice pattern mirrors OCaml's `x :: xs` cons-cell pattern almost exactly, making the structural recursion legible to anyone familiar with OCaml pattern matching.

5. **`take_while` composes.** Because it is an iterator adapter, it can be chained with `map`, `filter`, `enumerate`, etc., without intermediate allocation — the full chain remains lazy until `.collect()` forces evaluation.

## When to Use Each Style

**Use idiomatic Rust (`iter().take_while(...)`) when:** working in a pipeline, dealing with infinite iterators, or when performance matters — the adapter is zero-cost and composes freely.

**Use recursive Rust when:** teaching the OCaml parallel, working with algebraic data structures like linked lists, or when the recursive structure makes the termination condition more explicit for readers.
