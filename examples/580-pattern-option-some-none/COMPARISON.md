# OCaml vs Rust: Option Pattern Matching

## Basic Option Functions

### OCaml
```ocaml
let safe_div a b = if b = 0 then None else Some (a / b)
let safe_sqrt x = if x < 0.0 then None else Some (sqrt x)
```

### Rust
```rust
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}
fn safe_sqrt(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}
```

## Chaining with Monadic Bind

### OCaml
```ocaml
let (let*) = Option.bind

let compute a b =
  let* q = safe_div a b in
  let* r = safe_sqrt (float_of_int q) in
  Some (r *. 2.0)
```

### Rust
```rust
fn compute(a: i32, b: i32) -> Option<f64> {
    safe_div(a, b)
        .map(|q| q as f64)
        .and_then(safe_sqrt)
        .map(|r| r * 2.0)
}
```

## Common Combinators

| Operation | OCaml | Rust |
|-----------|-------|------|
| **Transform inner** | `Option.map f opt` | `opt.map(f)` |
| **Chain fallible** | `Option.bind opt f` | `opt.and_then(f)` |
| **Filter** | `Option.filter_map f lst` | `iter.filter_map(f)` |
| **Default** | `Option.value opt ~default:x` | `opt.unwrap_or(x)` |
| **Flatten** | Two binds | `opt.flatten()` |
| **Zip** | Manual | `opt1.zip(opt2)` |

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Bind syntax** | `let*` with custom operator | `.and_then()` method |
| **Method chaining** | Pipeline `\|>` | Dot `.` chaining |
| **filter_map** | `List.filter_map` | `.filter_map()` on iterators |
| **unwrap** | `Option.get` (may raise) | `.unwrap()` (panics) |
| **Safe default** | `Option.value ~default:x` | `.unwrap_or(x)` |
