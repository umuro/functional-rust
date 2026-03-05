# OCaml vs Rust: Continuation-Passing Style

## Direct vs CPS

### OCaml Direct
```ocaml
let rec fact n = if n <= 1 then 1 else n * fact (n-1)
```

### OCaml CPS
```ocaml
let rec fact_k n k =
  if n <= 1 then k 1
  else fact_k (n-1) (fun result -> k (n * result))
```

### Rust Direct
```rust
fn fact(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * fact(n - 1) }
}
```

### Rust CPS
```rust
fn fact_k<R>(n: u64, k: impl FnOnce(u64) -> R) -> R {
    if n <= 1 {
        k(1)
    } else {
        fact_k(n - 1, move |r| k(n * r))
    }
}
```

## Error Handling with CPS

### OCaml
```ocaml
let safe_div a b ok err =
  if b = 0.0 then err "division by zero"
  else ok (a /. b)
```

### Rust
```rust
fn safe_div_k<R>(
    a: f64, b: f64,
    ok: impl FnOnce(f64) -> R,
    err: impl FnOnce(&str) -> R
) -> R {
    if b == 0.0 { err("division by zero") }
    else { ok(a / b) }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Closure syntax** | `fun x -> ...` | `\|x\| ...` or `move \|x\| ...` |
| **Recursive CPS** | Direct | May need `Box<dyn FnOnce>` |
| **Type inference** | Full | Generic bounds needed |
| **Move semantics** | Implicit | Explicit with `move` |

## Why CPS?

1. **Explicit control flow** - No hidden returns
2. **Tail call optimization** - All calls are tail calls
3. **Error handling** - Multiple continuations for success/error
4. **Backtracking** - Save continuation for later use
