# Comparison: Example 183 — Heterogeneous Vector

## GADT Type Witness vs Any

### OCaml
```ocaml
type _ ty = TInt : int ty | TString : string ty | TBool : bool ty
type entry = Entry : 'a ty * 'a -> entry

let get_int (Entry (ty, v)) = match ty with TInt -> Some v | _ -> None

let entries = [Entry (TInt, 42); Entry (TString, "hello")]
```

### Rust
```rust
use std::any::Any;

let mut items: Vec<Box<dyn Any>> = Vec::new();
items.push(Box::new(42i64));
items.push(Box::new(String::from("hello")));

let n: Option<&i64> = items[0].downcast_ref::<i64>();  // Some(&42)
let s: Option<&i64> = items[1].downcast_ref::<i64>();  // None
```

## Display + Downcast

### OCaml
```ocaml
let to_string_entry (Entry (ty, v)) = match ty with
  | TInt -> string_of_int v
  | TString -> v
  | TBool -> string_of_bool v
```

### Rust
```rust
trait AnyDisplay: Any + fmt::Display {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + fmt::Display> AnyDisplay for T {
    fn as_any(&self) -> &dyn Any { self }
}

// Can display AND downcast
let item: Box<dyn AnyDisplay> = Box::new(42);
println!("{}", item);                        // Display
let n = item.as_any().downcast_ref::<i32>(); // Downcast
```

## Enum-Based

### OCaml
```ocaml
type value = VInt of int | VStr of string | VBool of bool
let as_int = function VInt n -> Some n | _ -> None
```

### Rust
```rust
enum Value { Int(i64), Str(String), Bool(bool) }
impl Value {
    fn as_int(&self) -> Option<i64> {
        match self { Value::Int(n) => Some(*n), _ => None }
    }
}
```
