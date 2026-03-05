# Comparison: Example 184 — Free Monad Introduction

## Free Monad Type

### OCaml
```ocaml
type 'next console_f =
  | Print of string * 'next
  | GetLine of (string -> 'next)

type 'a console =
  | CPure of 'a
  | CFree of 'a console console_f
```

### Rust
```rust
enum Console<A> {
    Pure(A),
    Print(String, Box<Console<A>>),
    GetLine(Box<dyn FnOnce(String) -> Console<A>>),
}
```

## Bind (FlatMap)

### OCaml
```ocaml
let rec bind m f = match m with
  | CPure a -> f a
  | CFree (Print (msg, next)) -> CFree (Print (msg, bind next f))
  | CFree (GetLine k) -> CFree (GetLine (fun s -> bind (k s) f))
```

### Rust
```rust
fn bind<A: 'static, B: 'static>(
    ma: Console<A>, f: impl FnOnce(A) -> Console<B> + 'static,
) -> Console<B> {
    match ma {
        Console::Pure(a) => f(a),
        Console::Print(msg, next) => Console::Print(msg, Box::new(bind(*next, f))),
        Console::GetLine(k) => Console::GetLine(Box::new(move |s| bind(k(s), f))),
    }
}
```

## Program Construction

### OCaml
```ocaml
let program =
  print "What is your name?" >>= fun () ->
  get_line () >>= fun name ->
  print ("Hello, " ^ name ^ "!") >>= fun () ->
  return_ name
```

### Rust
```rust
fn greet_program() -> Console<String> {
    bind(Console::print("What is your name?"), move |()| {
        bind(Console::get_line(), move |name: String| {
            bind(Console::print(&format!("Hello, {}!", name)), move |()| {
                Console::pure(name)
            })
        })
    })
}
```

## Pure Interpreter

### OCaml
```ocaml
let rec interpret_pure inputs prog = match prog with
  | CPure a -> ([], a)
  | CFree (Print (msg, next)) ->
    let (outputs, result) = interpret_pure inputs next in
    (msg :: outputs, result)
  | CFree (GetLine k) -> interpret_pure (List.tl inputs) (k (List.hd inputs))
```

### Rust
```rust
fn interpret_pure(inputs: &[&str], prog: Console<String>) -> (Vec<String>, String) {
    let mut current = prog;
    let mut outputs = Vec::new();
    let mut idx = 0;
    loop {
        match current {
            Console::Pure(a) => return (outputs, a),
            Console::Print(msg, next) => { outputs.push(msg); current = *next; }
            Console::GetLine(k) => { current = k(inputs[idx].into()); idx += 1; }
        }
    }
}
```
