# OCaml vs Rust: Move Closures

## Greeter Factory

**OCaml:**
```ocaml
let make_greeter name = fun () -> Printf.printf "Hello, %s!\n" name
```

**Rust:**
```rust
fn make_greeter(name: String) -> impl Fn() {
    move || println!("Hello, {name}!")
}
```

## Counter with State

**OCaml:**
```ocaml
let make_counter start =
  let count = ref start in
  fun () -> let v = !count in incr count; v
```

**Rust:**
```rust
fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || { let v = count; count += 1; v }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Capture | Implicit by reference | Explicit `move` keyword |
| Mutable state | `ref` cell | `mut` variable in closure |
| Shared ownership | GC handles | `Arc::clone()` pattern |
| Thread safety | GIL / manual | Enforced by `Send`/`Sync` |
