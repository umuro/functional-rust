# OCaml vs Rust: Iterator inspect()

## Side-by-Side Code

### OCaml
```ocaml
let tap f x = f x; x

let result =
  [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
  |> List.map (tap (fun x -> Printf.printf "[in:%d] " x))
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (tap (fun x -> Printf.printf "[even:%d] " x))
  |> List.map (fun x -> x * x)
```

### Rust (idiomatic)
```rust
let result: Vec<i32> = (1..=10)
    .inspect(|x| print!("[in:{x}] "))
    .filter(|x| x % 2 == 0)
    .inspect(|x| print!("[even:{x}] "))
    .map(|x| x * x)
    .collect();
```

### Rust (capturing observations into a Vec)
```rust
let mut evens_seen = Vec::new();
let result: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0)
    .inspect(|&x| evens_seen.push(x))
    .map(|x| x * x)
    .collect();
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tap helper | `val tap : ('a -> unit) -> 'a -> 'a` | built-in `.inspect(f)` adapter |
| Closure argument | `fun x -> ...` (owned value) | `\|x\| ...` (shared reference `&T`) |
| Pipeline operator | `\|>` (pipe-forward) | method chaining on `Iterator` |
| Collection | `'a list` | `Vec<T>` via `.collect()` |

## Key Insights

1. **Built-in vs. user-defined tap**: OCaml has no standard `tap` so you define it yourself (`let tap f x = f x; x`). Rust ships `.inspect()` as a first-class iterator adapter in `std`.
2. **Reference semantics in closures**: Rust's `.inspect(|x| ...)` receives `&T`, not `T`, because the iterator adapter borrows each element before passing it on. OCaml's `tap` receives the value directly since OCaml is garbage-collected and immutable by default.
3. **Lazy vs. eager evaluation**: Rust iterators are lazy — `.inspect()` closures only fire when the chain is driven by `.collect()` or a terminal. OCaml's `List.map` is strict, so the side effects execute immediately at each `|>` stage.
4. **Atomic counters for shared state**: When an `inspect` closure needs to mutate shared state (e.g., a counter) across a lazy chain, Rust requires `AtomicUsize` or `Cell` because the closure borrows the chain environment; OCaml can just close over a `ref` counter without ceremony.
5. **Production uses**: Both languages use the tap pattern for logging and metrics, but Rust's `.inspect()` integrates naturally with `tracing` spans and structured logging, while OCaml typically uses a logging library called in the tap closure.

## When to Use Each Style

**Use idiomatic Rust `.inspect()`** when debugging a live iterator pipeline, adding metrics counters, or wiring up tracing events — any case where you need to observe without restructuring the chain.
**Use the capturing-Vec pattern** in tests and library code where you need to assert on intermediate values without printing to stdout.
