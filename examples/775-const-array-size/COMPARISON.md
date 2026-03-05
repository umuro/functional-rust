# OCaml vs Rust: Const Array Size

## Stack-Allocated Vec

### Rust
```rust
pub struct StackVec<T: Copy + Default, const CAP: usize> {
    data: [T; CAP],
    len: usize,
}

let mut v: StackVec<i32, 4> = StackVec::new();
v.push(1)?;
```

### OCaml
No direct equivalent. Would need:
```ocaml
(* Array with runtime capacity check *)
type 'a stack_vec = {
  data: 'a array;
  mutable len: int;
}

let create cap default = {
  data = Array.make cap default;
  len = 0;
}
```

## Array Functions with Size

### Rust
```rust
pub fn sum<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

pub fn zip_arrays<T, U, const N: usize>(a: &[T; N], b: &[U; N]) -> [(T, U); N] {
    // Size match guaranteed at compile time!
}
```

### OCaml
```ocaml
(* Runtime size check *)
let zip a b =
  if Array.length a <> Array.length b then
    invalid_arg "different lengths";
  Array.init (Array.length a) (fun i -> (a.(i), b.(i)))
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Size guarantee | Runtime check | Compile-time |
| Stack allocation | Not controllable | Explicit with `[T; N]` |
| Type errors | Runtime | Compile-time |
| No allocation | Arrays copy | Arrays are inline |
