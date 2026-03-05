# Comparison: Example 169 — Operator Precedence

## Operator table

**OCaml:**
```ocaml
type assoc = Left | Right
type op_info = { symbol: string; precedence: int; associativity: assoc }

let operators = [
  { symbol = "+"; precedence = 5; associativity = Left };
  { symbol = "^"; precedence = 7; associativity = Right };
]
```

**Rust:**
```rust
#[derive(Clone, Copy)]
enum Assoc { Left, Right }

struct OpInfo { symbol: &'static str, precedence: u8, associativity: Assoc }

const OPERATORS: &[OpInfo] = &[
    OpInfo { symbol: "+", precedence: 5, associativity: Assoc::Left },
    OpInfo { symbol: "^", precedence: 7, associativity: Assoc::Right },
];
```

## Binding power from table

**OCaml:**
```ocaml
let binding_power op_info =
  let base = op_info.precedence * 2 in
  match op_info.associativity with
  | Left -> (base, base + 1)
  | Right -> (base + 1, base)
```

**Rust:**
```rust
fn binding_power(op: &OpInfo) -> (u8, u8) {
    let base = op.precedence * 2;
    match op.associativity {
        Assoc::Left => (base, base + 1),
        Assoc::Right => (base + 1, base),
    }
}
```
