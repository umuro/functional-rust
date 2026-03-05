# Comparison: Option Monad

## Monadic Bind

**OCaml:**
```ocaml
let bind m f = match m with None -> None | Some x -> f x
let ( >>= ) = bind

safe_div 100 4 >>= fun q ->
safe_sqrt q >>= fun r ->
Some (Float.to_int r)
```

**Rust:**
```rust
safe_div(100, 4)
    .and_then(|q| safe_sqrt(q))
    .map(|r| r as i32)
```

## Rust's ? Operator (Monadic Sugar)

**Rust:**
```rust
fn compute(a: i32, b: i32) -> Option<i32> {
    let q = safe_div(a, b)?;   // returns None early if None
    let r = safe_sqrt(q)?;     // returns None early if None
    Some(r as i32)
}
```

## Chained Lookups

**OCaml:**
```ocaml
lookup "HOME" env >>= fun home ->
lookup home paths >>= fun dirs ->
if List.mem "documents" dirs then Some "found" else None
```

**Rust:**
```rust
env.get("HOME")
    .and_then(|home| paths.get(*home))
    .and_then(|dirs| {
        if dirs.contains(&"documents") { Some("found") } else { None }
    })
```
