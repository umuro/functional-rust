# Comparison: Example 178 — Length-Indexed Lists

## Type Definition

### OCaml
```ocaml
type zero = Zero
type 'n succ = Succ

type ('a, 'n) vec =
  | Nil  : ('a, zero) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec
```

### Rust (const generics)
```rust
struct Vec2<T, const N: usize> {
    data: [T; N],
}
```

### Rust (Peano)
```rust
struct VNil;
struct VCons<T, N: Nat, Rest: TypeVec<T>>(T, Rest, PhantomData<N>);
```

## Safe Head

### OCaml
```ocaml
let head : type a n. (a, n succ) vec -> a = function
  | Cons (x, _) -> x
```

### Rust
```rust
impl<T: Copy, const N: usize> Vec2<T, N> {
    fn head(&self) -> T { self.data[0] }
}
```

## Zip (same length guaranteed)

### OCaml
```ocaml
let rec zip : type a b n. (a, n) vec -> (b, n) vec -> (a * b, n) vec =
  fun v1 v2 -> match v1, v2 with
    | Nil, Nil -> Nil
    | Cons (x, xs), Cons (y, ys) -> Cons ((x, y), zip xs ys)
```

### Rust
```rust
fn zip_vec<T: Copy + Default, U: Copy + Default, const N: usize>(
    a: &Vec2<T, N>, b: &Vec2<U, N>,
) -> Vec2<(T, U), N> {
    let mut result = [(T::default(), U::default()); N];
    for i in 0..N { result[i] = (a.data[i], b.data[i]); }
    Vec2 { data: result }
}
```

## Replicate

### OCaml
```ocaml
let rec replicate : type a n. n length -> a -> (a, n) vec =
  fun n x -> match n with
    | LZ -> Nil
    | LS n' -> Cons (x, replicate n' x)
```

### Rust
```rust
impl<T: Default + Copy, const N: usize> Vec2<T, N> {
    fn replicate(val: T) -> Self {
        Vec2 { data: [val; N] }
    }
}
```
