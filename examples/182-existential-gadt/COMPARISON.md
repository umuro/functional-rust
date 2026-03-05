# Comparison: Example 182 — Existential Types

## Basic Existential

### OCaml
```ocaml
type showable = Show : 'a * ('a -> string) -> showable

let show (Show (x, f)) = f x

let items = [Show (42, string_of_int); Show ("hello", Fun.id)]
let results = List.map show items
```

### Rust
```rust
let items: Vec<Box<dyn fmt::Display>> = vec![Box::new(42), Box::new("hello")];
let results: Vec<String> = items.iter().map(|x| format!("{}", x)).collect();
```

## Closure-Based Existential

### OCaml
```ocaml
type showable = Show : 'a * ('a -> string) -> showable
```

### Rust
```rust
struct Showable {
    show_fn: Box<dyn Fn() -> String>,
}

impl Showable {
    fn new<T: 'static>(value: T, to_string: fn(&T) -> String) -> Self {
        Showable { show_fn: Box::new(move || to_string(&value)) }
    }
}
```

## First-Class Module vs Super-Trait

### OCaml
```ocaml
module type PRINTABLE = sig
  type t
  val value : t
  val to_string : t -> string
end

let print_it (m : (module PRINTABLE)) =
  let module M = (val m) in M.to_string M.value
```

### Rust
```rust
trait Printable: fmt::Display + fmt::Debug {}
impl<T: fmt::Display + fmt::Debug> Printable for T {}

fn print_all(items: &[Box<dyn Printable>]) -> Vec<String> {
    items.iter().map(|x| format!("{}", x)).collect()
}
```
