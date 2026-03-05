# OCaml vs Rust: Thread Basics

## Spawning Threads

### OCaml
```ocaml
let handle = Thread.create (fun () ->
  let r = 42 * 42 in
  Printf.printf "Result: %d\n%!" r;
  r
) ()
```

### Rust
```rust
let handle = thread::spawn(move || {
    let r = 42 * 42;
    println!("Result: {}", r);
    r
});
```

## Joining Threads

### OCaml
```ocaml
(* Thread.join returns unit, cannot get return value *)
Thread.join handle
```

### Rust
```rust
// JoinHandle::join returns Result<T, Box<dyn Any>>
let result: i32 = handle.join().unwrap();
```

## Multiple Threads

### OCaml
```ocaml
let handles = Array.init 4 (fun i ->
  Thread.create (fun () -> i * i) ()
) in
Array.iter Thread.join handles
(* Cannot collect return values directly *)
```

### Rust
```rust
let handles: Vec<_> = (0..4)
    .map(|i| thread::spawn(move || i * i))
    .collect();

let results: Vec<_> = handles
    .into_iter()
    .map(|h| h.join().unwrap())
    .collect();
// results = [0, 1, 4, 9]
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Spawn syntax | `Thread.create f arg` | `thread::spawn(move \|\| ...)` |
| Return value | `unit` only | Any `Send + 'static` type |
| Join result | `unit` | `Result<T, Box<dyn Any>>` |
| Panic handling | Crashes domain | `Err` returned to joiner |
| Data capture | GC managed | `move` closure with ownership |
| Thread safety | Runtime (GIL in some impls) | Compile-time (Send/Sync) |

## Panic Safety

### OCaml
```ocaml
(* Uncaught exception in thread propagates or crashes *)
let _ = Thread.create (fun () -> failwith "boom") ()
```

### Rust
```rust
// Panic is contained, parent thread can handle it
let h = thread::spawn(|| panic!("boom"));
match h.join() {
    Ok(v)  => println!("got {}", v),
    Err(_) => println!("thread panicked safely"),
}
```
