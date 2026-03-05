# OCaml vs Rust: Scan / Accumulate

## Side-by-Side Code

### OCaml
```ocaml
(* Manual scan: returns init + every intermediate state *)
let scan f init lst =
  let rec aux acc state = function
    | [] -> List.rev acc
    | x :: rest ->
      let new_state = f state x in
      aux (new_state :: acc) new_state rest
  in
  aux [init] init lst

let running_sum lst = scan ( + ) 0 lst
```

### Rust (idiomatic — using built-in `scan` iterator adapter)
```rust
pub fn running_sum(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}
```

### Rust (functional/generic — mirrors OCaml structure)
```rust
pub fn scan<S, T, F>(init: S, data: &[T], f: F) -> Vec<S>
where
    S: Clone,
    F: Fn(S, &T) -> S,
{
    let mut result = Vec::with_capacity(data.len() + 1);
    result.push(init.clone());
    let mut state = init;
    for item in data {
        state = f(state, item);
        result.push(state.clone());
    }
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic scan | `val scan : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a list` | `fn scan<S, T, F>(init: S, data: &[T], f: F) -> Vec<S>` |
| Running sum | `val running_sum : int list -> int list` | `fn running_sum(data: &[i32]) -> Vec<i32>` |
| Iterator state | implicit via closure capture | `|acc, &x|` — `acc` is `&mut S`, mutated in place |
| Early termination | not built-in | return `None` from the closure to stop the iterator |

## Key Insights

1. **Built-in adapter**: Rust's `Iterator::scan` is a first-class iterator adapter that carries mutable state between elements, eliminating the need for the manual recursive helper OCaml requires.
2. **Mutable state in closures**: Rust's `scan` closure receives `acc` as `&mut S` — you mutate it in place and return `Some(value)` to continue, or `None` to stop early. OCaml's recursive approach threads state explicitly through parameters.
3. **Ownership of state**: The generic `scan` function requires `S: Clone` because each intermediate value must be stored independently in the output `Vec`. Rust makes this cost explicit; OCaml's GC handles sharing silently.
4. **Lazy vs eager**: Rust's `.scan(...).collect()` is lazy until `collect` drives it, enabling fusion with other adapters (`.filter`, `.take`) before materialising results. OCaml's list-based scan is strict and allocates immediately.
5. **Practical pattern**: Balance histories, running statistics, and state-machine trajectories are the canonical use cases. In both languages the pattern is the same: carry one piece of mutable state and emit its value after each update.

## When to Use Each Style

**Use idiomatic Rust (`.scan` adapter) when:** you want composable, lazy iteration — chain `.scan` with `.take_while`, `.filter`, or `.map` before collecting, or when you only need part of the trajectory.

**Use the generic `scan` function when:** you want a reusable utility with an explicit initial value included in the output (matching OCaml's `scan f init lst` semantics), or when the state type requires non-`Copy` cloning that you want to make visible.
