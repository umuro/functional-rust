📖 **[View on hightechmind.io →](https://hightechmind.io/rust/370-small-vec-pattern)**

---

# 370: SmallVec Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Most collections in practice are small — a function rarely returns more than a few results, most AST nodes have 2-3 children, most tokenizers produce short token sequences. Standard `Vec<T>` always heap-allocates, even for zero or one element. The `SmallVec` optimization stores up to N items inline (on the stack or in the struct), spilling to heap only when needed. The `smallvec` crate implements this; this example shows the pattern from scratch using a const-generic `N`. SmallVec is used in LLVM (for instruction operands), Rust's own compiler, browser engines, and game ECS implementations to eliminate heap pressure for small collections.

## Learning Outcomes

- Implement `SmallVec<T, const N: usize>` using an `Inline`/`Heap` enum
- Store `[Option<T>; N]` inline to avoid heap allocation for small counts
- Spill to `Vec<T>` when the inline capacity is exceeded
- Implement `push`, `get`, `len`, and `iter` abstracting over both variants
- Understand the const-generics feature (`const N: usize`) for compile-time array sizes
- Recognize the space/speed tradeoff: SmallVec is larger than a plain `Vec` (by the inline array size)

## Rust Application

```rust
#[derive(Debug, Clone)]
pub enum SmallVec<T, const N: usize> {
    Inline { data: [Option<T>; N], len: usize },
    Heap(Vec<T>),
}

impl<T: Clone + Default, const N: usize> SmallVec<T, N> {
    pub fn new() -> Self {
        Self::Inline {
            data: std::array::from_fn(|_| None),
            len: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        match self {
            Self::Inline { data, len } if *len < N => {
                data[*len] = Some(val);
                *len += 1;
            }
            Self::Inline { data, len } => {
                // Spill: move inline data to heap
                let mut v: Vec<T> = data[..*len].iter_mut()
                    .filter_map(|x| x.take()).collect();
                v.push(val);
                *self = Self::Heap(v);
            }
            Self::Heap(v) => v.push(val),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Inline { len, .. } => *len,
            Self::Heap(v) => v.len(),
        }
    }
}
```

`std::array::from_fn(|_| None)` initializes the inline array with `None` — required because `[None; N]` only works when `T: Copy`. The spill from `Inline` to `Heap` is a one-time O(N) operation that copies the N inline elements into a new `Vec` before pushing the overflow element.

## OCaml Approach

OCaml's minor heap already provides fast allocation for small values, making SmallVec less critical. A functional approximation uses a variant type:

```ocaml
type 'a small_vec =
  | Small of 'a array * int  (* inline data + length *)
  | Large of 'a list         (* spilled to list *)

let max_inline = 4

let push sv x = match sv with
  | Small (a, n) when n < max_inline ->
    a.(n) <- x; Small (a, n + 1)
  | Small (a, n) ->
    let lst = Array.to_list a |> List.filteri (fun i _ -> i < n) in
    Large (x :: lst)
  | Large lst -> Large (x :: lst)
```

In practice, OCaml's allocator is fast enough that this optimization is rarely needed — the GC minor heap bump-allocates small objects at CPU-cache speed.

## Key Differences

| Aspect | Rust `SmallVec<T, N>` | OCaml variant |
|--------|-----------------------|---------------|
| Inline storage | `[Option<T>; N]` on stack/struct | `'a array` in variant |
| Heap spill | One-time move to `Vec<T>` | Switch to list |
| Const generics | `const N: usize` — compile-time | Runtime constant |
| Production crate | `smallvec` (extensively used in Rust ecosystem) | No standard equivalent |
| Size tradeoff | `sizeof(SmallVec) = max(sizeof(Inline), sizeof(Heap))` | Similar |

## Exercises

1. **iter()**: Implement `fn iter(&self) -> impl Iterator<Item = &T>` that works for both `Inline` and `Heap` variants without allocating an intermediate `Vec`.
2. **Benchmark**: Create `SmallVec<i32, 4>` and `Vec<i32>`, push 1, 2, 4, and 8 elements respectively, and measure heap allocations using a custom allocator or `std::alloc::System` with tracking.
3. **`smallvec` crate**: Replace the manual implementation with `smallvec::SmallVec<[i32; 4]>` from the `smallvec` crate; verify that slicing, sorting, and `extend` all work correctly.
