// 726. Memory pool / bump allocator pattern
//
// Implements a typed pool allocator and a lifetime-safe bump arena.

use std::alloc::{alloc, dealloc, Layout};
use std::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

// ── Part 1: Fixed-size typed object pool ─────────────────────────────────────

/// A pool of `CAP` pre-allocated `T` slots.
/// Allocation and deallocation are O(1) via a free-list.
pub struct Pool<T, const CAP: usize> {
    slots:     Box<[std::mem::MaybeUninit<T>; CAP]>,
    free_head: Option<usize>,
    next_free: [usize; CAP], // next pointer for free-list
    live:      usize,
}

impl<T, const CAP: usize> Pool<T, CAP> {
    pub fn new() -> Self {
        // Build free list: 0 → 1 → 2 → … → CAP-1 → sentinel
        let mut next_free = [0usize; CAP];
        for i in 0..CAP { next_free[i] = i + 1; }
        Self {
            // SAFETY: Array of MaybeUninit requires no initialisation.
            slots:     Box::new(unsafe { std::mem::MaybeUninit::uninit().assume_init() }),
            free_head: Some(0),
            next_free,
            live:      0,
        }
    }

    /// Allocate a slot. Returns `None` when the pool is exhausted.
    pub fn alloc(&mut self, val: T) -> Option<usize> {
        let idx = self.free_head?;
        let next = self.next_free[idx];
        self.free_head = if next < CAP { Some(next) } else { None };
        self.slots[idx].write(val);
        self.live += 1;
        Some(idx)
    }

    /// Return a handle to the pool. Panics on invalid index.
    pub fn get(&self, idx: usize) -> &T {
        assert!(idx < CAP);
        // SAFETY: `idx` was returned by `alloc` and not yet freed, so the slot
        // is initialised.
        unsafe { self.slots[idx].assume_init_ref() }
    }

    /// Deallocate the slot at `idx`.
    ///
    /// # Safety
    /// `idx` must have been returned by `alloc` and not yet freed.
    pub unsafe fn dealloc(&mut self, idx: usize) {
        assert!(idx < CAP);
        // SAFETY: caller guarantees the slot is live.
        unsafe { self.slots[idx].assume_init_drop(); }
        self.next_free[idx] = self.free_head.map_or(CAP, |h| h);
        self.free_head = Some(idx);
        self.live -= 1;
    }

    pub fn live(&self) -> usize { self.live }
}

// ── Part 2: Lifetime-safe bump arena ─────────────────────────────────────────

/// A bump allocator backed by a heap-allocated byte slab.
/// All objects allocated from this arena must not outlive it.
///
/// The `'arena` lifetime parameter propagates to every `&'arena T` returned.
pub struct Arena {
    ptr: NonNull<u8>,
    cap: usize,
    pos: Cell<usize>, // Cell for interior mutability through shared ref
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 16).expect("valid layout");
        // SAFETY: layout has non-zero size (we require capacity > 0).
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).expect("allocation failed");
        Self { ptr, cap: capacity, pos: Cell::new(0) }
    }

    /// Allocate space for one `T`, returning a mutable reference with lifetime
    /// tied to this arena. Zero-initialises the slot.
    pub fn alloc<T: Default>(&self) -> &mut T {
        let layout = Layout::new::<T>();
        let offset  = self.pos.get();
        let aligned = (offset + layout.align() - 1) & !(layout.align() - 1);
        let next    = aligned + layout.size();
        assert!(next <= self.cap, "Arena OOM");
        self.pos.set(next);

        // SAFETY: `aligned` is within the allocated slab and properly aligned
        // for `T`. We return a unique `&mut T` tied to `&self` (arena lifetime).
        // The arena does not move the slab, so the reference stays valid.
        let slot_ptr = unsafe { self.ptr.as_ptr().add(aligned) as *mut T };
        unsafe {
            slot_ptr.write(T::default());
            &mut *slot_ptr
        }
    }

    /// Allocate a byte slice of `len` bytes.
    pub fn alloc_slice(&self, len: usize) -> &mut [u8] {
        let aligned = self.pos.get();
        let next = aligned + len;
        assert!(next <= self.cap, "Arena OOM");
        self.pos.set(next);
        // SAFETY: `aligned..next` is within the slab, not aliased.
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr().add(aligned), len)
        }
    }

    /// Reset the bump pointer. All previous allocations become invalid.
    ///
    /// # Safety
    /// Caller must guarantee no references into this arena are live after this call.
    pub unsafe fn reset(&self) {
        self.pos.set(0);
    }

    pub fn used(&self) -> usize { self.pos.get() }
    pub fn capacity(&self) -> usize { self.cap }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.cap, 16).unwrap();
        // SAFETY: `self.ptr` was allocated with this layout in `new()`.
        unsafe { dealloc(self.ptr.as_ptr(), layout); }
    }
}

// ── Part 3: Arena-allocated parse tree ────────────────────────────────────────

/// A simple expression AST node — tied to arena lifetime `'a`.
#[derive(Debug)]
pub enum Expr<'a> {
    Num(i64),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
}

impl<'a> Expr<'a> {
    pub fn eval(&self) -> i64 {
        match self {
            Expr::Num(n)   => *n,
            Expr::Add(l,r) => l.eval() + r.eval(),
            Expr::Mul(l,r) => l.eval() * r.eval(),
        }
    }
}

/// Build `(1 + 2) * 3` in the arena — no separate heap allocations.
fn build_ast(arena: &Arena) -> &Expr<'_> {
    let one   = arena.alloc::<Expr>();
    *one = Expr::Num(1);

    let two   = arena.alloc::<Expr>();
    *two = Expr::Num(2);

    let three = arena.alloc::<Expr>();
    *three = Expr::Num(3);

    let add   = arena.alloc::<Expr>();
    *add = Expr::Add(one, two);

    let mul   = arena.alloc::<Expr>();
    *mul = Expr::Mul(add, three);
    mul
}

// ── main ──────────────────────────────────────────────────────────────────────


// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_alloc_dealloc() {
        let mut p = Pool::<i32, 4>::new();
        let h = p.alloc(42).unwrap();
        assert_eq!(*p.get(h), 42);
        assert_eq!(p.live(), 1);
        // SAFETY: h just returned from alloc, not freed.
        unsafe { p.dealloc(h); }
        assert_eq!(p.live(), 0);
    }

    #[test]
    fn pool_exhaustion() {
        let mut p = Pool::<u8, 2>::new();
        assert!(p.alloc(1).is_some());
        assert!(p.alloc(2).is_some());
        assert!(p.alloc(3).is_none()); // full
    }

    #[test]
    fn pool_reuse_slot() {
        let mut p = Pool::<u8, 2>::new();
        let h = p.alloc(1).unwrap();
        // SAFETY: h just returned from alloc.
        unsafe { p.dealloc(h); }
        let h2 = p.alloc(99).unwrap();
        assert_eq!(*p.get(h2), 99);
    }

    #[test]
    fn arena_alloc_and_reset() {
        let arena = Arena::new(1024);
        let x = arena.alloc::<u64>();
        *x = 12345;
        assert_eq!(*x, 12345);
        assert!(arena.used() > 0);
        // SAFETY: no references live after this.
        unsafe { arena.reset(); }
        assert_eq!(arena.used(), 0);
    }

    #[test]
    fn arena_ast_eval() {
        let arena = Arena::new(1024);
        let expr = build_ast(&arena);
        assert_eq!(expr.eval(), 9);  // (1+2)*3
    }

    #[test]
    #[should_panic(expected = "Arena OOM")]
    fn arena_oom() {
        let arena = Arena::new(8);
        let _ = arena.alloc_slice(9); // too big
    }
}
