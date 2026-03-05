# OCaml vs Rust: Windows and Chunks

## Side-by-Side Code

### OCaml
```ocaml
let windows n lst =
  let arr = Array.of_list lst in
  let len = Array.length arr in
  if n > len then []
  else List.init (len - n + 1) (fun i ->
    Array.to_list (Array.sub arr i n))

let moving_average n lst =
  let ws = windows n (List.map float_of_int lst) in
  List.map (fun w ->
    List.fold_left (+.) 0.0 w /. float_of_int n) ws

let chunks n lst =
  let rec aux acc current count = function
    | [] -> List.rev (if current = [] then acc else List.rev current :: acc)
    | x :: rest ->
      if count = n then aux (List.rev current :: acc) [x] 1 rest
      else aux acc (x :: current) (count + 1) rest
  in aux [] [] 0 lst
```

### Rust (idiomatic)
```rust
pub fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    data.windows(window_size)
        .map(|w| w.iter().sum::<f64>() / window_size as f64)
        .collect()
}

pub fn chunk_sums(data: &[i32], size: usize) -> Vec<i32> {
    data.chunks(size).map(|c| c.iter().sum()).collect()
}

pub fn chunk_exact_sums(data: &[i32], size: usize) -> (Vec<i32>, &[i32]) {
    let iter = data.chunks_exact(size);
    let remainder = iter.remainder();
    let sums = iter.map(|c| c.iter().sum()).collect();
    (sums, remainder)
}
```

### Rust (functional/recursive)
```rust
pub fn windows_recursive<T: Clone>(slice: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 || slice.len() < n {
        return vec![];
    }
    let mut result = vec![slice[..n].to_vec()];
    result.extend(windows_recursive(&slice[1..], n));
    result
}

pub fn chunks_recursive<T: Clone>(slice: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 || slice.is_empty() {
        return vec![];
    }
    let end = n.min(slice.len());
    let mut result = vec![slice[..end].to_vec()];
    result.extend(chunks_recursive(&slice[end..], n));
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Windows | `val windows : int -> 'a list -> 'a list list` | `fn windows_recursive<T: Clone>(&[T], usize) -> Vec<Vec<T>>` |
| Built-in windows | `Array.sub` (copies) | `slice.windows(n)` → `Iterator<Item=&[T]>` (zero-copy) |
| Chunks | manual recursive + `Array.sub` | `slice.chunks(n)` → `Iterator<Item=&[T]>` (zero-copy) |
| Exact chunks | manual boundary check | `slice.chunks_exact(n)` + `.remainder()` |
| Moving average | `List.fold_left` over sub-lists | `.map(|w| w.iter().sum::<f64>() / n as f64)` |

## Key Insights

1. **Zero-copy vs O(n²) allocation:** OCaml's `Array.sub` allocates a new array for every window or chunk — O(n·k) total allocations. Rust's `.windows(n)` and `.chunks(n)` hand out `&[T]` slice references into the original data with no copying whatsoever.

2. **Built-in vs hand-rolled:** OCaml has no standard `windows` or `chunks` on lists; you must write them from scratch using `Array.sub` or recursive list manipulation. Rust bakes both directly into the slice type as iterator-producing methods.

3. **`chunks_exact` for strict batching:** Rust provides `chunks_exact(n)` which silently skips the final partial chunk and exposes it through `.remainder()`. This makes it easy to separate "full batches" from "leftover" without manual length arithmetic.

4. **Iterator composability:** Because `.windows` and `.chunks` produce iterators, they compose directly with `.map`, `.filter`, and `.collect` — no intermediate `List.map` steps needed. The OCaml versions must go through an intermediate list of lists.

5. **Recursive style as explicit OCaml mirror:** The recursive Rust implementations (`windows_recursive`, `chunks_recursive`) use slice-slicing `&slice[1..]` and `&slice[end..]` instead of list destructuring, but the structural recursion is identical to the OCaml versions — demonstrating how Rust can mimic functional decomposition while still being safe and zero-cost.

## When to Use Each Style

**Use idiomatic Rust (`.windows` / `.chunks`) when:** processing slices in production code — it is zero-copy, bounds-checked, and composes cleanly with iterators.
**Use recursive Rust when:** teaching the OCaml parallel explicitly, or when building a generic abstraction over a custom data structure that doesn't expose slices.
