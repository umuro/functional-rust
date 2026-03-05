# Comparison: Writer Monad

## Writer Type

**OCaml:**
```ocaml
type ('w, 'a) writer = Writer of ('a * 'w)
let tell w = Writer ((), [w])
let return_ x = Writer (x, [])
```

**Rust:**
```rust
struct Writer<A> { value: A, log: Vec<String> }
impl<A> Writer<A> {
    fn pure(a: A) -> Self { Writer { value: a, log: vec![] } }
    fn tell(msg: String) -> Writer<()> { Writer { value: (), log: vec![msg] } }
}
```

## Bind (Log Accumulation)

**OCaml:**
```ocaml
let bind (Writer (a, w1)) f =
  let Writer (b, w2) = f a in
  Writer (b, w1 @ w2)    (* list append *)
```

**Rust:**
```rust
fn and_then<B>(self, f: impl FnOnce(A) -> Writer<B>) -> Writer<B> {
    let w2 = f(self.value);
    let mut log = self.log;
    log.extend(w2.log);    // vec extend
    Writer { value: w2.value, log }
}
```

## Logged Computation

**OCaml:**
```ocaml
add_with_log 3 4 >>= fun sum ->
multiply_with_log sum 2 >>= fun product ->
tell "Done!" >>= fun () ->
return_ product
```

**Rust:**
```rust
add_with_log(3, 4)
    .and_then(|sum| multiply_with_log(sum, 2))
    .and_then(|product| Writer::tell("Done!".into()).map(move |()| product))
```
