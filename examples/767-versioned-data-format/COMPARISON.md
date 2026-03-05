# OCaml vs Rust: Versioned Data Format

## Version Handling

### Rust
```rust
pub enum Data {
    V1(DataV1),
    V2(DataV2),
    V3(DataV3),
}

impl Data {
    pub fn upgrade(self) -> DataV3 {
        match self {
            Data::V1(v1) => DataV3 { name: v1.name, value: v1.value as f64, .. },
            Data::V2(v2) => DataV3 { name: v2.name, value: v2.value as f64, tags: v2.tags, .. },
            Data::V3(v3) => v3,
        }
    }
}
```

### OCaml
```ocaml
type data =
  | V1 of data_v1
  | V2 of data_v2
  | V3 of data_v3

let upgrade = function
  | V1 v1 -> { name = v1.name; value = float_of_int v1.value; tags = []; metadata = [] }
  | V2 v2 -> { name = v2.name; value = float_of_int v2.value; tags = v2.tags; metadata = [] }
  | V3 v3 -> v3
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Enum variants | `V1 of type` | `V1(Type)` |
| Upgrade | Function | Method |
| Default fields | Must specify all | Struct update syntax |
| Type conversion | `float_of_int` | `as f64` |
