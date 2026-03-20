📖 **[View on hightechmind.io →](https://hightechmind.io/rust/506-closure-move-semantics)**

---

# Closure Move Semantics

The `move` keyword forces a closure to take ownership of every variable it captures from the enclosing scope — required for thread spawning, returning closures from functions, and async tasks where the closure must outlive its creator.

## Problem Statement

When a closure is sent to another thread or returned from a function, it must own its captured data. The enclosing scope (and its stack frame) will be gone when the closure executes. `move || data.len()` moves `data` into the closure's environment at creation time. Without `move`, the borrow checker would reject the code because the borrowed reference would dangle. This is why `thread::spawn` requires `move` closures: the spawned thread may outlive the spawning thread's stack frame.

## Learning Outcomes

- Use `move` to transfer ownership of captured values into a closure
- Understand that `move` with `Copy` types copies the value (semantically the same as moving)
- Return a `move` closure that outlives its creating scope
- Clone a value before moving to retain a copy in the original scope
- Recognise `move` as mandatory for `thread::spawn` and `async` blocks

## Rust Application

Thread spawning requires `move` — `data` is moved into the closure:

```rust
pub fn spawn_with_data(data: Vec<i32>) -> thread::JoinHandle<i32> {
    thread::spawn(move || data.iter().sum())
}
```

Multiple values moved simultaneously:

```rust
pub fn move_multiple() -> impl FnOnce() -> (String, Vec<i32>) {
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    move || (s, v)  // both moved
}
```

Clone before move — keep the original, move the copy:

```rust
pub fn clone_then_move(s: String) -> (impl Fn() -> usize, String) {
    let cloned = s.clone();
    let f = move || cloned.len();
    (f, s)  // f owns cloned, caller retains s
}
```

## OCaml Approach

OCaml closures automatically capture variables by reference to GC-managed values — the GC prevents dangling:

```ocaml
let spawn_with_data data =
  let result = ref 0 in
  Domain.spawn (fun () -> result := List.fold_left (+) 0 data);
  result  (* data captured by reference; GC keeps it alive *)
```

In Multicore OCaml, domains capture the enclosing value by reference; the GC ensures safety. There is no `move` keyword because values never "move" — they stay on the heap and the GC manages their lifetime.

## Key Differences

1. **Explicit vs. implicit move**: Rust requires `move` to be explicit; OCaml always captures by GC reference (conceptually always "moved to the heap").
2. **`'static` bound on threads**: Rust's `thread::spawn` requires `F: 'static + Send` — `move` ensures `'static`; OCaml has no such bound.
3. **Clone discipline**: Rust's `clone_then_move` requires explicit `.clone()` to retain both a closure and the original; OCaml captures by shared reference automatically.
4. **`FnOnce` enforcement**: A `move` closure that consumes a non-`Copy` value can only be called once (`FnOnce`); OCaml has no type-level equivalent.

## Exercises

1. **Parallel map**: Write `fn parallel_map<T: Send + 'static, U: Send + 'static>(data: Vec<T>, f: impl Fn(T) -> U + Send + Sync + 'static) -> Vec<U>` using `thread::scope` or `Arc` + `move` closures.
2. **Move vs. clone benchmark**: Create a `Vec<String>` with 1000 elements and measure the time of `thread::spawn(move || ...)` vs. cloning the vec before spawn using `criterion`.
3. **Async simulation**: Write `fn make_async_task(data: String) -> impl FnOnce() -> String` returning a `move` closure that simulates a deferred computation — verify it can be called from a different scope.
