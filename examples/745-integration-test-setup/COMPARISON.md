# OCaml vs Rust: Integration Test Setup

## Project Structure

### OCaml (Dune)
```
my_lib/
├── lib/
│   └── my_lib.ml
├── test/
│   ├── dune
│   ├── common.ml     (* shared test helpers *)
│   └── test_main.ml  (* integration tests *)
└── dune-project
```

### Rust (Cargo)
```
my_crate/
├── src/
│   └── lib.rs
├── tests/
│   ├── common/
│   │   └── mod.rs    // shared helpers (NOT a test binary)
│   ├── config_test.rs
│   └── api_test.rs
└── Cargo.toml
```

## Shared Test Helpers

### OCaml
```ocaml
(* test/common.ml *)
let test_config = MyLib.with_config "test-host" 9999 10

let make_test_config ?(host="test") ?(port=9999) ?(max=10) () =
  MyLib.with_config host port max
```

### Rust
```rust
// tests/common/mod.rs
use my_crate::{Config, validate_config};

pub fn test_config() -> Config {
    Config::new("test-host", 9999, 10)
}

pub fn assert_valid(c: &Config) {
    assert!(validate_config(c).is_ok(), "config should be valid: {:?}", c);
}
```

## Integration Tests

### OCaml
```ocaml
(* test/test_main.ml *)
let () =
  let cfg = Common.make_test_config () in
  assert (cfg.MyLib.port = 9999);
  
  match MyLib.parse_port "8080" with
  | MyLib.Ok n -> assert (n = 8080)
  | MyLib.Err e -> failwith e
```

### Rust
```rust
// tests/config_test.rs
mod common;

use my_crate::{Config, parse_port, ConfigError};

#[test]
fn default_config_is_valid() {
    let cfg = Config::default();
    common::assert_valid(&cfg);
}

#[test]
fn parse_port_rejects_invalid() {
    assert!(parse_port("0").is_err());
    assert!(parse_port("65536").is_err());
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Test location | `test/` directory with Dune config | `tests/` directory (auto-discovered) |
| Shared helpers | Separate module linked in | `tests/common/mod.rs` explicitly imported |
| Visibility | Module signatures | Only `pub` items visible from `tests/` |
| Running tests | `dune test` | `cargo test` |
| Single file | `dune test test_name` | `cargo test --test config_test` |
| Test binary | One or more executables | Each `tests/*.rs` is a separate crate |

## Why This Matters

Both OCaml and Rust enforce that integration tests can only access the public API:
- **OCaml**: Module signatures control what's exported
- **Rust**: `tests/` files are separate crates; they can only see `pub` items

This ensures your public API is tested as users will actually use it, not with access to private internals.
