# OCaml vs Rust: Async Blocks and Lazy Evaluation

## Lazy Computation Creation

**OCaml:**
```ocaml
let lazy_comp label f =
  Printf.printf "Creating: %s\n" label;
  fun () -> Printf.printf "Executing: %s\n" label; f ()
```

**Rust:**
```rust
fn lazy_comp<F: FnOnce() -> T, T>(label: &str, f: F) -> impl FnOnce() -> T + '_ {
    println!("Creating: {label}");
    move || { println!("Executing: {label}"); f() }
}
```

## Conditional Execution

**OCaml:**
```ocaml
let run_if cond thunk = if cond then Some (thunk ()) else None
```

**Rust:**
```rust
fn run_if<F: FnOnce() -> T, T>(cond: bool, t: F) -> Option<T> {
    if cond { Some(t()) } else { None }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Thunk type | `unit -> 'a` | `impl FnOnce() -> T` |
| Closure syntax | `fun () -> ...` | `\|\| { ... }` |
| Move semantics | Implicit (GC) | Explicit `move` keyword |
| Type constraints | Inferred | Explicit trait bounds |
| Laziness | Explicit thunks | Implicit in async, explicit here |
