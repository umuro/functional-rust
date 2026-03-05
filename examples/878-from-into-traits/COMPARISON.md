# Comparison: From/Into Traits

## Infallible Conversion

**OCaml:**
```ocaml
let fahrenheit_of_celsius c = { f = c.c *. 9.0 /. 5.0 +. 32.0 }
let celsius_of_fahrenheit f = { c = (f.f -. 32.0) *. 5.0 /. 9.0 }
```

**Rust:**
```rust
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}
// Into<Celsius> for Fahrenheit comes free!
let c: Celsius = Fahrenheit(212.0).into();
```

## Fallible Conversion

**OCaml:**
```ocaml
let int_of_string_opt s =
  try Some (int_of_string s) with Failure _ -> None
```

**Rust:**
```rust
impl TryFrom<&str> for Point {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // parse "(x, y)" format
    }
}
let p = Point::try_from("(3, 4)")?;
```

## Generic Into Bounds

**OCaml:**
```ocaml
(* Must pass conversion function explicitly *)
let print_celsius convert temp =
  let c = convert temp in
  Printf.printf "%.1f°C" c.c
```

**Rust:**
```rust
fn print_temperature<T: Into<Celsius>>(temp: T) {
    let c: Celsius = temp.into();
    println!("Temperature: {}", c);
}
print_temperature(Fahrenheit(98.6));  // auto-converts
print_temperature(Celsius(37.0));     // identity
```
