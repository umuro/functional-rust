**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[finger-tree on hightechmind.io](https://hightechmind.io/posts/functional-rust/finger-tree)

---

## Problem Statement

Explore the finger tree — a purely functional deque with O(1) amortized push/pop at both ends and O(log n) split/concat. The classic recursive type creates an infinitely-nested `FingerTree<Node<T>>` that Rust's type system cannot express directly. This implementation uses a `VecDeque`-backed simplified finger tree to demonstrate the interface while explaining the structural challenge.

## Learning Outcomes

- Understand the finger tree structure: two "fingers" (small buffers at each end) + a lazy spine
- Recognize the recursive type problem: `FingerTree<Node<T>>` is not a finite type in Rust without boxing
- Implement a simplified finger tree API using `VecDeque` with consuming `push`/`pop` methods
- Apply consuming (value-taking) methods: `push_front(self, x) -> Self` returns a new tree
- Understand when `VecDeque` is sufficient vs when a true finger tree adds value

## Rust Application

```rust
#[derive(Debug, Clone)]
pub struct FingerTree<T> {
    deque: VecDeque<T>,
}

impl<T: Clone> FingerTree<T> {
    pub fn empty() -> Self {
        FingerTree { deque: VecDeque::new() }
    }

    // Consuming push — returns new tree (functional style)
    pub fn push_front(mut self, x: T) -> Self {
        self.deque.push_front(x);
        self
    }

    pub fn push_back(mut self, x: T) -> Self {
        self.deque.push_back(x);
        self
    }

    // Consuming pop — returns (element, remaining_tree)
    pub fn pop_front(mut self) -> (Option<T>, Self) {
        let item = self.deque.pop_front();
        (item, self)
    }

    pub fn pop_back(mut self) -> (Option<T>, Self) {
        let item = self.deque.pop_back();
        (item, self)
    }

    pub fn len(&self) -> usize { self.deque.len() }
    pub fn is_empty(&self) -> bool { self.deque.is_empty() }

    pub fn concat(mut self, mut other: Self) -> Self {
        self.deque.extend(other.deque.drain(..));
        self
    }
}
```

The consuming API (`self` not `&mut self`) reflects the functional style where operations return new values rather than mutating in place. Callers chain operations: `tree.push_back(x).push_back(y)`.

The real finger tree type in Haskell is:

```
data FingerTree a
  = Empty
  | Single a
  | Deep (Digit a) (FingerTree (Node a)) (Digit a)
```

The `FingerTree (Node a)` creates an infinite type that requires `Box<FingerTree<Node<T>>>` in Rust — feasible but complex. For most practical purposes `VecDeque` suffices.

## OCaml Approach

```ocaml
(* Simplified finger tree as two-list deque *)
type 'a finger_tree = {
  front: 'a list;
  back:  'a list;
}

let empty = { front = []; back = [] }

let push_front x t = { t with front = x :: t.front }
let push_back  x t = { t with back  = x :: t.back  }

let pop_front t = match t.front with
  | [] -> (match List.rev t.back with
    | [] -> None, t
    | x :: rest -> Some x, { front = rest; back = [] })
  | x :: rest -> Some x, { t with front = rest }

(* True finger tree: see Jane Street's Core.Deque or Batteries' Deque *)
```

OCaml's `with` record update syntax makes persistent push O(1). The two-list variant is the practical OCaml equivalent. For a true finger tree, Haskell's `Data.Sequence` or OCaml's `BatDeque` provides the full implementation.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Recursive type | Cannot express `FingerTree<Node<T>>` without boxing | Natural recursive type in Haskell/OCaml |
| Consuming API | `fn push(self) -> Self` | Persistent `{ t with ... }` |
| Practical deque | `VecDeque` — O(1) amortized | Two-list or stdlib `Queue` |
| True finger tree | Requires `Box<...>` indirection | `Data.Sequence` (Haskell), `BatDeque` (OCaml) |

Finger trees generalize to sequences supporting any measured monoid (e.g., size, priority, index) with O(log n) split/concat. They are the theoretical foundation of functional sequence libraries.

## Exercises

1. Implement a true two-finger tree: `struct TwoFinger<T> { front: Vec<T>, spine: VecDeque<T>, back: Vec<T> }` — O(1) push/pop at both ends when fingers are small.
2. Implement `split_at(self, idx: usize) -> (Self, Self)` for the simplified version.
3. Implement `append_all(self, items: impl IntoIterator<Item=T>) -> Self` efficiently.
4. Show how a finger tree with a `Size` monoid supports O(log n) random access by index.
5. Implement a `map` method: `map<U: Clone, F: Fn(T) -> U>(self, f: F) -> FingerTree<U>`.
