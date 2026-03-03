# OCaml vs Rust: Functor Comparable Set

## Side-by-Side Code

### OCaml

```ocaml
module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module MakeSet (C : COMPARABLE) = struct
  type t = C.t list
  let empty = []
  let mem x = List.exists (fun y -> C.compare x y = 0)
  let add x s = if mem x s then s else x :: s
  let to_list s = List.sort C.compare s
end

module IntSet = MakeSet(Int)

let s = IntSet.(empty |> add 3 |> add 1 |> add 3 |> add 2)
(* IntSet.to_list s = [1; 2; 3] *)
```

### Rust (idiomatic — sorted Vec, binary search)

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparableSet<T: Ord> {
    items: Vec<T>,
}

impl<T: Ord> ComparableSet<T> {
    pub fn new() -> Self { ComparableSet { items: Vec::new() } }

    pub fn contains(&self, x: &T) -> bool {
        self.items.binary_search(x).is_ok()
    }

    #[must_use]
    pub fn insert(mut self, x: T) -> Self {
        match self.items.binary_search(&x) {
            Ok(_)    => self,
            Err(pos) => { self.items.insert(pos, x); self }
        }
    }

    pub fn to_sorted_vec(&self) -> &[T] { &self.items }
}

let s = ComparableSet::new().insert(3).insert(1).insert(3).insert(2);
// s.to_sorted_vec() == [1, 2, 3]
```

### Rust (functional/recursive — mirrors OCaml list strategy)

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctorSet<T: Ord> {
    items: Vec<T>,   // unsorted, mirrors OCaml `C.t list`
}

impl<T: Ord> FunctorSet<T> {
    pub fn new() -> Self { FunctorSet { items: Vec::new() } }

    // OCaml: let mem x = List.exists (fun y -> C.compare x y = 0)
    pub fn mem(&self, x: &T) -> bool {
        self.items.iter().any(|y| y == x)
    }

    // OCaml: let add x s = if mem x s then s else x :: s
    #[must_use]
    pub fn push(mut self, x: T) -> Self {
        if self.mem(&x) { self } else { self.items.push(x); self }
    }

    // OCaml: let to_list s = List.sort C.compare s
    pub fn to_list(&self) -> Vec<&T> {
        let mut v: Vec<&T> = self.items.iter().collect();
        v.sort();
        v
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Module constraint | `module type COMPARABLE` | `T: Ord` trait bound |
| Functor application | `module IntSet = MakeSet(Int)` | `ComparableSet::<i32>` (monomorphisation) |
| Set type | `C.t list` (internally) | `Vec<T>` |
| Membership | `val mem : C.t -> t -> bool` | `fn contains(&self, x: &T) -> bool` |
| Insert | `val add : C.t -> t -> t` | `fn insert(self, x: T) -> Self` |
| Sorted view | `val to_list : t -> C.t list` | `fn to_sorted_vec(&self) -> &[T]` |

## Key Insights

1. **Functor = generic struct**: OCaml's `MakeSet(C)` functor application produces a new module; Rust's monomorphisation of `ComparableSet<T>` at compile time achieves the same effect — one concrete implementation per element type, zero runtime overhead.
2. **Module type = trait**: `COMPARABLE` specifies `type t` and `val compare : t -> t -> int`; Rust's `Ord` trait encodes exactly this contract (plus `PartialOrd`), and is already implemented by all numeric primitives and `String`.
3. **Immutable update style**: OCaml's `add` returns a new `t` (functional update). The Rust `insert(self, ...) -> Self` signature consumes the old set and returns a new one, encoding the same ownership discipline without hidden copies.
4. **Performance difference**: OCaml's `MakeSet` stores elements in an arbitrary-order list and pays O(n) on `mem` and O(n log n) on `to_list`. The idiomatic Rust `ComparableSet` maintains sorted order on every `insert` using `binary_search`, paying O(n) insert but only O(log n) for `contains` and O(1) for `to_sorted_vec`.
5. **Builder chaining**: OCaml uses the `|>` pipe operator (`empty |> add 3 |> add 1`). Rust replaces this with builder-style method chaining (`ComparableSet::new().insert(3).insert(1)`), which the `#[must_use]` attribute enforces so callers cannot accidentally discard the updated set.

## When to Use Each Style

**Use idiomatic Rust (`ComparableSet`):** When you need frequent membership tests or iteration in sorted order — the sorted `Vec` with binary search gives O(log n) lookups and O(1) sorted iteration with no extra allocation.

**Use functional Rust (`FunctorSet`):** When faithfully translating OCaml code or teaching the functor-to-generic-struct mapping, and when you want to show the direct line from `List.exists`/`List.sort` to their Rust iterator equivalents.
