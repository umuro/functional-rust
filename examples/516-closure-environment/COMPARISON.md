# OCaml vs Rust: Complex Closure Environments

## OCaml
```ocaml
let make_counter start =
  let count = ref start in
  fun () ->
    let c = !count in
    count := c + 1;
    c

let make_cycler items =
  let idx = ref 0 in
  let arr = Array.of_list items in
  fun () ->
    let v = arr.(!idx) in
    idx := (!idx + 1) mod Array.length arr;
    v
```

## Rust
```rust
pub fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        let current = count;
        count += 1;
        current
    }
}

pub fn make_cycler<T: Clone>(items: Vec<T>) -> impl FnMut() -> T {
    let mut index = 0;
    move || {
        let val = items[index].clone();
        index = (index + 1) % items.len();
        val
    }
}
```

## Key Differences

1. **OCaml**: Uses `ref` for mutable captured values
2. **Rust**: `move` captures ownership, `mut` allows mutation
3. Both: Closures can capture structs, collections, other closures
4. **Rust**: FnMut trait indicates the closure mutates its environment
5. Both enable stateful closures with complex captured environments
