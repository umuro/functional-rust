# OCaml vs Rust: Bounded Channels

## Bounded Channel Creation

### OCaml (Manual implementation)
```ocaml
let make_bounded capacity =
  let queue = Queue.create () in
  let mutex = Mutex.create () in
  let not_full = Condition.create () in
  let not_empty = Condition.create () in
  
  let send v =
    Mutex.lock mutex;
    while Queue.length queue >= capacity do
      Condition.wait not_full mutex
    done;
    Queue.push v queue;
    Condition.signal not_empty;
    Mutex.unlock mutex
  in
  
  let recv () =
    Mutex.lock mutex;
    while Queue.is_empty queue do
      Condition.wait not_empty mutex
    done;
    let v = Queue.pop queue in
    Condition.signal not_full;
    Mutex.unlock mutex;
    v
  in
  (send, recv)
```

### Rust
```rust
let (tx, rx) = mpsc::sync_channel::<u32>(capacity);

// Send blocks when full
tx.send(value).unwrap();

// Receive blocks when empty
let value = rx.recv().unwrap();
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Bounded channel | Manual | `sync_channel(cap)` |
| Backpressure | Manual Condvar | Built-in blocking |
| Try operations | Manual | `try_send`, `try_recv` |
| Multi-consumer | Shared queue | `Arc<Mutex<Receiver>>` |

## Multi-Consumer Pattern

### OCaml
```ocaml
(* Same queue shared by multiple threads *)
let consumers = Array.init num_consumers (fun id ->
  Thread.create (fun () ->
    (* Each thread calls recv on shared channel *)
  ) ()
)
```

### Rust
```rust
let (tx, rx) = mpsc::channel::<u32>();
let rx = Arc::new(Mutex::new(rx));  // Wrap for sharing

let consumers: Vec<_> = (0..num_consumers).map(|_| {
    let rx = Arc::clone(&rx);
    thread::spawn(move || loop {
        match rx.lock().unwrap().recv() {
            Ok(v) => process(v),
            Err(_) => break,  // Channel closed
        }
    })
}).collect();
```

## Non-blocking Operations

### Rust
```rust
// Try to send without blocking
match tx.try_send(value) {
    Ok(()) => println!("sent"),
    Err(TrySendError::Full(v)) => println!("channel full"),
    Err(TrySendError::Disconnected(v)) => println!("closed"),
}

// Try to receive without blocking
match rx.try_recv() {
    Ok(v) => println!("got {}", v),
    Err(TryRecvError::Empty) => println!("nothing yet"),
    Err(TryRecvError::Disconnected) => println!("closed"),
}
```

## With Crossbeam (Real library)

```rust
use crossbeam_channel::{bounded, select};

let (tx, rx) = bounded::<i32>(10);

// Select from multiple channels
select! {
    recv(rx1) -> msg => println!("from rx1: {:?}", msg),
    recv(rx2) -> msg => println!("from rx2: {:?}", msg),
    default => println!("nothing ready"),
}
```
