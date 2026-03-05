# OCaml vs Rust: Group By Iterator

## Side-by-Side Code

### OCaml
```ocaml
(* Group consecutive equal elements *)
let group_consecutive lst =
  match lst with
  | [] -> []
  | first :: rest ->
    let groups, current = List.fold_left (fun (groups, current) x ->
      match current with
      | [] -> (groups, [x])
      | hd :: _ when hd = x -> (groups, x :: current)
      | _ -> (List.rev current :: groups, [x])
    ) ([], [first]) rest in
    List.rev (List.rev current :: groups)

(* Group by key function *)
let group_by_key key lst =
  match lst with
  | [] -> []
  | first :: rest ->
    let groups, current_key, current = List.fold_left (fun (groups, k, current) x ->
      let new_key = key x in
      if new_key = k then (groups, k, x :: current)
      else (List.rev current :: groups, new_key, [x])
    ) ([], key first, [first]) rest in
    List.rev (List.rev current :: groups)
```

### Rust (idiomatic)
```rust
pub fn group_consecutive<T: PartialEq + Clone>(data: &[T]) -> Vec<Vec<T>> {
    let mut groups: Vec<Vec<T>> = Vec::new();
    let mut iter = data.iter();
    let Some(first) = iter.next() else { return groups; };
    let mut current = vec![first.clone()];

    for item in iter {
        if *item == *current[0] {
            current.push(item.clone());
        } else {
            groups.push(current);
            current = vec![item.clone()];
        }
    }
    groups.push(current);
    groups
}
```

### Rust (functional/recursive — run-length encoding via iterator composition)
```rust
pub fn run_length_encode<T: PartialEq + Clone>(data: &[T]) -> Vec<(T, usize)> {
    group_consecutive(data)
        .into_iter()
        .map(|g| {
            let len = g.len();
            (g.into_iter().next().unwrap(), len)
        })
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Group consecutive | `'a list -> 'a list list` | `fn group_consecutive<T: PartialEq + Clone>(data: &[T]) -> Vec<Vec<T>>` |
| Group by key | `('a -> 'b) -> 'a list -> 'a list list` | `fn group_by_key<T: Clone, K: PartialEq>(data: &[T], key: impl Fn(&T) -> K) -> Vec<(K, Vec<T>)>` |
| Run-length encode | `'a list -> ('a * int) list` | `fn run_length_encode<T: PartialEq + Clone>(data: &[T]) -> Vec<(T, usize)>` |
| List type | `'a list` | `&[T]` (borrowed slice) |
| Grouped result | `'a list list` | `Vec<Vec<T>>` |

## Key Insights

1. **Fold vs imperative accumulation:** OCaml uses `List.fold_left` with an immutable accumulator tuple `(groups, current)`, building the result purely functionally. Rust naturally reaches for mutable `Vec` accumulators inside a `for` loop — equally idiomatic and avoids repeated allocation overhead.

2. **Structural pattern matching vs. index comparison:** OCaml matches on the head of `current` with `hd :: _ when hd = x`, which is elegant list deconstruction. Rust compares via `current[0]` on a `Vec`, trading structural elegance for direct, bounds-clear indexing.

3. **Key function as higher-order argument:** Both languages accept a key function (`Fn(&T) -> K` in Rust, `'a -> 'b` in OCaml). Rust's trait bounds make the constraint explicit — `K: PartialEq` — while OCaml infers equality capability automatically via polymorphic `=`.

4. **Ownership and `Clone`:** Rust requires `T: Clone` to copy values into groups from a borrowed slice. OCaml's garbage collector and persistent lists make this cost invisible — values are always shared. The `Clone` bound is the honest cost of building owned sub-collections from a borrowed input.

5. **Iterator composition:** Rust's `run_length_encode` naturally composes `group_consecutive` with `.map().collect()`, mirroring functional pipeline style. OCaml achieves the same with `List.map` over the grouped result — both idioms express the same dataflow clearly.

## When to Use Each Style

**Use idiomatic Rust (mutable accumulator)** when: performance matters and you want to avoid repeated intermediate allocations; the grouping logic is straightforward and clarity trumps minimalism.

**Use iterator composition** when: you want to build higher-level operations from simpler ones (e.g., `run_length_encode` built on top of `group_consecutive`), keeping each function small, pure, and independently testable.
