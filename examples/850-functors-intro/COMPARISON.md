# Comparison: Functors Introduction

## Defining a Functor Interface

**OCaml:**
```ocaml
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end
```

**Rust:**
```rust
trait Functor {
    type Inner;
    type Mapped<U>: Functor;
    fn fmap<U, F: FnOnce(Self::Inner) -> U>(self, f: F) -> Self::Mapped<U>;
}
```

## Implementing Map for a Custom Type

**OCaml:**
```ocaml
type 'a maybe = Nothing | Just of 'a

let map f = function
  | Nothing -> Nothing
  | Just x -> Just (f x)
```

**Rust:**
```rust
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(f(x)),
        }
    }
}
```

## Chaining Maps

**OCaml:**
```ocaml
Just 3 |> map (fun x -> x + 1) |> map (fun x -> x * 2)
(* = Just 8 *)
```

**Rust:**
```rust
Maybe::Just(3)
    .map(|x| x + 1)
    .map(|x| x * 2)
// = Just(8)
```

## Tree Functor

**OCaml:**
```ocaml
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec tree_map f = function
  | Leaf -> Leaf
  | Node (l, v, r) -> Node (tree_map f l, f v, tree_map f r)
```

**Rust:**
```rust
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),  // Box needed for recursive type
}

impl<T> Tree<T> {
    fn map<U, F: Fn(&T) -> U>(&self, f: &F) -> Tree<U> {
        match self {
            Tree::Leaf => Tree::Leaf,
            Tree::Node(l, v, r) => Tree::Node(
                Box::new(l.map(f)), f(v), Box::new(r.map(f)),
            ),
        }
    }
}
```
