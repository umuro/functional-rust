# OCaml vs Rust: Type Ascription and Dispatch

## Type Dispatch via Sum Types

### OCaml
```ocaml
type value = I of int | F of float | S of string | B of bool

let type_name = function
  | I _ -> "int"
  | F _ -> "float"
  | S _ -> "string"
  | B _ -> "bool"

let to_f64 = function
  | I n -> Some (float_of_int n)
  | F f -> Some f
  | S s -> (try Some (float_of_string s) with _ -> None)
  | _ -> None
```

### Rust
```rust
enum Value { Int(i64), Float(f64), Str(String), Bool(bool) }

impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Str(_) => "str",
            Value::Bool(_) => "bool",
        }
    }
    
    fn to_f64(&self) -> Option<f64> {
        match self {
            Value::Int(n) => Some(*n as f64),
            Value::Float(f) => Some(*f),
            Value::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
}
```

## Runtime Type Introspection

### OCaml
```ocaml
(* OCaml has no runtime type reflection *)
(* Must use sum types for dynamic typing *)
```

### Rust
```rust
use std::any::Any;

fn describe_any(v: &dyn Any) -> &'static str {
    if v.downcast_ref::<i32>().is_some() { "i32" }
    else if v.downcast_ref::<f64>().is_some() { "f64" }
    else { "unknown" }
}
```

## Type Casting

### OCaml
```ocaml
let x = 42
let f = float_of_int x    (* Explicit conversion function *)
```

### Rust
```rust
let x: i32 = 42;
let f = x as f64;         // `as` keyword for numeric casts
let safe = i64::try_from(x);  // TryFrom for fallible conversion
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Dynamic typing** | Sum types only | `Any` trait + sum types |
| **Type introspection** | None | `downcast_ref`, `type_id` |
| **Numeric casting** | `float_of_int` etc. | `as` keyword |
| **Safe conversion** | Custom functions | `TryFrom` trait |
| **Type erasure** | Limited | `dyn Any` |
