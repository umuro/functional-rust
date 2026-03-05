# Comparison: Associated Types

## Container with Associated Type

**OCaml:**
```ocaml
module type Container = sig
  type t
  type item
  val empty : t
  val push : item -> t -> t
  val pop : t -> (item * t) option
end

module Stack : Container with type item = int = struct
  type item = int
  type t = int list
  let empty = []
  let push x xs = x :: xs
  let pop = function [] -> None | x :: xs -> Some (x, xs)
end
```

**Rust:**
```rust
trait Container {
    type Item;
    fn empty() -> Self;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<T> Container for Stack<T> {
    type Item = T;
    fn empty() -> Self { Stack { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
}
```

## Transformer Pattern

**OCaml:**
```ocaml
module type Transformer = sig
  type input
  type output
  val transform : input -> output
end

module StringLen : Transformer with type input = string and type output = int = struct
  type input = string
  type output = int
  let transform = String.length
end
```

**Rust:**
```rust
trait Transformer {
    type Input;
    type Output;
    fn transform(&self, input: Self::Input) -> Self::Output;
}

impl Transformer for StringLength {
    type Input = String;
    type Output = usize;
    fn transform(&self, input: String) -> usize { input.len() }
}
```
