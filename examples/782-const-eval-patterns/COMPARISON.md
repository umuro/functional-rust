# OCaml vs Rust: Const Eval Patterns

## Compile-Time Functions

### Rust
```rust
pub const fn const_log2(n: usize) -> usize {
    (usize::BITS - n.leading_zeros() - 1) as usize
}

pub const fn const_next_pow2(n: usize) -> usize {
    1 << (usize::BITS - (n - 1).leading_zeros())
}

// Computed at compile time
pub const LOG2_256: usize = const_log2(256);
```

### OCaml
```ocaml
(* Computed at runtime *)
let log2 n = int_of_float (log (float_of_int n) /. log 2.)
let next_pow2 n = 1 lsl (log2 (n - 1) + 1)

let log2_256 = log2 256
```

## Compile-Time String Hash

### Rust
```rust
pub const fn const_hash(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

pub const HASH_HELLO: u64 = const_hash("hello");
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Evaluation | Runtime | Compile-time |
| Loop in const | N/A | `while` loop |
| Bit operations | Available | Available |
| Binary embedding | N/A | Values embedded |
