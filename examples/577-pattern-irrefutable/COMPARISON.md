# OCaml vs Rust: Irrefutable vs Refutable Patterns

## Irrefutable Patterns (Always Match)

### OCaml
```ocaml
(* Tuple destructuring - always succeeds *)
let (a, b) = (1, 2)

(* Function parameter *)
let add (x, y) = x + y
```

### Rust
```rust
// Tuple destructuring - always succeeds
let (a, b) = (1, 2);

// Function parameter
fn add((x, y): (i32, i32)) -> i32 { x + y }

// Struct destructuring
let Point { x, y } = point;

// For loop destructuring
for (n, ch) in &pairs { ... }
```

## Refutable Patterns (Might Fail)

### OCaml
```ocaml
(* Must use match for refutable patterns *)
let extract opt =
  match opt with
  | Some v -> Some (v * 2)
  | None -> None

(* Warning if you try: let Some v = opt *)
```

### Rust
```rust
// if let for refutable patterns
if let Some(v) = opt {
    Some(v * 2)
} else {
    None
}

// while let for iterating until pattern fails
while let Some(v) = stack.pop() {
    sum += v;
}

// Compile error if you try: let Some(v) = opt;
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Irrefutable `let`** | `let (a, b) = tuple` | `let (a, b) = tuple` |
| **Refutable handling** | `match` required | `if let`, `while let`, or `match` |
| **Refutable in `let`** | Warning + may panic | Compile error |
| **For loop** | `List.iter (fun (a, b) -> ...)` | `for (a, b) in pairs` |
| **Function params** | `fun (a, b) -> ...` | `fn f((a, b): (T, U))` |

## Why the Distinction Matters

**Irrefutable patterns** guarantee success - safe for `let`, `for`, and function parameters.

**Refutable patterns** might fail - need syntax that handles the failure case (`if let`, `while let`, `match`).

This is enforced at compile time in Rust, preventing runtime surprises.
