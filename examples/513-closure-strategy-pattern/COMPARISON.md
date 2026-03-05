# OCaml vs Rust: Strategy Pattern

## OCaml
```ocaml
type 'a sorter = { compare: 'a -> 'a -> int }

let make_sorter compare = { compare }
let sort sorter data = List.sort sorter.compare data

let ascending = make_sorter compare
let descending = make_sorter (fun a b -> compare b a)
```

## Rust
```rust
pub struct Sorter<T> {
    compare: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T: Clone> Sorter<T> {
    pub fn new(compare: impl Fn(&T, &T) -> Ordering + 'static) -> Self {
        Sorter { compare: Box::new(compare) }
    }
}
```

## Key Differences

1. **OCaml**: Functions as record fields are natural
2. **Rust**: Need Box<dyn Fn> for runtime polymorphism
3. Both: Strategy is just a function/closure stored in struct
4. **Rust**: Can use generics with impl Fn for compile-time dispatch
5. Both enable swapping algorithms without inheritance
