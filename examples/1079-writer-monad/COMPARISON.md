# OCaml vs Rust: Writer Monad — Logging Computation

## Side-by-Side Code

### OCaml
```ocaml
type 'a writer = { value: 'a; log: string list }

let return x = { value = x; log = [] }
let bind w f =
  let w' = f w.value in
  { value = w'.value; log = w.log @ w'.log }
let ( >>= ) = bind
let tell msg = { value = (); log = [msg] }

let half x =
  { value = x / 2; log = [Printf.sprintf "halved %d to %d" x (x / 2)] }

let compute x =
  return x >>= fun n ->
  half n >>= fun n ->
  tell (Printf.sprintf "result is %d" n) >>= fun () ->
  return n
```

### Rust (idiomatic)
```rust
pub struct Writer<A> {
    pub value: A,
    pub log: Vec<String>,
}

impl<A> Writer<A> {
    pub fn new(value: A) -> Self { Writer { value, log: Vec::new() } }

    pub fn bind<B, F>(self, f: F) -> Writer<B>
    where F: FnOnce(A) -> Writer<B> {
        let mut result = f(self.value);
        let mut combined = self.log;
        combined.append(&mut result.log);
        Writer { value: result.value, log: combined }
    }
}

pub fn compute(x: i64) -> Writer<i64> {
    Writer::new(x)
        .bind(half)
        .bind(|n| tell(format!("result is {n}")).map(|()| n))
}
```

### Rust (generic monoid)
```rust
pub trait Monoid: Default {
    fn combine(self, other: Self) -> Self;
}

pub struct GenericWriter<W, A> {
    pub value: A,
    pub log: W,
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Writer type | `type 'a writer = { value: 'a; log: string list }` | `struct Writer<A> { value: A, log: Vec<String> }` |
| Return/pure | `val return : 'a -> 'a writer` | `fn new(value: A) -> Writer<A>` |
| Bind | `val bind : 'a writer -> ('a -> 'b writer) -> 'b writer` | `fn bind<B>(self, f: FnOnce(A) -> Writer<B>) -> Writer<B>` |
| Tell | `val tell : string -> unit writer` | `fn tell(msg: impl Into<String>) -> Writer<()>` |

## Key Insights

1. **OCaml's `>>=` reads like a pipeline** — `return x >>= half >>= ...` flows left-to-right. Rust's `.bind(half).bind(...)` achieves the same with method chaining.
2. **Rust's `self` consumption is monadic by nature** — `bind(self, f)` takes ownership, which mirrors the monad law that each bind transforms the entire computation, not just the value.
3. **Log concatenation differs** — OCaml uses `@` (list append, O(n)). Rust uses `Vec::append` which is amortized O(1) because it moves the buffer pointer.
4. **The Monoid abstraction generalizes the pattern** — by parameterizing the log type with a `Monoid` trait, Rust can use `String`, `Vec<T>`, or any accumulator. OCaml achieves this with module functors.
5. **Both avoid side effects** — the log is part of the return value, not a mutable global. This makes the computation pure and testable.

## When to Use Each Style

**Use Writer monad when:** You need structured, composable logging that's part of the return type — audit trails, computation traces, query plan explanations.
**Use simple method chaining when:** You just need basic logging and the full monadic abstraction is overkill — Rust's `log` crate or `tracing` is often simpler for real applications.
