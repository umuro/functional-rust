# OCaml vs Rust: Const Lookup Table

## Compile-Time Table Generation

### Rust
```rust
pub const fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        // compute CRC entry...
        i += 1;
    }
    table
}

// Generated at compile time, embedded in binary
pub const CRC32_TABLE: [u32; 256] = generate_crc32_table();
```

### OCaml
```ocaml
(* Generated at runtime, module initialization *)
let crc32_table =
  Array.init 256 (fun i ->
    let rec loop crc j =
      if j = 8 then crc
      else if crc land 1 <> 0 then
        loop ((crc lsr 1) lxor 0xEDB88320) (j + 1)
      else
        loop (crc lsr 1) (j + 1)
    in
    loop i 0
  )
```

## Using Lookup Tables

### Rust
```rust
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF;
    for &byte in data {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];
    }
    !crc
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Table generation | Runtime | Compile time |
| Binary size | Code only | Code + table data |
| Startup time | Table computed | Zero (embedded) |
| Optimization | JIT possible | Inline possible |
