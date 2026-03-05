# OCaml vs Rust: Executor Basics

## Task Queue

**OCaml (Lwt):**
```ocaml
(* Implicit scheduler in Lwt *)
Lwt_main.run (task1 >>= fun () -> task2)
```

**Rust:**
```rust
let ex = SimpleExecutor::new();
ex.spawn(async { task1() });
ex.spawn(async { task2() });
ex.run();
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Executor | Implicit (Lwt scheduler) | Explicit `run()` call |
| Task queue | Internal to Lwt | `mpsc::sync_channel` |
| Waker mechanism | Lwt callbacks | Manual `Waker` vtable |
| Spawn | `Lwt.async` | `executor.spawn()` |
