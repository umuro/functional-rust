# OCaml vs Rust: Closure State Machines

## OCaml
```ocaml
type state_result = Accept | Reject | Continue of (char -> state_result)

let rec state_start c = match c with
  | 'a' -> Continue state_after_a
  | 'b' -> Continue state_after_b
  | _ -> Reject
```

## Rust
```rust
pub enum StateResult {
    Accept,
    Reject,
    Continue(Box<dyn Fn(char) -> StateResult>),
}

pub fn state_start(c: char) -> StateResult {
    match c {
        'a' => StateResult::Continue(Box::new(state_after_a)),
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _ => StateResult::Reject,
    }
}
```

## Key Differences

1. **OCaml**: Functions stored directly in variants
2. **Rust**: Need Box<dyn Fn> for dynamic dispatch
3. Both: State transitions as function returns
4. **Rust**: Enum-based approach often more idiomatic
5. Both support modeling complex state machines functionally
