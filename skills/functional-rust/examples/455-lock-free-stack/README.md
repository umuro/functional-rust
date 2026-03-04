# 455: Lock-Free Stack — Concurrent Push/Pop Without Mutex

**Difficulty:** 3  **Level:** Intermediate

Build a stack where multiple threads push and pop concurrently, using only atomic pointer operations — no mutex, no blocking.

## The Problem This Solves

A `Mutex<Vec<T>>` stack is correct but serialises every push and pop: only one thread operates at a time. Under high contention — many threads pushing and popping rapidly — this becomes a bottleneck. Threads spend most of their time waiting for the lock, not doing work.

A lock-free stack replaces the mutex with a CAS loop on the head pointer. Push: create a node, point it at the current head, CAS the head to your node. Pop: read the head, CAS the head to head.next. If the CAS fails (another thread changed the head), retry. Progress is guaranteed: every failure means another thread succeeded, so the system as a whole makes forward progress even if individual threads retry.

This example also demonstrates Rust's approach to unsafe code. The lock-free stack requires raw pointers — operations that the borrow checker cannot statically verify safe. Rust doesn't forbid this; it requires you to put unsafe code in an `unsafe` block, document the invariants you're maintaining, and verify them yourself. The `unsafe` keyword is a signal: "I, the programmer, am responsible for proving this correct."

## The Intuition

The stack head is an `AtomicPtr<Node<T>>`. Push atomically swings the head to a new node; pop atomically swings it to the current head's next. Each operation is a load-then-CAS loop. Multiple threads can execute their loops simultaneously; at most one succeeds per round and the rest retry with fresh data.

In Java: `java.util.concurrent.ConcurrentLinkedQueue` uses the same pattern internally. In Go: you'd build a similar structure with `atomic.Pointer[Node]`. In Rust, you have to go `unsafe` because raw pointer dereferences require it — but the `unsafe` is isolated and the public API is safe.

## How It Works in Rust

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> { value: T, next: *mut Node<T> }

pub struct Stack<T> { head: AtomicPtr<Node<T>> }

// Safety: we manage ownership via Box; T: Send ensures T is thread-safe
unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Send> Sync for Stack<T> {}

impl<T> Stack<T> {
    pub fn push(&self, v: T) {
        // Allocate node on heap, leak it to a raw pointer
        let n = Box::into_raw(Box::new(Node { value: v, next: ptr::null_mut() }));
        loop {
            let h = self.head.load(Ordering::Relaxed);
            unsafe { (*n).next = h; } // point new node at current head
            // CAS: if head is still h, replace with n (our new node)
            match self.head.compare_exchange_weak(h, n, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => break,  // published
                Err(_) => {}     // retry — head changed
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let h = self.head.load(Ordering::Acquire);
            if h.is_null() { return None; }
            let next = unsafe { (*h).next };
            // CAS: if head is still h, replace with h.next
            match self.head.compare_exchange_weak(h, next, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => {
                    // We own h now — no other thread can reach it
                    let v = unsafe { ptr::read(&(*h).value) };
                    unsafe { drop(Box::from_raw(h)); }  // reclaim memory
                    return Some(v);
                }
                Err(_) => {} // retry
            }
        }
    }
}
```

**Important caveat:** This implementation has the ABA problem. If thread A reads head = P, thread B pops P and pushes a new node that happens to get allocated at address P, thread A's CAS will succeed but now `P.next` points somewhere unexpected. Safe production use requires epoch-based memory reclamation (the `crossbeam-epoch` crate). This example is for educational clarity, not production use.

## What This Unlocks

- **High-contention concurrent stacks** — work-stealing queues, free-lists, and undo stacks in concurrent algorithms.
- **Understanding unsafe Rust** — see how `Box::into_raw`, `ptr::read`, and `Box::from_raw` manage ownership manually across raw pointer operations.
- **CAS loop patterns** — the push/pop loops here are the template for every lock-free data structure.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Head pointer | `Atomic.make head_node` (OCaml 5) | `AtomicPtr<Node<T>>` |
| Allocate node | GC-managed | `Box::into_raw(Box::new(...))` — explicit ownership |
| Push | CAS loop to prepend | `compare_exchange_weak` on head, `Ordering::Release` |
| Pop | CAS loop to remove head | `compare_exchange_weak`, then `Box::from_raw` to reclaim |
| Memory reclamation | GC | manual (ABA risk) or epoch-based (`crossbeam-epoch`) |
| ABA problem | GC prevents it | present in this impl — use epoch reclamation in production |
