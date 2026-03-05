# OCaml vs Rust: Stateful Accumulation with scan()

## Side-by-Side Code

### OCaml
```ocaml
let scan init f lst =
  let (_, result) = List.fold_left (fun (acc, acc_list) x ->
    let new_acc = f acc x in
    (new_acc, acc_list @ [new_acc])
  ) (init, []) lst in
  result

let () =
  let nums = [1; 2; 3; 4; 5] in
  let running_sum = scan 0 (+) nums in
  Printf.printf "Running sum: %s\n"
    (String.concat ", " (List.map string_of_int running_sum));

  let transactions = [100; -30; 50; -80; 200] in
  let balances = scan 0 (+) transactions in
  Printf.printf "Balances: %s\n"
    (String.concat ", " (List.map string_of_int balances))
```

### Rust (idiomatic)
```rust
pub fn running_sum(nums: &[i64]) -> Vec<i64> {
    nums.iter()
        .scan(0i64, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}
```

### Rust (functional/generic — mirrors OCaml's `scan init f lst`)
```rust
pub fn scan<T, F>(init: T, mut f: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: FnMut(T, T) -> T,
{
    items
        .iter()
        .scan(init, |state, &x| {
            *state = f(*state, x);
            Some(*state)
        })
        .collect()
}
```

### Rust (early termination — no OCaml equivalent without manual recursion)
```rust
pub fn running_sum_until(nums: &[i64], limit: i64) -> Vec<i64> {
    nums.iter()
        .scan(0i64, |state, &x| {
            *state += x;
            if *state > limit { None } else { Some(*state) }
        })
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| scan signature | `val scan : 'a -> ('a -> 'b -> 'a) -> 'b list -> 'a list` | `fn scan<T, F>(init: T, f: F, items: &[T]) -> Vec<T>` |
| Iterator scan | N/A (must build with fold) | `Iterator::scan(init, \|state, item\| -> Option<B>)` |
| State threading | tuple in fold accumulator | `&mut state` in closure |
| Early termination | manual recursion required | `None` from closure stops iterator |
| Input type | `'b list` | `&[T]` (borrowed slice) |
| Output type | `'a list` | `Vec<T>` (owned) |

## Key Insights

1. **Built-in vs hand-rolled**: OCaml has no built-in `scan`; it must be implemented using `List.fold_left` with a tuple accumulator `(state, result_list)`. Rust provides `Iterator::scan` as a first-class lazy iterator adapter.

2. **Lazy vs eager**: Rust's `scan` is a lazy iterator — it produces values on demand and composes with other adapters without allocating intermediate collections. OCaml's `fold_left`-based scan builds the entire result list eagerly, and the `acc_list @ [new_acc]` pattern is O(n²) due to list append.

3. **Mutable state model**: Rust threads state via `&mut state` — the closure mutates it in place, which is explicit and zero-cost. OCaml uses immutable tuple `(acc, acc_list)` replaced at each step, relying on the GC to reclaim old values.

4. **Early termination**: Returning `None` from Rust's `scan` closure halts the iterator immediately — no values are computed beyond that point. Replicating this in OCaml requires switching to explicit recursion with a base case, since `fold_left` cannot short-circuit.

5. **Generality**: Rust's `scan` adapter works over any `Iterator`, not just lists — it can scan over file lines, network packets, channels, or infinite ranges without buffering the entire input. This makes it suitable for streaming and real-time accumulation tasks that OCaml's list-based approach cannot express directly.

## When to Use Each Style

**Use idiomatic Rust (`Iterator::scan`) when:** you need a lazy, composable running accumulation — prefix sums, running totals, state machine output — especially over large or infinite sequences where you don't want to materialise the whole input first.

**Use the generic `scan<T, F>` wrapper when:** you want an API that directly mirrors the OCaml `scan init f lst` calling convention, passing the combining function as a value rather than inlining it in the closure.

**Use early-terminating scan (`None` return) when:** you want to take a prefix of the accumulation up to some condition — e.g., "give me balances until the account goes negative" — which fold cannot express without post-processing.
