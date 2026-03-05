# OCaml vs Rust: Command Dispatch

## Command Type

### OCaml
```ocaml
type cmd =
  | Set    of string * int
  | Remove of string
  | Incr   of string * int
  | Clear
```

### Rust
```rust
enum Cmd {
    Set(String, i64),
    Remove(String),
    Increment(String, i64),
    Clear,
}
```

## Command Execution

### OCaml (Pure, Immutable)
```ocaml
let execute store = function
  | Set(k, v)  -> (k, v) :: List.filter (fun (k', _) -> k' <> k) store
  | Remove k   -> List.filter (fun (k', _) -> k' <> k) store
  | Incr(k, d) -> (* update or insert *)
  | Clear      -> []
```

### Rust (Mutable with History)
```rust
impl Store {
    fn execute(&mut self, cmd: Cmd) {
        match &cmd {
            Cmd::Set(k, v) => { self.data.insert(k.clone(), *v); }
            Cmd::Remove(k) => { self.data.remove(k); }
            Cmd::Increment(k, d) => {
                *self.data.entry(k.clone()).or_default() += d;
            }
            Cmd::Clear => { self.data.clear(); }
        }
        self.history.push(cmd);
    }
}
```

## Pure Replay

Both support replaying commands:

### OCaml
```ocaml
let final_state = List.fold_left execute [] commands
```

### Rust
```rust
let final_state = commands.iter().fold(HashMap::new(), |acc, c| apply(acc, c));
```

## Benefits of Command Pattern

1. **Serializable** - Commands can be stored/transmitted
2. **Replayable** - Reconstruct state from history
3. **Undoable** - Can implement inverse commands
4. **Auditable** - Full command history for debugging
