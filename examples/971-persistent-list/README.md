[persistent-list on hightechmind.io](https://hightechmind.io/posts/functional-rust/persistent-list)

---

## Problem Statement

Implement a persistent linked list where `push` creates a new version sharing the tail with the original. Rust requires `Rc<T>` for shared ownership since the borrow checker prohibits multiple owners. Each version of the list points to its tail via `Rc::clone` — a reference count increment costing O(1) with no data copying.

## Learning Outcomes

- Implement `enum PList<T> { Nil, Cons(T, Rc<PList<T>>) }` with `Rc` for shared tails
- Implement `push(x, tail) -> Rc<PList<T>>` using `Rc::clone` to share the existing tail — O(1)
- Implement `pop(list) -> Option<(&T, Rc<PList<T>>)>` that returns a reference to the head and a clone of the tail
- Understand that `Rc<T>` enables shared immutable ownership (like OCaml's GC) but without thread safety (use `Arc<T>` for `Send + Sync`)
- Recognize structural sharing: two lists pointing to the same tail consume only O(1) extra memory

## Rust Application

```rust
#[derive(Debug)]
pub enum PList<T> {
    Nil,
    Cons(T, Rc<PList<T>>),
}

impl<T: Clone + PartialEq> PList<T> {
    pub fn nil() -> Rc<PList<T>> {
        Rc::new(PList::Nil)
    }

    pub fn push(x: T, tail: &Rc<PList<T>>) -> Rc<PList<T>> {
        Rc::new(PList::Cons(x, Rc::clone(tail)))  // O(1), shares tail
    }

    pub fn pop(list: &Rc<PList<T>>) -> Option<(&T, Rc<PList<T>>)> {
        match list.as_ref() {
            PList::Nil => None,
            PList::Cons(x, rest) => Some((x, Rc::clone(rest))),
        }
    }

    pub fn peek(list: &Rc<PList<T>>) -> Option<&T> {
        match list.as_ref() {
            PList::Nil => None,
            PList::Cons(x, _) => Some(x),
        }
    }
}
```

`Rc::clone(tail)` increments the reference count without copying the list data. Two `PList` versions that share a tail both point to the same heap allocation. The tail is freed only when all `Rc` clones to it are dropped — equivalent to OCaml's GC tracing.

`list.as_ref()` converts `&Rc<PList<T>>` to `&PList<T>` for pattern matching. The returned `&T` in `pop` borrows from the list, so its lifetime is tied to the `Rc<PList<T>>` being held alive by the caller.

## OCaml Approach

```ocaml
(* OCaml lists are persistent by default — no explicit Rc needed *)
type 'a plist = Nil | Cons of 'a * 'a plist

let push x tail = Cons (x, tail)      (* O(1), GC shares tail *)
let pop = function
  | Nil -> None
  | Cons (x, rest) -> Some (x, rest)

(* Using standard OCaml list — identical behavior *)
let push_std x xs = x :: xs          (* O(1) *)
let pop_std = function [] -> None | x :: rest -> Some (x, rest)
```

In OCaml, all list values are persistent by default — `x :: xs` shares `xs` without copying. The GC tracks references automatically. There is no need to wrap in `Rc` because OCaml's garbage collector manages sharing transparently.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Shared ownership | `Rc<T>` — explicit reference counting | GC — automatic |
| Push | `Rc::new(Cons(x, Rc::clone(tail)))` | `x :: xs` or `Cons (x, tail)` |
| Thread safety | `Rc` not `Send`; use `Arc` for threads | GC handles all cases |
| Memory freedom | Reference count drops to 0 | GC collects when unreachable |

Persistent data structures enable "time travel" — holding `Rc<PList<T>>` from before a `push` gives access to the previous version. This is how functional programs implement undo, versioned state, and persistent queues.

## Exercises

1. Implement `length(list: &Rc<PList<T>>) -> usize` — count elements iteratively to avoid stack overflow.
2. Implement `append(a: &Rc<PList<T>>, b: &Rc<PList<T>>) -> Rc<PList<T>>` — creates copies of `a`'s elements pointing to shared `b`.
3. Implement `to_vec(list: &Rc<PList<T>>) -> Vec<T> where T: Clone` — collect all elements.
4. Convert `Rc` to `Arc` and verify the list is `Send + Sync` for multi-threaded access.
5. Implement a persistent stack with `push`, `pop`, and `peek` methods, demonstrating that old versions remain accessible after `push`.
