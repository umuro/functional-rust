# Comparison: Foldable Trait

## Foldable Interface

**OCaml:**
```ocaml
module type FOLDABLE = sig
  type 'a t
  val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b
  val fold_right : ('a -> 'b -> 'b) -> 'a t -> 'b -> 'b
end
```

**Rust:**
```rust
trait Foldable {
    type Item;
    fn fold_left<B, F: FnMut(B, &Self::Item) -> B>(&self, init: B, f: F) -> B;
    fn fold_right<B, F: FnMut(&Self::Item, B) -> B>(&self, init: B, f: F) -> B;
}
```

## Tree Implementation

**OCaml:**
```ocaml
let rec fold_left f acc = function
  | Leaf -> acc
  | Node (l, v, r) ->
    let acc = fold_left f acc l in
    let acc = f acc v in
    fold_left f acc r
```

**Rust:**
```rust
fn fold_left<B, F: FnMut(B, &T) -> B>(&self, init: B, mut f: F) -> B {
    match self {
        Tree::Leaf => init,
        Tree::Node(l, v, r) => {
            let acc = l.fold_left(init, &mut f);
            let acc = f(acc, v);
            r.fold_left(acc, f)
        }
    }
}
```

## Generic Sum

**OCaml:**
```ocaml
let sum (type a) (module F : FOLDABLE with type 'x t = a) xs =
  F.fold_left (fun acc x -> acc + x) 0 xs
```

**Rust:**
```rust
fn sum<F: Foldable<Item = i32>>(foldable: &F) -> i32 {
    foldable.fold_left(0, |acc, x| acc + x)
}
```
