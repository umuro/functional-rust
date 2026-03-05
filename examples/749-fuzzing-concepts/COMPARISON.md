# OCaml vs Rust: Fuzzing Concepts

## Fuzz Target Setup

### Rust (cargo-fuzz)
```rust
// fuzz/fuzz_targets/parse_packet.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = my_crate::parse_packet(data);  // must NEVER panic
});
```

Run with: `cargo fuzz run parse_packet`

### OCaml (afl-fuzz)
```ocaml
(* afl_input.ml *)
let () =
  let input = In_channel.input_all In_channel.stdin in
  let _ = My_lib.parse_packet input in
  ()
```

## Key Principle: Never Panic

### Rust
```rust
pub fn parse_packet(data: &[u8]) -> Result<Packet, ParseError> {
    if data.len() < 2 {
        return Err(ParseError::TooShort);
    }
    let version = data[0];
    if version == 0 || version > 5 {
        return Err(ParseError::InvalidVersion(version));
    }
    // ... safe parsing with bounds checks
    Ok(packet)
}
```

### OCaml
```ocaml
let parse_packet data =
  if String.length data < 2 then
    Error TooShort
  else
    let version = Char.code data.[0] in
    if version = 0 || version > 5 then
      Error (InvalidVersion version)
    else
      (* ... safe parsing *)
      Ok packet
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Fuzzing tool | afl-fuzz, crowbar | cargo-fuzz (libFuzzer) |
| Input format | stdin/file | `&[u8]` parameter |
| Coverage | afl instrumentation | LLVM sanitizers |
| Setup complexity | Manual | `cargo fuzz init` |
| Structured fuzzing | crowbar | arbitrary crate |
