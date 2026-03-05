# OCaml vs Rust: Zero-Copy Deserialize

## Borrowing vs Copying

### Rust (Zero-Copy)
```rust
pub struct Message<'a> {
    pub header: &'a str,  // Borrows from input
    pub body: &'a str,    // Borrows from input
}

pub fn parse_message(input: &str) -> Option<Message<'_>> {
    let pos = input.find('\n')?;
    Some(Message {
        header: &input[..pos],      // No allocation!
        body: &input[pos + 1..],    // No allocation!
    })
}
```

### OCaml (Copying)
```ocaml
type message = {
  header: string;  (* Owned copy *)
  body: string;    (* Owned copy *)
}

let parse_message input =
  match String.index_opt input '\n' with
  | None -> None
  | Some pos ->
      Some {
        header = String.sub input 0 pos;       (* Allocates new string *)
        body = String.sub input (pos + 1) ...; (* Allocates new string *)
      }
```

## Lifetime Annotations

### Rust
```rust
// 'a ties output lifetime to input
pub fn parse_kv(input: &str) -> Option<KeyValue<'_>> {
    let pos = input.find('=')?;
    Some(KeyValue {
        key: &input[..pos],
        value: &input[pos + 1..],
    })
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| String slicing | Always copies | Zero-copy with `&str` |
| Lifetime tracking | GC handles it | Explicit `'a` |
| Memory usage | O(n) for n fields | O(1) pointers |
| Input lifetime | Independent | Must outlive result |
