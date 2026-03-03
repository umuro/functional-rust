# Comparison: Scan Left — OCaml vs Rust

## Core Insight

Scan is fold's verbose cousin — it keeps all intermediate results. OCaml lacks a built-in `scan_left`, so you build it by accumulating into a reversed list during `fold_left`. Rust provides `Iterator::scan` in the standard library, with a mutable state parameter — more flexible but slightly different semantics (it doesn't automatically include the initial value).

## OCaml

```ocaml
let scan_left f init lst =
  let _, result = List.fold_left (fun (acc, res) x ->
    let acc' = f acc x in (acc', acc' :: res)
  ) (init, [init]) lst in
  List.rev result
```

## Rust — Iterator::scan

```rust
xs.iter().scan(init, |state, &x| {
    *state = f(*state, x);
    Some(*state)
})
```

## Rust — Manual

```rust
pub fn scan_left<T: Clone>(f: impl Fn(&T, &T) -> T, init: T, xs: &[T]) -> Vec<T> {
    let mut result = vec![init.clone()];
    let mut acc = init;
    for x in xs { acc = f(&acc, x); result.push(acc.clone()); }
    result
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Built-in | No | `Iterator::scan` |
| Returns init | Must include manually | Not included by default |
| Laziness | Eager (list) | Lazy (iterator) |
| State | Tuple `(acc, result_list)` | `&mut state` parameter |
| Reverse needed | Yes (`List.rev`) | No (push appends) |
| Result type | `'a list` | `impl Iterator<Item = T>` |

## Learner Notes

- **`Iterator::scan` quirk**: Rust's scan doesn't include the initial value — you need `once(init).chain(scan(...))` if you want it
- **Mutable state**: Rust's scan gives `&mut State` — you mutate it and return `Some(output)`, which can differ from state
- **`List.rev` cost**: OCaml's cons-based building produces reversed results; the final `List.rev` is O(n)
- **Practical use**: Running totals, prefix sums, moving averages — scan is underused but powerful
- **Relationship**: `xs.fold(init, f)` == `xs.scan(init, f).last()` — fold is scan without history
