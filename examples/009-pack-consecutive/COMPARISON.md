# Comparison: Pack Consecutive Duplicates

## OCaml — Accumulator recursion

```ocaml
let pack lst =
  let rec aux current acc = function
    | [] -> []
    | [x] -> (x :: current) :: acc
    | h1 :: (h2 :: _ as t) ->
        if h1 = h2 then aux (h1 :: current) acc t
        else aux [] ((h1 :: current) :: acc) t
  in
  List.rev (aux [] [] lst)
```

## Rust — Idiomatic (imperative)

```rust
pub fn pack<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
    if list.is_empty() { return vec![]; }
    let mut result = Vec::new();
    let mut current = vec![list[0].clone()];
    for item in &list[1..] {
        if *item == current[0] {
            current.push(item.clone());
        } else {
            result.push(current);
            current = vec![item.clone()];
        }
    }
    result.push(current);
    result
}
```

## Rust — Functional (fold)

```rust
pub fn pack_fold<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
    list.iter().fold(Vec::new(), |mut acc, item| {
        match acc.last_mut() {
            Some(group) if group[0] == *item => group.push(item.clone()),
            _ => acc.push(vec![item.clone()]),
        }
        acc
    })
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Accumulator | Two: `current` + `acc` | One: `Vec<Vec<T>>` with `last_mut()` |
| List building | Prepend + reverse | Append (amortized O(1)) |
| Zero-copy | Not possible | `pack_slices` returns `&[T]` |
| Pattern matching | `h1 :: (h2 :: _ as t)` | Iterator + conditional |
| Empty handling | Pattern match `[]` | Early return on `is_empty()` |

## Takeaways

1. Rust's `last_mut()` enables elegant fold-based grouping without OCaml's double-accumulator pattern
2. The slice-based version (`pack_slices`) showcases Rust's borrowing — zero-copy grouping
3. `Vec::push` appending eliminates the `List.rev` step needed in OCaml
4. Both languages use the same core algorithm — iterate and group — but express it differently
5. The ownership choice (clone vs borrow) is explicit in Rust, invisible in OCaml
