# Comparison: Example 179 — Safe Head

## Non-Empty List Type

### OCaml
```ocaml
type ('a, _) safe_list =
  | SNil  : ('a, empty) safe_list
  | SCons : 'a * ('a, _) safe_list -> ('a, nonempty) safe_list

let safe_head : type a. (a, nonempty) safe_list -> a = function
  | SCons (x, _) -> x
```

### Rust
```rust
struct SafeList<T, S> { data: Vec<T>, _state: PhantomData<S> }

impl<T> SafeList<T, NonEmpty> {
    fn head(&self) -> &T { &self.data[0] }
}
// SafeList<T, Empty> has NO head method — compile error if you try
```

## NonEmpty Module/Struct

### OCaml
```ocaml
module NonEmpty = struct
  type 'a t = { head : 'a; tail : 'a list }
  let head ne = ne.head
  let of_list = function [] -> None | x :: xs -> Some { head = x; tail = xs }
  let map f ne = { head = f ne.head; tail = List.map f ne.tail }
end
```

### Rust
```rust
struct NonEmptyVec<T> { head: T, tail: Vec<T> }

impl<T> NonEmptyVec<T> {
    fn head(&self) -> &T { &self.head }
    fn from_vec(v: Vec<T>) -> Option<Self> {
        let mut iter = v.into_iter();
        iter.next().map(|head| NonEmptyVec { head, tail: iter.collect() })
    }
    fn map<U>(&self, f: impl Fn(&T) -> U) -> NonEmptyVec<U> {
        NonEmptyVec { head: f(&self.head), tail: self.tail.iter().map(f).collect() }
    }
}
```

## Type-State Transition

### OCaml
```ocaml
(* Constructing always gives nonempty *)
let l = SCons (1, SCons (2, SNil))  (* type: (int, nonempty) safe_list *)
```

### Rust
```rust
let list = SafeList::<_, Empty>::new()  // Empty
    .push(1)                             // → NonEmpty
    .push(2);                            // still NonEmpty
// Type changes from SafeList<i32, Empty> to SafeList<i32, NonEmpty>
```
