# OCaml vs Rust: From/Into Conversion Traits

## Side-by-Side Code

### OCaml — Module-based conversion
```ocaml
module type TryFrom = sig
  type source
  type target
  type error
  val try_from : source -> (target, error) result
end

module StringToInt : TryFrom
  with type source = string
   and type target = int
   and type error = string = struct
  type source = string
  type target = int
  type error = string
  let try_from s =
    match int_of_string_opt s with
    | Some n -> Ok n
    | None -> Error ("Not a number: " ^ s)
end

let () =
  match StringToInt.try_from "42" with
  | Ok n -> Printf.printf "Got: %d\n" n
  | Error e -> Printf.printf "Error: %s\n" e
```

### Rust — Trait-based conversion
```rust
use std::convert::TryFrom;

struct PositiveInt(u32);

impl TryFrom<i32> for PositiveInt {
    type Error = &'static str;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if n > 0 {
            Ok(PositiveInt(n as u32))
        } else {
            Err("must be positive")
        }
    }
}

fn main() {
    match PositiveInt::try_from(42) {
        Ok(p) => println!("Got: {}", p.0),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Infallible conversion | `Float.of_int`, `Int.to_string` | `From` trait, `.into()` |
| Fallible conversion | `*_opt` functions, custom modules | `TryFrom` trait, `.try_into()` |
| Generic bounds | Module functors | `T: Into<U>` trait bounds |
| Automatic impl | None | `Into` auto-derived from `From` |
| Error type | Part of result | Associated `type Error` |
| Primitive casts | Type-specific functions | `as` keyword |

---

## From vs Into

```rust
// Implementing From gives you Into for free
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Fahrenheit {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

// Now both work:
let f1 = Fahrenheit::from(Celsius(100.0));  // From
let f2: Fahrenheit = Celsius(100.0).into();  // Into (auto-derived)
```

The idiom: **implement `From`, use `Into` in bounds**.

```rust
// Generic function accepting anything convertible
fn process<T: Into<String>>(input: T) {
    let s: String = input.into();
    // ...
}

process("literal");        // &str -> String
process(String::from("x")); // String -> String (no-op)
```

---

## TryFrom vs TryInto

For fallible conversions:

```rust
impl TryFrom<&str> for Port {
    type Error = PortError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n: u16 = s.parse()?;
        if n >= 1024 { Ok(Port(n)) }
        else { Err(PortError::TooLow) }
    }
}

// Usage
let p: Result<Port, _> = "8080".try_into();
let p2 = Port::try_from("80");  // Err
```

---

## The `as` Keyword

Rust's `as` is for primitive casts only:

```rust
let a: i64 = 1000;
let b: i32 = a as i32;  // truncating (safe here)

let c: i32 = 300;
let d: u8 = c as u8;    // wrapping: 300 % 256 = 44 ⚠️

let e: f64 = 42i32 as f64;  // widening (safe)
```

**Warning**: `as` can silently truncate! Use `TryFrom` when overflow matters.

---

## 5 Takeaways

1. **Implement `From`, use `Into` in bounds.**
   `impl From<A> for B` gives you `impl Into<B> for A` automatically.

2. **`TryFrom`/`TryInto` are for fallible conversions.**
   Return `Result` with an associated error type.

3. **OCaml uses modules; Rust uses traits.**
   Same concept, different implementation patterns.

4. **`as` is only for primitives and can lose data.**
   It's fast but unchecked — use `TryFrom` for safety.

5. **Generic `Into` bounds make APIs flexible.**
   Accept `impl Into<String>` to take `&str`, `String`, etc.
