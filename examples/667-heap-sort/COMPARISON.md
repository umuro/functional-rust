# OCaml vs Rust: Heap Sort

Both use imperative array operations since heap sort is inherently in-place.

## Rust
```rust
fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let (mut largest, left, right) = (i, 2*i+1, 2*i+2);
    if left < n && arr[left] > arr[largest] { largest = left; }
    ...
}
```

## OCaml
```ocaml
let rec heapify size i =
  let largest = ref i in
  if left < size && arr.(left) > arr.(!largest) then largest := left;
  ...
```

## Key Insight
Both implementations are similar - heap sort doesn't benefit from functional style.
