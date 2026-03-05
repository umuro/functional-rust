# OCaml vs Rust: MPSC Channels

## Channel Creation

### OCaml (Manual with Queue + Mutex + Condition)
```ocaml
let queue = Queue.create ()
let mutex = Mutex.create ()
let cond = Condition.create ()

let send v =
  Mutex.lock mutex;
  Queue.push v queue;
  Condition.signal cond;
  Mutex.unlock mutex

let recv () =
  Mutex.lock mutex;
  while Queue.is_empty queue do
    Condition.wait cond mutex
  done;
  let v = Queue.pop queue in
  Mutex.unlock mutex;
  v
```

### Rust
```rust
let (tx, rx) = mpsc::channel::<String>();

// Send
tx.send("message".into()).unwrap();

// Receive (blocking)
let msg = rx.recv().unwrap();
```

## Multiple Producers

### OCaml
```ocaml
(* Same send function works from multiple threads *)
let producers = List.init 3 (fun id ->
  Thread.create (fun () ->
    for i = 1 to 5 do
      send (Printf.sprintf "p%d-msg%d" id i)
    done
  ) ()
)
```

### Rust
```rust
let handles: Vec<_> = (0..3).map(|id| {
    let tx = tx.clone();  // Clone the sender
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("p{}-msg{}", id, i)).unwrap();
        }
    })
}).collect();

drop(tx);  // Drop original to close channel
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Built-in channel | No (manual) | Yes (`std::sync::mpsc`) |
| Sender cloning | Same function | `tx.clone()` |
| Channel close | Sentinel value | Drop all senders |
| Shutdown signal | Manual | Automatic (`recv()` → `Err`) |
| Bounded channel | Manual size check | `sync_channel(size)` |

## Consumer Iteration

### OCaml
```ocaml
(* Must know message count or use sentinel *)
let consumer = Thread.create (fun () ->
  for _ = 1 to 15 do
    Printf.printf "got: %s\n%!" (recv ())
  done
) ()
```

### Rust
```rust
// Iterate until channel closes
for msg in rx {
    println!("got: {}", msg);
}
// Loop exits when all senders drop
```

## Non-blocking Operations

### OCaml
```ocaml
(* Manual try with immediate check *)
let try_recv () =
  Mutex.lock mutex;
  let result =
    if Queue.is_empty queue then None
    else Some (Queue.pop queue)
  in
  Mutex.unlock mutex;
  result
```

### Rust
```rust
// try_recv returns immediately
match rx.try_recv() {
    Ok(msg) => println!("got {}", msg),
    Err(TryRecvError::Empty) => println!("no message"),
    Err(TryRecvError::Disconnected) => println!("closed"),
}

// try_iter drains all available
let all: Vec<_> = rx.try_iter().collect();
```

## Timeout Receive (Rust-specific)

```rust
match rx.recv_timeout(Duration::from_secs(1)) {
    Ok(msg) => process(msg),
    Err(RecvTimeoutError::Timeout) => handle_timeout(),
    Err(RecvTimeoutError::Disconnected) => break,
}
```
