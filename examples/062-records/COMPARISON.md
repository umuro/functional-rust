## Core Insight

Records/structs group named fields. Both languages support pattern matching on fields and functional update syntax (creating a new value with some fields changed).

## OCaml Approach
- `type t = { field1: type1; field2: type2 }` — record definition
- `{ r with field = new_value }` — functional update
- Mutable fields with `mutable` keyword (rare)
- Pattern match: `let { field1; field2 } = r`

## Rust Approach
- `struct T { field1: Type1, field2: Type2 }` — struct definition
- `T { field: new_val, ..old }` — struct update syntax (moves non-Copy fields!)
- All fields private by default; `pub` for visibility
- Destructuring: `let T { field1, field2 } = s;`

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Define | `type t = { x: int }` | `struct T { x: i32 }` |
| Create | `{ x = 5 }` | `T { x: 5 }` |
| Access | `r.x` | `s.x` |
| Update | `{ r with x = 10 }` | `T { x: 10, ..s }` |
| Destructure | `let { x; y } = r` | `let T { x, y } = s` |
| Mutability | `mutable` per field | `let mut s` (all or nothing) |
