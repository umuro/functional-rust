# OCaml vs Rust: Option Combinators

## Pattern 1: Map Some Value

### OCaml
```ocaml
let some_5 = Some 5 in
let mapped = Option.map (fun x -> x * 2) some_5
(* Some 10 *)
```

### Rust
```rust
let doubled = Some(5).map(|x| x * 2);
// Some(10)
```

## Pattern 2: Chain Optional Operations

### OCaml
```ocaml
let safe_div x y = if y = 0 then None else Some (x / y) in
let result = Option.bind some_5 (fun n -> safe_div 10 n)
```

### Rust
```rust
fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}
let result = Some(5).and_then(|n| safe_div(10, n));
```

## Pattern 3: Filter by Predicate

### OCaml
```ocaml
let even = Option.filter (fun x -> x mod 2 = 0) (Some 6)
(* Some 6 *)
```

### Rust
```rust
let even = Some(6).filter(|&x| x % 2 == 0);
// Some(6)
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map Some | `Option.map f opt` | `opt.map(f)` |
| Chain optional | `Option.bind opt f` | `opt.and_then(f)` |
| Filter | `Option.filter pred opt` | `opt.filter(pred)` |
| Default value | `Option.value ~default opt` | `opt.unwrap_or(default)` |
| Lazy fallback | `Option.value_or_thunk` | `opt.unwrap_or_else(f)` |
