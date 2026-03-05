# OCaml vs Rust: JSON Format From Scratch

## JSON Value Type

### Rust
```rust
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}
```

### OCaml (Yojson-like)
```ocaml
type json =
  | `Null
  | `Bool of bool
  | `Float of float
  | `String of string
  | `List of json list
  | `Assoc of (string * json) list
```

## Serialization

### Rust
```rust
impl JsonValue {
    pub fn to_json(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", escape(s)),
            JsonValue::Array(arr) => format!("[{}]", 
                arr.iter().map(|v| v.to_json()).collect::<Vec<_>>().join(", ")),
            JsonValue::Object(obj) => format!("{{{}}}",
                obj.iter().map(|(k, v)| format!("\"{}\": {}", k, v.to_json()))
                    .collect::<Vec<_>>().join(", ")),
        }
    }
}
```

### OCaml
```ocaml
let rec to_string = function
  | `Null -> "null"
  | `Bool b -> string_of_bool b
  | `Float f -> string_of_float f
  | `String s -> Printf.sprintf "\"%s\"" (escape s)
  | `List lst -> Printf.sprintf "[%s]" 
      (String.concat ", " (List.map to_string lst))
  | `Assoc pairs -> Printf.sprintf "{%s}"
      (String.concat ", " (List.map (fun (k, v) -> 
        Printf.sprintf "\"%s\": %s" k (to_string v)) pairs))
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Variant syntax | Polymorphic variants | Enum |
| String format | `Printf.sprintf` | `format!` |
| List join | `String.concat` | `.join()` |
| Escape handling | Manual function | Manual function |
