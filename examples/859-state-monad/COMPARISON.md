# Comparison: State Monad

## State Type

**OCaml:**
```ocaml
type ('s, 'a) state = State of ('s -> 'a * 's)
let run_state (State f) s = f s
```

**Rust:**
```rust
struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}
```

## Bind / and_then

**OCaml:**
```ocaml
let bind m f = State (fun s ->
  let (a, s') = run_state m s in
  run_state (f a) s')
```

**Rust:**
```rust
fn and_then<B>(self, f: impl FnOnce(A) -> State<S, B> + 'static) -> State<S, B> {
    State::new(move |s| {
        let (a, s2) = self.run(s);
        f(a).run(s2)
    })
}
```

## Counter Example

**OCaml:**
```ocaml
let tick = get >>= fun n -> put (n + 1) >>= fun () -> return_ n
let (result, _) = run_state (tick >>= fun a -> tick >>= fun b -> return_ (a, b)) 0
(* result = (0, 1) *)
```

**Rust (idiomatic — no monad needed):**
```rust
fn count3_explicit(state: i32) -> ((i32, i32, i32), i32) {
    let a = state; let state = state + 1;
    let b = state; let state = state + 1;
    let c = state; let state = state + 1;
    ((a, b, c), state)
}
```
