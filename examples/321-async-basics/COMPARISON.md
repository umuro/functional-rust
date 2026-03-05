# OCaml vs Rust: Async Basics

## Sequential Fetch

**OCaml:**
```ocaml
let fetch_user id =
  Thread.delay 0.05;
  Printf.sprintf "User(%d)" id

let () =
  let user = fetch_user 42 in
  let posts = fetch_posts 42 in
  (* Sequential: ~80ms total *)
```

**Rust:**
```rust
fn sequential_fetch(id: u32) -> (String, Vec<String>) {
    (fetch_user(id), fetch_posts(id))
    // Sequential: ~18ms total
}
```

## Concurrent Fetch

**OCaml (with threads):**
```ocaml
let parallel tasks =
  let threads = List.map (fun f -> Thread.create f ()) tasks in
  List.iter Thread.join threads
```

**Rust:**
```rust
fn concurrent_fetch(id: u32) -> (String, Vec<String>) {
    let h1 = thread::spawn(move || fetch_user(id));
    let h2 = thread::spawn(move || fetch_posts(id));
    (h1.join().unwrap(), h2.join().unwrap())
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Native async | No (use Lwt/Async) | Yes (`async`/`await`) |
| Thread API | `Thread.create` | `thread::spawn` |
| Move semantics | Implicit | Explicit `move` |
| Error handling | Exceptions | `Result` from `join()` |
| Concurrency model | GIL limits parallelism | True parallelism |
