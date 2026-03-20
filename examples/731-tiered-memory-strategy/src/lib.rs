#![allow(clippy::all)]
/// 731: Tiered Memory — Stack → Arena Pool → Heap
use std::cell::Cell;

// ── Tier 2: Bump Arena ────────────────────────────────────────────────────────

/// A simple bump allocator backed by a fixed-size stack slab.
/// All allocations are `u8` slices with `'arena` lifetime.
struct BumpArena<const CAP: usize> {
    slab: [u8; CAP],
    offset: Cell<usize>,
}

impl<const CAP: usize> BumpArena<CAP> {
    const fn new() -> Self {
        BumpArena {
            slab: [0u8; CAP],
            offset: Cell::new(0),
        }
    }

    /// Allocate `n` bytes from the arena. Returns `None` if full.
    fn alloc(&self, n: usize) -> Option<&mut [u8]> {
        let start = self.offset.get();
        let end = start.checked_add(n)?;
        if end > CAP {
            return None;
        }
        self.offset.set(end);
        // SAFETY: we've verified bounds; slab lives as long as &self
        let ptr = unsafe { (self.slab.as_ptr() as *mut u8).add(start) };
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, n) })
    }

    /// Reset the arena — O(1), no destructors called.
    fn reset(&self) {
        self.offset.set(0);
    }

    fn used(&self) -> usize {
        self.offset.get()
    }
    fn remaining(&self) -> usize {
        CAP - self.used()
    }
}

// ── Tier 3: Heap fallback ─────────────────────────────────────────────────────

enum Allocation<'a> {
    Stack(u8),           // Tier 1: trivial value
    Arena(&'a mut [u8]), // Tier 2: pool
    Heap(Box<[u8]>),     // Tier 3: heap
}

impl<'a> Allocation<'a> {
    fn as_slice(&self) -> &[u8] {
        match self {
            Allocation::Stack(v) => std::slice::from_ref(v),
            Allocation::Arena(s) => s,
            Allocation::Heap(b) => b,
        }
    }
}

fn tier_alloc<'a, const CAP: usize>(arena: &'a BumpArena<CAP>, size: usize) -> Allocation<'a> {
    if size == 1 {
        return Allocation::Stack(0);
    }
    if let Some(slice) = arena.alloc(size) {
        return Allocation::Arena(slice);
    }
    Allocation::Heap(vec![0u8; size].into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_alloc_and_use() {
        let arena: BumpArena<128> = BumpArena::new();
        let slice = arena.alloc(10).unwrap();
        slice[0] = 7;
        assert_eq!(slice[0], 7);
        assert_eq!(arena.used(), 10);
    }

    #[test]
    fn arena_full_returns_none() {
        let arena: BumpArena<8> = BumpArena::new();
        assert!(arena.alloc(8).is_some());
        assert!(arena.alloc(1).is_none());
    }

    #[test]
    fn arena_reset_reclaims_space() {
        let arena: BumpArena<16> = BumpArena::new();
        arena.alloc(16).unwrap();
        assert_eq!(arena.remaining(), 0);
        arena.reset();
        assert_eq!(arena.remaining(), 16);
        assert!(arena.alloc(16).is_some());
    }

    #[test]
    fn tier_alloc_heap_fallback() {
        let arena: BumpArena<4> = BumpArena::new();
        let alloc = tier_alloc(&arena, 100);
        assert_eq!(alloc.as_slice().len(), 100);
    }

    #[test]
    fn tier_alloc_uses_arena_when_space() {
        let arena: BumpArena<512> = BumpArena::new();
        let _a1 = tier_alloc(&arena, 10);
        assert_eq!(arena.used(), 10);
    }
}
