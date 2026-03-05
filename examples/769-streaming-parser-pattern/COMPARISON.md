# OCaml vs Rust: Streaming Parser Pattern

## State Machine Parser

### Rust
```rust
pub enum ParseState {
    Ready,
    InBody { remaining: usize },
    Complete,
    Error(String),
}

pub struct StreamingParser {
    state: ParseState,
    buffer: Vec<u8>,
}

impl StreamingParser {
    pub fn feed(&mut self, data: &[u8]) -> usize {
        for &byte in data {
            match &self.state {
                ParseState::Ready => { /* ... */ }
                ParseState::InBody { remaining } => { /* ... */ }
                _ => break,
            }
        }
    }
}
```

### OCaml
```ocaml
type parse_state =
  | Ready
  | In_body of { remaining: int }
  | Complete
  | Error of string

type parser = {
  mutable state: parse_state;
  mutable buffer: bytes;
}

let feed parser data =
  (* Process byte by byte *)
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Mutability | `mutable` fields | `&mut self` |
| State enum | Variants | Enum with data |
| Buffer growth | `Bytes.extend` | `Vec::push` |
| Take pattern | Manual clear | `std::mem::take` |
