# OCaml vs Rust: Tuple Pattern Matching

## FizzBuzz Example

### OCaml
```ocaml
let fizzbuzz n = match (n mod 3 = 0, n mod 5 = 0) with
  | (true, true)   -> "FizzBuzz"
  | (true, false)  -> "Fizz"
  | (false, true)  -> "Buzz"
  | (false, false) -> string_of_int n
```

### Rust
```rust
fn fizzbuzz(n: u32) -> String {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true)   => "FizzBuzz".into(),
        (true, false)  => "Fizz".into(),
        (false, true)  => "Buzz".into(),
        (false, false) => n.to_string(),
    }
}
```

## State Machine with Tuple

### OCaml
```ocaml
type light = Red | Yellow | Green

let next (l, emergency) = match (l, emergency) with
  | (_, true)        -> Red
  | (Red, false)     -> Green
  | (Green, false)   -> Yellow
  | (Yellow, false)  -> Red
```

### Rust
```rust
fn next_light(light: Light, emergency: bool) -> Light {
    match (light, emergency) {
        (_, true)             => Light::Red,
        (Light::Red, false)   => Light::Green,
        (Light::Green, false) => Light::Yellow,
        (Light::Yellow, false)=> Light::Red,
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Syntax** | `match (a, b) with` | `match (a, b) { }` |
| **Wildcard** | `_` | `_` |
| **Arrow** | `->` | `=>` |
| **Pattern guards** | `when condition` | `if condition` |

## Benefits of Tuple Matching

1. **Multi-value decisions** - Match on combinations without nested if/else
2. **State machines** - Clear representation of (state, input) → new_state
3. **Exhaustiveness** - Compiler ensures all combinations are handled
4. **Readability** - Truth table style patterns
