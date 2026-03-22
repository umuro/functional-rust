# OCaml vs Rust: Stack Module with Signature

## Side-by-Side Code

### OCaml

```ocaml
module type STACK = sig
  type 'a t
  exception Empty
  val empty    : 'a t
  val is_empty : 'a t -> bool
  val push     : 'a -> 'a t -> 'a t
  val peek     : 'a t -> 'a      (* raises Empty *)
  val pop      : 'a t -> 'a t    (* raises Empty *)
  val size     : 'a t -> int
end

module ListStack : STACK = struct
  type 'a t = 'a list
  exception Empty
  let empty = []
  let push x s = x :: s
  let peek = function [] -> raise Empty | x :: _ -> x
  let pop  = function [] -> raise Empty | _ :: s  -> s
end

(* Usage: pipeline operator with push *)
let s = ListStack.(empty |> push 1 |> push 2 |> push 3)
```

### Rust (idiomatic)

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn empty() -> Self { Stack { items: Vec::new() } }

    /// Push consumes self and returns the new stack (persistent-style API)
    pub fn push(mut self, x: T) -> Self {
        self.items.push(x);
        self
    }

    pub fn peek(&self) -> Result<&T, Empty> {
        self.items.last().ok_or(Empty)
    }

    pub fn pop(mut self) -> Result<Self, Empty> {
        if self.items.is_empty() { Err(Empty) }
        else { self.items.pop(); Ok(self) }
    }
}
```

### Rust (functional/recursive)

```rust
// Recursive push on an immutable-style linked-list stack
pub enum ListStack<T> {
    Nil,
    Cons(T, Box<ListStack<T>>),
}

impl<T> ListStack<T> {
    pub fn empty() -> Self { ListStack::Nil }
    pub fn push(self, x: T) -> Self { ListStack::Cons(x, Box::new(self)) }
    pub fn peek(&self) -> Option<&T> {
        match self { ListStack::Cons(x, _) => Some(x), _ => None }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| stack type | `module type STACK with type 'a t` | `struct Stack<T>` |
| empty stack | `val empty : 'a t` (value) | `fn empty() -> Self` (constructor method) |
| push | `val push : 'a -> 'a t -> 'a t` | `fn push(self, x: T) -> Self` (consuming) |
| peek | `val peek : 'a t -> 'a` (raises `Empty`) | `fn peek(&self) -> Result<&T, Empty>` |
| pop | `val pop : 'a t -> 'a t` (raises `Empty`) | `fn pop(self) -> Result<Self, Empty>` |
| empty check | `val is_empty : 'a t -> bool` | `fn is_empty(&self) -> bool` |

## Key Insights

1. **Error handling:** OCaml raises `Empty` as an exception that propagates up the call stack unless caught with `try ... with Empty -> ...`. Rust returns `Result<_, Empty>`, requiring callers to handle the error case at every call site. The Rust approach makes the failure branch visible in every function signature.
2. **Module signatures vs. types:** OCaml uses a module signature (`module type STACK`) to separate interface from implementation — callers cannot observe that the backing type is a list. Rust achieves the same encapsulation by making `items: Vec<T>` private — callers can't access the internal `Vec` directly.
3. **Consuming `self` for persistent-style API:** OCaml's `push x s` returns a new stack value; the original `s` is still accessible (OCaml lists are persistent). Rust simulates this by consuming `self` in `push(mut self, x: T) -> Self` — once pushed, the old binding is moved and only the new stack is accessible. In practice, the inner `Vec` is mutated in place before being returned, which is an invisible optimization.
4. **Pipeline operator vs. method chaining:** OCaml: `empty |> push 1 |> push 2 |> push 3`. Rust: `Stack::empty().push(1).push(2).push(3)`. Method chaining in Rust reads left-to-right and is syntactically equivalent.
5. **Linked-list alternative:** A `Box`-linked list stack (`Cons(T, Box<ListStack<T>>)`) is the most direct OCaml analog in Rust, but it allocates one heap node per element. The `Vec`-backed stack amortizes allocations and is idiomatic for production code.

## When to Use Each Style

**Use `Vec`-backed `Stack<T>` when:** Building a real stack for production use — amortized O(1) push/pop with contiguous memory is significantly faster than a linked list.
**Use `ListStack<T>` (linked list) when:** Teaching the OCaml parallel explicitly, or when you need true structural sharing between stack versions (e.g., sharing a common suffix between two stacks built from the same base).
