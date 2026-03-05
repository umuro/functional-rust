# OCaml vs Rust: Thread Pool Pattern

## Thread Pool Creation

### OCaml
```ocaml
let make_pool n =
  let q = Queue.create () in
  let m = Mutex.create () in
  let c = Condition.create () in
  let stop = ref false in
  let workers = Array.init n (fun _ ->
    Thread.create (fun () ->
      while not !stop do
        Mutex.lock m;
        while Queue.is_empty q && not !stop do
          Condition.wait c m
        done;
        if not (Queue.is_empty q) then
          let f = Queue.pop q in
          Mutex.unlock m; f ()
        else Mutex.unlock m
      done) ()
  ) in
  (* returns submit and shutdown functions *)
```

### Rust
```rust
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let workers = (0..size).map(|_| {
            let rx = Arc::clone(&receiver);
            thread::spawn(move || loop {
                match rx.lock().unwrap().recv() {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            })
        }).collect();
        
        ThreadPool { workers, sender: Some(sender) }
    }
}
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Job type | `unit -> unit` | `Box<dyn FnOnce() + Send + 'static>` |
| Shutdown | Manual `stop` flag + broadcast | Drop sender → recv returns Err |
| Thread safety | Mutex + Condition | MPSC channel + Arc |
| Cleanup | Manual join | `Drop` trait implementation |

## Job Submission

### OCaml
```ocaml
let submit f =
  Mutex.lock m;
  Queue.push f q;
  Condition.signal c;
  Mutex.unlock m
```

### Rust
```rust
pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
    self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
}
```

## Graceful Shutdown

### OCaml
```ocaml
let shutdown () =
  Mutex.lock m;
  stop := true;
  Condition.broadcast c;
  Mutex.unlock m;
  Array.iter Thread.join workers
```

### Rust
```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());  // Close channel
        for w in self.workers.drain(..) {
            w.join().unwrap();
        }
    }
}
```
