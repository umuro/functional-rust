# OCaml vs Rust: Scoped Threads

## Parallel Sum Pattern

### OCaml
```ocaml
let parallel_sum arr =
  let n = Array.length arr in
  let mid = n / 2 in
  let left  = ref 0 in
  let right = ref 0 in
  let t1 = Thread.create (fun () ->
    left := Array.fold_left (+) 0 (Array.sub arr 0 mid)) () in
  let t2 = Thread.create (fun () ->
    right := Array.fold_left (+) 0 (Array.sub arr mid (n-mid))) () in
  Thread.join t1; Thread.join t2;
  !left + !right
```

### Rust
```rust
fn parallel_sum(data: &[i64]) -> i64 {
    let (left, right) = data.split_at(data.len() / 2);
    let mut ls = 0i64;
    let mut rs = 0i64;
    
    thread::scope(|s| {
        let t1 = s.spawn(|| left.iter().sum::<i64>());
        let t2 = s.spawn(|| right.iter().sum::<i64>());
        ls = t1.join().unwrap();
        rs = t2.join().unwrap();
    }); // auto-join here
    
    ls + rs
}
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Data passing | Copy or ref (GC managed) | Direct borrow (`&[T]`) |
| Join guarantee | Manual — programmer must remember | Automatic at scope exit |
| Return values | Via `ref` cells | Direct from `join()` |
| Memory safety | GC prevents dangling | Scope lifetime proves safety |
| Zero-copy | Requires sub-array copy | `split_at` is zero-copy |

## Borrowing Local Variables

### OCaml
```ocaml
let message = "hello" in
let t = Thread.create (fun () ->
  Printf.printf "%s\n" message
) () in
Thread.join t
(* Works because GC tracks the string *)
```

### Rust
```rust
let message = String::from("hello");
thread::scope(|s| {
    s.spawn(|| println!("{}", message));  // borrows &message
    s.spawn(|| println!("len={}", message.len()));
});
// message still owned here — no move needed
```

## Mutable Access in Parallel

### OCaml
```ocaml
(* Requires mutex for mutable access *)
let arr = [|1;2;3;4;5;6|] in
let mutex = Mutex.create () in
(* Manual coordination needed *)
```

### Rust
```rust
let mut data = vec![1, 2, 3, 4, 5, 6];
let (left, right) = data.split_at_mut(3);

thread::scope(|s| {
    s.spawn(|| left.iter_mut().for_each(|x| *x *= 2));
    s.spawn(|| right.iter_mut().for_each(|x| *x *= 3));
});
// Compiler proves left and right don't overlap
```
