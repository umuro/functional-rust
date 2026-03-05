# OCaml vs Rust: Lifetimes in impl Blocks

## OCaml
```ocaml
(* Methods just work — no lifetime management *)
type 'a view = { data: 'a array }

let make_view data = { data }
let get view i = view.data.(i)
let slice view start end_ =
  { data = Array.sub view.data start (end_ - start) }
```

## Rust
```rust
pub struct View<'a, T> {
    data: &'a [T],
}

// 'a appears on impl and is used in methods
impl<'a, T> View<'a, T> {
    pub fn new(data: &'a [T]) -> Self { View { data } }

    // Return type tied to 'a, not &self
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }
}
```

## Key Differences

1. **OCaml**: Type parameter 'a is just a generic, not lifetime
2. **Rust**: impl<'a, T> declares lifetime for all methods
3. **Rust**: Return &'a T means "valid as long as original data"
4. **Rust**: &self vs 'a distinction matters for return types
5. Both: Methods can return references into borrowed data
