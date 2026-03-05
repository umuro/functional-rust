# OCaml vs Rust: Trampoline Pattern

## Trampoline Type

### OCaml
```ocaml
type 'a bounce = Done of 'a | Bounce of (unit -> 'a bounce)
```

### Rust
```rust
enum Bounce<T> {
    Done(T),
    More(Box<dyn FnOnce() -> Bounce<T>>),
}
```

## Runner

### OCaml
```ocaml
let run t =
  let rec go = function
    | Done v    -> v
    | Bounce th -> go (th ())
  in go t
```

### Rust
```rust
fn run<T>(mut b: Bounce<T>) -> T {
    loop {
        match b {
            Bounce::Done(v) => return v,
            Bounce::More(th) => b = th(),
        }
    }
}
```

## Why Trampolines?

Rust doesn't have tail call optimization. Deep recursion will stack overflow.
Trampolines convert recursion to iteration:

1. Return `Done(value)` for base case
2. Return `More(thunk)` for recursive case
3. `run` loops until `Done`

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **TCO** | Often available | Not guaranteed |
| **Closure boxing** | Implicit | Explicit `Box<dyn FnOnce>` |
| **Run loop** | Recursive go | Iterative loop |
