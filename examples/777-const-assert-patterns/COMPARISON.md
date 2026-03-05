# OCaml vs Rust: Const Assert Patterns

## Compile-Time Assertions

### Rust
```rust
#[macro_export]
macro_rules! const_assert {
    ($cond:expr) => {
        const _: () = assert!($cond);
    };
}

const_assert!(BUFFER_SIZE.is_power_of_two());
const_assert!(MIN < MAX);
```

### OCaml
No compile-time assertions. Must use runtime:
```ocaml
let () =
  if not (buffer_size land (buffer_size - 1) = 0) then
    failwith "buffer size must be power of two"
```

## Size Assertions

### Rust
```rust
const_assert_size!(Header, 16);
const_assert!(std::mem::align_of::<Data>() == 8);
```

### OCaml
```ocaml
(* No compile-time size checks *)
let () = assert (Obj.size (Obj.repr header) = 4)
```

## Validated Constants

### Rust
```rust
pub const fn non_zero(n: u64) -> u64 {
    assert!(n != 0, "must be non-zero");
    n
}

pub const DIVISOR: u64 = non_zero(42); // Checked at compile time!
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Compile-time assert | None | `const { assert!() }` |
| Size check | Runtime only | Compile-time |
| Config validation | Module init | Before compilation |
| Error timing | Runtime | Compile time |
