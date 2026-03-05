# Thread-Local Storage

## OCaml
```ocaml
(* No built-in TLS; use Domain.DLS *)
let key = Domain.DLS.new_key (fun () -> ref 0)
let () = Domain.DLS.get key := 42
```

## Rust
```rust
thread_local! {
    static COUNTER: Cell<usize> = Cell::new(0);
}

COUNTER.with(|c| {
    c.set(c.get() + 1);
});
```

## Key Differences
| Feature | OCaml | Rust |
|---------|-------|------|
| Syntax | `Domain.DLS` | `thread_local!` macro |
| Type | `'a key` | Static with Cell/RefCell |
| Cleanup | On domain exit | On thread exit |
