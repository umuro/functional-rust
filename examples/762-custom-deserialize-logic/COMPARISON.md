# OCaml vs Rust: Custom Deserialize Logic

## Deserialize Trait

### Rust
```rust
pub trait Deserialize: Sized {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError>;
}

impl Deserialize for u32 {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        input.read_u32().ok_or(DeserializeError::UnexpectedEof)
    }
}
```

### OCaml
```ocaml
module type DESERIALIZE = sig
  type t
  val deserialize : input -> (t, error) result
end

let deserialize_int32 input =
  match read_bytes input 4 with
  | Some bytes -> Ok (Bytes.get_int32_le bytes 0)
  | None -> Error Unexpected_eof
```

## Validation During Parse

### Rust
```rust
impl Deserialize for Email {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        let s = String::deserialize(input)?;
        if s.contains('@') {
            Ok(Email(s))
        } else {
            Err(DeserializeError::ValidationFailed("invalid email".into()))
        }
    }
}
```

### OCaml
```ocaml
let deserialize_email input =
  let* s = deserialize_string input in
  if String.contains s '@' then
    Ok (Email s)
  else
    Error (Validation_failed "invalid email")
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Return type | `result` | `Result<T, E>` |
| Error chaining | `let*` / `bind` | `?` operator |
| Newtype validation | Smart constructor | `impl Deserialize` |
| Buffer position | Mutable ref | Mutable struct |
