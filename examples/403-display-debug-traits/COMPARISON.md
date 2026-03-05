# OCaml vs Rust: Display and Debug Traits

## Side-by-Side Code

### OCaml — Manual to_string functions
```ocaml
type color = Red | Green | Blue | Rgb of int * int * int

let string_of_color = function
  | Red -> "red"
  | Green -> "green"
  | Blue -> "blue"
  | Rgb (r, g, b) -> Printf.sprintf "rgb(%d,%d,%d)" r g b

let debug_color = function
  | Red -> "Color::Red"
  | Green -> "Color::Green"
  | Blue -> "Color::Blue"
  | Rgb (r, g, b) -> Printf.sprintf "Color::Rgb(%d, %d, %d)" r g b

let () =
  Printf.printf "display: %s  debug: %s\n"
    (string_of_color Red) (debug_color Red)
```

### Rust — Traits with automatic integration
```rust
use std::fmt;

#[derive(Debug)]  // Auto-generates Debug
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Rgb(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
        }
    }
}

fn main() {
    let c = Color::Red;
    println!("display: {}  debug: {:?}", c, c);
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| User-facing format | `string_of_*` function | `Display` trait, `{}` placeholder |
| Developer format | Separate `debug_*` function | `Debug` trait, `{:?}` placeholder |
| Auto-derivation | `[@@deriving show]` (ppx) | `#[derive(Debug)]` (built-in) |
| Integration | Manual function calls | Automatic via `println!`, `format!` |
| Pretty printing | Manual formatting | `{:#?}` for multi-line |
| Custom formatters | More functions | More traits: `LowerHex`, `Binary`, etc. |

---

## Format Specifiers

Rust's formatting system supports many specifiers:

```rust
let n = 255;
println!("{}", n);      // Display: 255
println!("{:?}", n);    // Debug: 255
println!("{:x}", n);    // LowerHex: ff
println!("{:X}", n);    // UpperHex: FF
println!("{:o}", n);    // Octal: 377
println!("{:b}", n);    // Binary: 11111111
println!("{:e}", 1.5);  // LowerExp: 1.5e0
```

OCaml uses Printf with different format strings:
```ocaml
Printf.printf "%d %x %o\n" 255 255 255
(* 255 ff 377 *)
```

---

## The Formatter Parameter

Rust's `fmt::Formatter` provides formatting options:

```rust
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // f.precision() — requested decimal places
        // f.width() — requested field width
        // f.fill() — padding character
        // f.align() — Left, Right, Center
        match f.precision() {
            Some(p) => write!(f, "({:.prec$}, {:.prec$})", self.x, self.y, prec = p),
            None => write!(f, "({:.2}, {:.2})", self.x, self.y),
        }
    }
}

// Now users can control precision:
println!("{:.4}", point);  // 4 decimal places
```

---

## 5 Takeaways

1. **`#[derive(Debug)]` is free and should be used almost everywhere.**
   It's a one-liner that enables `{:?}` printing.

2. **`Display` is for users, `Debug` is for developers.**
   `Display` should be human-readable; `Debug` should be useful for debugging.

3. **OCaml's approach is more explicit, less magical.**
   You write functions and call them. Rust uses trait dispatch.

4. **Rust's formatter traits integrate with `println!` and `format!`.**
   No need to call conversion functions — use format specifiers.

5. **Custom format specifiers are possible via additional traits.**
   `LowerHex`, `Binary`, `Octal`, etc. — implement what makes sense for your type.
