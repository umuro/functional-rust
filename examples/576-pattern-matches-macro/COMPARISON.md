# OCaml vs Rust: Pattern Matching to Boolean

## Basic Pattern Test

### OCaml
```ocaml
type status = Active | Inactive | Pending | Banned

let is_active = function Active -> true | _ -> false
let is_usable = function Active | Pending -> true | _ -> false
```

### Rust
```rust
#[derive(Debug)]
enum Status { Active, Inactive, Pending, Banned }

fn is_active(s: &Status) -> bool {
    matches!(s, Status::Active)
}

fn is_usable(s: &Status) -> bool {
    matches!(s, Status::Active | Status::Pending)
}
```

## In Filter Predicates

### OCaml
```ocaml
let active_count = 
    users |> List.filter (function Active -> true | _ -> false) |> List.length
```

### Rust
```rust
let active_count = users.iter()
    .filter(|u| matches!(u, Status::Active))
    .count();
```

## With Guards

### OCaml
```ocaml
let is_even_small x = match x with
  | n when n mod 2 = 0 && n <= 6 -> true
  | _ -> false
```

### Rust
```rust
fn is_even_small(x: i32) -> bool {
    matches!(x, n if n % 2 == 0 && n <= 6)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Syntax** | `function P -> true \| _ -> false` | `matches!(val, P)` |
| **Verbosity** | Requires explicit true/false arms | Single macro call |
| **OR patterns** | `P1 \| P2 -> true` | `matches!(v, P1 \| P2)` |
| **Guards** | `when condition` | `if condition` |
| **Common in** | Less common (full match preferred) | Very common in filter/assert |

## Macro Expansion

`matches!(x, Pattern)` expands to:
```rust
match x {
    Pattern => true,
    _ => false,
}
```

No runtime overhead — purely syntactic convenience.
