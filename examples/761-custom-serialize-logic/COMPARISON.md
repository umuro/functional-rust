# OCaml vs Rust: Custom Serialize Logic

## Custom Date Serialization

### Rust
```rust
impl Date {
    pub fn to_iso_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
    
    pub fn to_compact(&self) -> u32 {
        self.year as u32 * 10000 + self.month as u32 * 100 + self.day as u32
    }
}
```

### OCaml
```ocaml
let date_to_iso { year; month; day } =
  Printf.sprintf "%04d-%02d-%02d" year month day

let date_to_compact { year; month; day } =
  year * 10000 + month * 100 + day
```

## Secret Values (Redaction)

### Rust
```rust
impl<T> std::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Secret([REDACTED])")
    }
}
```

### OCaml
```ocaml
type 'a secret = Secret of 'a

let secret_to_string _ = "[REDACTED]"
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Custom Display | `to_string` function | `Display` trait |
| Custom Debug | `%a` format | `Debug` trait |
| Newtype wrapper | Single variant | Struct wrapper |
| Format control | Printf directives | format! macros |
