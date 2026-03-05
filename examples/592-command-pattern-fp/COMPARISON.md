# OCaml vs Rust: Command Pattern (Functional)

## Command as Data

### OCaml
```ocaml
type cmd =
  | MoveTo  of float * float
  | LineTo  of float * float
  | SetColor of string
```

### Rust
```rust
enum DrawCmd {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    SetColor(String),
}
```

## Command Execution

### OCaml
```ocaml
let execute state cmd =
  match cmd with
  | MoveTo(x, y) -> state.x <- x; state.y <- y
  | LineTo(x, y) -> 
      Printf.printf "line -> (%f,%f)\n" x y;
      state.x <- x; state.y <- y
  | SetColor c -> state.color <- c
```

### Rust
```rust
fn execute(state: &mut DrawState, cmd: &DrawCmd) -> Option<String> {
    match cmd {
        DrawCmd::MoveTo(x, y) => {
            state.x = *x; state.y = *y;
            None
        }
        DrawCmd::LineTo(x, y) => {
            let log = format!("line -> ({},{})", x, y);
            state.x = *x; state.y = *y;
            Some(log)
        }
        DrawCmd::SetColor(c) => {
            state.color = c.clone();
            None
        }
    }
}
```

## Command Composition

Both languages can compose commands into sequences:

```rust
fn rect(x: f64, y: f64, w: f64, h: f64) -> Vec<DrawCmd> {
    vec![
        DrawCmd::MoveTo(x, y),
        DrawCmd::LineTo(x + w, y),
        DrawCmd::LineTo(x + w, y + h),
        DrawCmd::LineTo(x, y + h),
        DrawCmd::LineTo(x, y),
    ]
}
```

## Benefits

1. **Serializable** - Commands can be saved/transmitted
2. **Replayable** - Reconstruct state from command history
3. **Optimizable** - Remove redundant commands
4. **Testable** - Test command sequences in isolation
