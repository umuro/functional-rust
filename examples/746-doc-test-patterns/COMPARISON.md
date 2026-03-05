# OCaml vs Rust: Documentation Tests

## Doc Comment Examples

### OCaml (odoc)
```ocaml
(**
  Clamp a value within [lo, hi].

  @example
  {[
    let _ = clamp 0 10 (-5)  (* = 0 *)
    let _ = clamp 0 10 5     (* = 5 *)
  ]}
*)
let clamp lo hi x = max lo (min hi x)
```
**Note:** OCaml's odoc does NOT execute these examples.

### Rust (rustdoc)
```rust
/// Clamps `x` to the inclusive range `[lo, hi]`.
///
/// # Examples
///
/// ```
/// use my_crate::clamp;
/// assert_eq!(clamp(0, 10, -5), 0);
/// assert_eq!(clamp(0, 10, 5), 5);
/// ```
pub fn clamp(lo: i32, hi: i32, x: i32) -> i32 {
    x.max(lo).min(hi)
}
```
**These examples are compiled and run by `cargo test`!**

## Hidden Setup Lines (Rust Only)

```rust
/// # Examples
///
/// ```
/// # use my_crate::helper;  // hidden in rendered docs
/// let result = helper();
/// assert!(result.is_ok());
/// ```
```

The `#` prefix includes the line in compilation but hides it in documentation.

## Testing Panics

### OCaml
```ocaml
let test_panic () =
  try
    let _ = factorial 0 in
    failwith "expected exception"
  with Invalid_argument _ -> ()
```

### Rust
```rust
/// ```should_panic
/// my_crate::factorial(0);  // this line panics
/// ```
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Doc examples executed | ❌ No | ✅ Yes, by `cargo test` |
| Hidden setup lines | ❌ No | ✅ `# use ...` syntax |
| Panic testing | Manual try/catch | `should_panic` attribute |
| Compile-fail tests | ❌ No | ✅ `compile_fail` attribute |
| Discovery | Manual odoc setup | Automatic with rustdoc |

## Why Rust's Approach Is Better

1. **Examples can't lie** — if API changes, doc tests fail
2. **Copy-paste friendly** — users know examples work
3. **No maintenance burden** — docs stay in sync automatically
4. **Panic behavior documented** — `should_panic` proves the documented behavior
