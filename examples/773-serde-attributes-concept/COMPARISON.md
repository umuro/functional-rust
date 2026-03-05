# OCaml vs Rust: Serde Attributes Concept

## Field Attributes

### Rust (with serde)
```rust
#[derive(Serialize)]
struct User {
    id: u64,
    #[serde(rename = "user_name")]
    username: String,
    #[serde(skip)]
    password_hash: String,
    #[serde(default)]
    display_name: Option<String>,
}
```

### OCaml (ppx_deriving_yojson)
```ocaml
type user = {
  id: int;
  username: string [@key "user_name"];
  (* No direct skip - use wrapper type *)
  display_name: string option [@default None];
} [@@deriving yojson]
```

## Common Serde Attributes

| Attribute | Purpose |
|-----------|---------|
| `rename` | Different JSON key name |
| `skip` | Don't serialize this field |
| `default` | Use Default::default() if missing |
| `flatten` | Inline nested struct |
| `rename_all` | Apply case conversion to all fields |

## Case Conversion

### Rust
```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    database_host: String,  // -> "databaseHost"
}
```

### Manual Implementation
```rust
pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut cap_next = false;
    for c in s.chars() {
        if c == '_' { cap_next = true; }
        else if cap_next { result.push(c.to_ascii_uppercase()); cap_next = false; }
        else { result.push(c); }
    }
    result
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Attribute syntax | `[@key ...]` | `#[serde(...)]` |
| Skip field | Use wrapper type | `#[serde(skip)]` |
| Derive | `[@@deriving yojson]` | `#[derive(Serialize)]` |
