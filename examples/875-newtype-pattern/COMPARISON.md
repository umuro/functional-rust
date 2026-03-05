# Comparison: Newtype Pattern

## Abstract Type vs Tuple Struct

**OCaml:**
```ocaml
module UserId : sig
  type t
  val create : int -> t
  val value : t -> int
end = struct
  type t = int
  let create x = if x > 0 then x else failwith "invalid"
  let value x = x
end
```

**Rust:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

impl UserId {
    fn new(id: u64) -> Option<Self> {
        if id > 0 { Some(UserId(id)) } else { None }
    }
    fn value(self) -> u64 { self.0 }
}
```

## Temperature Units

**OCaml:**
```ocaml
type celsius = { celsius_value : float }
type fahrenheit = { fahrenheit_value : float }

let to_fahrenheit c =
  { fahrenheit_value = c.celsius_value *. 9.0 /. 5.0 +. 32.0 }
```

**Rust:**
```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl Celsius {
    fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}
```

## Validated Data

**OCaml:**
```ocaml
module Email : sig
  type t = private string
  val create : string -> t option
end = struct
  type t = string
  let create s = if String.contains s '@' then Some s else None
end
```

**Rust:**
```rust
struct Email(String);

impl Email {
    fn new(s: &str) -> Option<Self> {
        if s.contains('@') { Some(Email(s.to_string())) } else { None }
    }
}
```
