# 551: Rc and Weak for Cycles

**Difficulty:** 4  **Level:** Advanced

Break reference cycles with non-owning weak pointers.

## The Problem This Solves

`Rc<T>` uses reference counting: when the count reaches zero, the value is dropped. This works perfectly for trees where ownership flows in one direction. But add a back-pointer — a child holding a reference to its parent — and you have a cycle. Both sides keep the other's count above zero. Neither ever drops. You have a memory leak.

This problem appears in every bidirectional data structure: trees with parent pointers, doubly-linked lists, observer patterns where subjects know about their observers, and graph nodes that reference neighbors. In garbage-collected languages, the GC handles cycles. In Rust's ownership model, you need to break cycles explicitly.

`Weak<T>` is a non-owning reference: it doesn't increment the strong count. You create one with `Rc::downgrade(&rc)`. When all strong `Rc<T>` references drop, the value is deallocated — even if `Weak<T>` references remain. Those weak refs become "dead": `weak.upgrade()` returns `None`. This makes the pattern explicit: the owner holds a strong `Rc`, the back-pointer holds a `Weak`.

## The Intuition

Think of `Rc` as owning a house and `Weak` as having a key to it. Owning the house keeps it standing. Holding a key doesn't. Once all the owners sell and the house is demolished, your key opens nothing — `upgrade()` returns `None`. The key doesn't prevent demolition.

In a tree: parent owns its children (`Rc`). Children have a key to their parent (`Weak`). The tree is alive as long as someone holds the root. Drop the root, everything falls.

## How It Works in Rust

1. **Downgrade to weak** — `Rc::downgrade(&strong)` creates a `Weak<T>` without incrementing the strong count.
2. **Upgrade to check liveness** — `weak.upgrade()` returns `Option<Rc<T>>`: `Some` if the value is still alive, `None` if it was dropped.
3. **Tree parent pattern** — parent field is `Option<Weak<RefCell<Node>>>`: weak because children don't own their parent.
4. **Reference count inspection** — `Rc::strong_count(&rc)` and `Rc::weak_count(&rc)` let you observe the counts; useful for debugging cycles.
5. **Cycle prevention check** — wrap in a block: create `Rc`, downgrade to `Weak`, drop the `Rc`; `upgrade()` then returns `None`, confirming no cycle.

## What This Unlocks

- Build bidirectional tree and graph structures without memory leaks.
- Implement observer/listener patterns where subjects don't own their observers.
- Reason precisely about object lifetimes in data structures with back-references.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared ownership | GC; no explicit reference counting | `Rc<T>`: single-threaded reference counting |
| Cycles | GC detects and collects cycles | `Rc` leaks cycles; must use `Weak` to break them |
| Non-owning reference | Mutable weak refs via `Weak` module or custom | `Weak<T>`: explicit non-owning handle; `upgrade()` → `Option` |
| Back-pointers in trees | Record field; GC handles liveness | `Option<Weak<RefCell<Node>>>`: explicit, non-owning |
