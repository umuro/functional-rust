# OCaml vs Rust: Const Generics

## Side-by-Side Code

### OCaml (functor-based fixed-size arrays)
```ocaml
module type SIZE = sig val n : int end

module FixedArray (S : SIZE) = struct
  type 'a t = 'a array
  let create default = Array.make S.n default
  let length _ = S.n
  let dot a b =
    let sum = ref 0.0 in
    for i = 0 to S.n - 1 do sum := !sum +. a.(i) *. b.(i) done;
    !sum
end

module Size3 = struct let n = 3 end
module Vec3 = FixedArray(Size3)
```

### Rust (idiomatic — const generic struct)
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FixedArray<T, const N: usize> {
    data: [T; N],
}

pub fn dot<const N: usize>(a: &FixedArray<f64, N>, b: &FixedArray<f64, N>) -> f64 {
    a.data.iter().zip(b.data.iter()).map(|(x, y)| x * y).sum()
}
```

### Rust (matrix multiplication — compile-time dimension checking)
```rust
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

// (M×K) × (K×N) → (M×N): shared dimension K enforced by the type checker
pub fn matmul<const M: usize, const K: usize, const N: usize>(
    a: &Matrix<M, K>,
    b: &Matrix<K, N>,
) -> Matrix<M, N> { ... }
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Size parameter | `module Size3 = struct let n = 3 end` | `const N: usize` in `<T, const N: usize>` |
| Fixed array type | `'a FixedArray(Size3).t` | `FixedArray<T, 3>` |
| Dot product | `Vec3.dot : float array -> float array -> float` | `fn dot<const N: usize>(a: &FixedArray<f64, N>, b: &FixedArray<f64, N>) -> f64` |
| Matrix mult | runtime shape check or separate types | `fn matmul<const M: usize, const K: usize, const N: usize>(a: &Matrix<M,K>, b: &Matrix<K,N>) -> Matrix<M,N>` |
| Wrong-size error | runtime `failwith` or type error at functor application | compile error — types don't unify |

## Key Insights

1. **Encoding size in the type system**: OCaml uses first-class modules (functors) parameterised over a `SIZE` signature to embed the size value `n` into a module. Rust encodes the size directly as a `const` type parameter — `FixedArray<T, N>` — which is part of the monomorphised type, not a runtime module record.

2. **When errors are caught**: OCaml's functor approach catches shape mismatches at module instantiation time (compile time for well-typed programs) but relies on explicit `assert`/`failwith` guards inside the functor for index operations. Rust catches dimension mismatches automatically: `Matrix<2,3>` and `Matrix<3,2>` are incompatible types and no guard code is needed.

3. **Zero runtime overhead**: Both approaches are zero-cost in the sense that size information is erased at runtime (OCaml) or monomorphised (Rust). In Rust, `[T; N]` is a stack-allocated array whose size the compiler knows statically, so no heap allocation or length field is needed.

4. **Ergonomics of the matrix multiply signature**: The Rust signature `matmul<M, K, N>(a: &Matrix<M,K>, b: &Matrix<K,N>) -> Matrix<M,N>` reads almost like a mathematical type rule. The compiler enforces the shared-dimension constraint `K` without any user-written assertion. OCaml would need a functor taking two module arguments and an explicit proof that their sizes match, or a runtime check.

5. **Generality**: OCaml functors can express more complex relationships (e.g., sharing two modules that carry different values) and are a general module-level abstraction. Rust const generics are narrower — only numeric (or other primitive) constants — but integrate seamlessly with trait bounds, inference, and the monomorphiser, making them far more ergonomic for numeric/array programming.

## When to Use Each Style

**Use const generics (Rust) when:** you need stack-allocated, fixed-size collections where the size is known at compile time and you want dimension errors to be compile errors — linear algebra, SIMD wrappers, fixed-size protocols, type-safe matrices.

**Use runtime sizes (Vec / slice) when:** sizes are determined by user input, file I/O, or any other runtime source; or when you need a single generic implementation that works across many sizes without code bloat from monomorphisation.
