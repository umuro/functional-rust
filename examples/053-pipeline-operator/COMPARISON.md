# Comparison: Pipeline Operator

## OCaml — custom `|>` operator

```ocaml
let ( |> ) x f = f x
let result  = 5       |> double |> add1    (* 11   *)
let greeting = "hello" |> shout  |> exclaim (* HELLO! *)
```

## Rust — nested calls (equivalent semantics)

```rust
let result   = add1(double(5));             // 11
let greeting = exclaim(shout("hello"));     // HELLO!
```

## Rust — trait-based pipe

```rust
pub trait Pipe: Sized {
    fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B { f(self) }
}
impl<T> Pipe for T {}

let result   = 5.pipe(double).pipe(add1);
let greeting = "hello".pipe(shout).pipe(exclaim);
```

## Rust — macro pipe

```rust
macro_rules! pipe {
    ($val:expr => $($f:expr),+ $(,)?) => {{
        let mut v = $val;
        $(v = $f(v);)+
        v
    }};
}

let result = pipe!(5 => double, add1);           // 11
let chained = pipe!(5 => double, add1, double);  // 22
```

## Comparison Table

| Aspect | OCaml | Rust (trait) | Rust (macro) |
|--------|-------|-------------|-------------|
| Syntax | `x \|> f \|> g` | `x.pipe(f).pipe(g)` | `pipe!(x => f, g)` |
| Direction | Left-to-right | Left-to-right | Left-to-right |
| Operator | Custom infix `\|>` | Method call | Macro invocation |
| Generics | Polymorphic `'a` | `T: Sized` bound | Any `Expr` |
| Runtime cost | None (inlined) | None (inlined) | None (expanded) |

## Type Signatures

- OCaml `|>`: `val ( |> ) : 'a -> ('a -> 'b) -> 'b`
- Rust `Pipe::pipe`: `fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B`

## Takeaways

1. `|>` is just reverse application — `x |> f` is `f(x)` — a trivial but powerful idea
2. The `Pipe` trait adds `.pipe()` to every type with a single `impl<T> Pipe for T {}` blanket impl
3. Rust's blanket impl idiom is the idiomatic equivalent of OCaml's operator definition
4. The macro makes multi-step pipelines read like a pipeline: `pipe!(x => f, g, h)`
5. All three Rust approaches compile to identical code — pick based on readability preference
