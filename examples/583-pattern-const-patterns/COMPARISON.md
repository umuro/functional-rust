# OCaml vs Rust: Constant Patterns

## Constants in Patterns

### OCaml
```ocaml
(* OCaml cannot use let-bound values directly in patterns *)
let max_age = 65  (* Cannot use this in match patterns *)

let classify n =
  match n with
  | 0 -> "zero"
  | n when n = 100 -> "century"  (* Must use guard *)
  | n when n < 100 -> "medium"
  | _ -> "large"
```

### Rust
```rust
const MIN_AGE: u32 = 18;
const MAX_AGE: u32 = 65;

fn classify_age(age: u32) -> &'static str {
    match age {
        0 => "newborn",
        1..=17 => "minor",
        MIN_AGE..=MAX_AGE => "adult",  // Constants work directly!
        _ => "senior",
    }
}
```

## Associated Constants

### Rust
```rust
struct Config;
impl Config {
    const TIMEOUT: u32 = 30;
}

fn classify_timeout(t: u32) -> &'static str {
    match t {
        0 => "none",
        Config::TIMEOUT => "default",  // Associated const in pattern
        _ => "other",
    }
}
```

### OCaml
```ocaml
(* No direct equivalent - use module values with guards *)
module Config = struct
  let timeout = 30
end

let classify_timeout t =
  match t with
  | 0 -> "none"
  | t when t = Config.timeout -> "default"
  | _ -> "other"
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Constants in patterns** | Not allowed | `const` values work |
| **Workaround** | `when` guards | Direct matching |
| **Associated constants** | Module values + guards | `impl` const in patterns |
| **Range with constants** | Not supported | `MIN..=MAX` works |

## Benefits of Rust's Approach

1. **Compile-time checking** - Invalid constant patterns caught early
2. **Exhaustiveness** - Works with range patterns
3. **Readability** - Named constants are self-documenting
4. **Maintainability** - Change constant once, patterns update
