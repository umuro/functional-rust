# OCaml vs Rust: Insertion Sort

## OCaml (functional)
```ocaml
let rec insert x = function
  | [] -> [x]
  | h :: t -> if x <= h then x :: h :: t else h :: insert x t

let insertion_sort lst = List.fold_right insert lst []
```

## Rust (imperative)
```rust
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
```

## Key Insight

OCaml's functional version is elegant but allocates.
Rust's in-place version is more cache-friendly.
