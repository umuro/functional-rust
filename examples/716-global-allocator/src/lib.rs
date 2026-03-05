//! # 716: Custom Global Allocator with `#[global_allocator]`
//!
//! Demonstrates replacing Rust's default allocator with a tracking wrapper
//! and a simple bump allocator over a fixed static buffer.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// ── Part 1: Tracking Allocator ────────────────────────────────────────────────

/// Wraps the system allocator and tracks live bytes + allocation count.
pub struct TrackingAllocator {
    inner: System,
}

static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Default for TrackingAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackingAllocator {
    pub const fn new() -> Self {
        Self { inner: System }
    }

    /// Returns the number of bytes currently live (allocated but not yet freed).
    pub fn live_bytes() -> usize {
        LIVE_BYTES.load(Ordering::Relaxed)
    }

    /// Returns the total number of `alloc` calls made so far.
    pub fn alloc_count() -> usize {
        ALLOC_COUNT.load(Ordering::Relaxed)
    }

    /// Returns the total number of `dealloc` calls made so far.
    pub fn dealloc_count() -> usize {
        DEALLOC_COUNT.load(Ordering::Relaxed)
    }

    /// Returns a snapshot `(live_bytes, alloc_count, dealloc_count)`.
    pub fn snapshot() -> (usize, usize, usize) {
        (
            LIVE_BYTES.load(Ordering::Relaxed),
            ALLOC_COUNT.load(Ordering::Relaxed),
            DEALLOC_COUNT.load(Ordering::Relaxed),
        )
    }
}

// SAFETY: All invariants are delegated to `System`, which is a correct
// implementation of `GlobalAlloc`. We only add atomic bookkeeping around it.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: caller guarantees `layout` is valid (non-zero size, power-of-two align).
        let ptr = unsafe { self.inner.alloc(layout) };
        if !ptr.is_null() {
            LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: caller guarantees `ptr` was returned by `alloc` with the same layout.
        unsafe { self.inner.dealloc(ptr, layout) };
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
        DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}

/// Install `TrackingAllocator` as the global allocator.
///
/// Note: a single binary can have only one `#[global_allocator]`. In a
/// library the annotation is placed here for demonstration; a real-world
/// crate would leave the choice to the binary crate.
#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

// ── Part 2: Bump Allocator (no-std compatible, fixed buffer) ─────────────────

/// A very simple bump allocator backed by a fixed-size byte array.
///
/// Allocations are given out sequentially from the buffer. Deallocation is a
/// no-op — memory is reclaimed only when the arena is reset or dropped.
///
/// This is deliberately not installed as a `#[global_allocator]` in this
/// example because we already have `TrackingAllocator` above; it is used
/// standalone via its own `alloc_bytes` / `reset` API.
pub struct BumpAllocator<const N: usize> {
    buf: [u8; N],
    cursor: usize,
}

impl<const N: usize> BumpAllocator<N> {
    /// Creates a new bump allocator backed by an `N`-byte array.
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            cursor: 0,
        }
    }

    /// Allocates `size` bytes aligned to `align`.
    ///
    /// Returns `None` when the arena is exhausted.
    pub fn alloc_bytes(&mut self, size: usize, align: usize) -> Option<&mut [u8]> {
        // Align the cursor upward.
        let aligned = self.cursor.wrapping_add(align - 1) & !(align - 1);
        let end = aligned.checked_add(size)?;
        if end > N {
            return None;
        }
        self.cursor = end;
        Some(&mut self.buf[aligned..end])
    }

    /// Resets the allocator, making all memory available again.
    /// Existing slices obtained from `alloc_bytes` must **not** be used
    /// after calling `reset` — their backing memory will be reused.
    pub fn reset(&mut self) {
        self.cursor = 0;
        self.buf.iter_mut().for_each(|b| *b = 0);
    }

    /// Number of bytes already allocated (before the next reset).
    pub fn used(&self) -> usize {
        self.cursor
    }

    /// Number of bytes still available.
    pub fn remaining(&self) -> usize {
        N.saturating_sub(self.cursor)
    }
}

impl<const N: usize> Default for BumpAllocator<N> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Part 3: helper — measure heap delta ──────────────────────────────────────

/// Returns `(live_bytes_before, live_bytes_after, alloc_count_before, alloc_count_after)`.
pub fn measure_heap<F: FnOnce()>(f: F) -> (usize, usize, usize, usize) {
    let (live_before, alloc_before, _) = TrackingAllocator::snapshot();
    f();
    let (live_after, alloc_after, _) = TrackingAllocator::snapshot();
    (live_before, live_after, alloc_before, alloc_after)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // --- TrackingAllocator tests ---

    #[test]
    fn test_tracking_alloc_count_increases_on_heap_use() {
        let before = TrackingAllocator::alloc_count();
        // Allocating a Vec forces at least one `alloc` call.
        let v: Vec<u8> = (0..64).collect();
        let after = TrackingAllocator::alloc_count();
        drop(v);
        assert!(after > before, "alloc_count should increase after heap use");
    }

    #[test]
    fn test_tracking_dealloc_count_increases_on_drop() {
        let before = TrackingAllocator::dealloc_count();
        {
            let _s = String::from("hello, tracking allocator!");
        } // dropped here → dealloc called
        let after = TrackingAllocator::dealloc_count();
        assert!(after > before, "dealloc_count should increase after drop");
    }

    #[test]
    fn test_live_bytes_returns_to_lower_value_after_drop() {
        let live_before = TrackingAllocator::live_bytes();
        let v: Vec<u8> = vec![0u8; 1024];
        let live_during = TrackingAllocator::live_bytes();
        drop(v);
        let live_after = TrackingAllocator::live_bytes();

        // live_during should be >= live_before + 1024 (could be more due to capacity)
        assert!(
            live_during >= live_before + 1024,
            "live bytes should grow by at least 1024 while Vec is alive"
        );
        // After drop, live bytes should have decreased
        assert!(
            live_after < live_during,
            "live bytes should decrease after Vec is dropped"
        );
    }

    #[test]
    fn test_measure_heap_captures_allocation_delta() {
        let (_, _, alloc_before, alloc_after) = measure_heap(|| {
            let _v: Vec<i32> = (0..128).collect();
            // _v is dropped at end of closure
        });
        assert!(
            alloc_after > alloc_before,
            "measure_heap should capture allocation activity"
        );
    }

    // --- BumpAllocator tests ---

    #[test]
    fn test_bump_alloc_basic() {
        let mut arena = BumpAllocator::<256>::new();
        let slice = arena.alloc_bytes(16, 1).expect("should allocate 16 bytes");
        assert_eq!(slice.len(), 16);
        assert_eq!(arena.used(), 16);
        assert_eq!(arena.remaining(), 240);
    }

    #[test]
    fn test_bump_alloc_alignment() {
        let mut arena = BumpAllocator::<256>::new();
        // Allocate 1 byte to misalign cursor
        let _ = arena.alloc_bytes(1, 1);
        assert_eq!(arena.used(), 1);

        // Allocate 8 bytes with 8-byte alignment — cursor must jump to offset 8
        let slice = arena
            .alloc_bytes(8, 8)
            .expect("aligned allocation should succeed");
        assert_eq!(slice.len(), 8);
        // cursor is now at the end of the aligned region
        assert_eq!(arena.used(), 16); // 1 → aligned to 8 → +8 = 16
    }

    #[test]
    fn test_bump_alloc_exhaustion_returns_none() {
        let mut arena = BumpAllocator::<32>::new();
        // Use up the entire buffer
        let _ = arena.alloc_bytes(32, 1).expect("should fit exactly");
        // Next allocation must fail
        assert!(
            arena.alloc_bytes(1, 1).is_none(),
            "exhausted arena must return None"
        );
    }

    #[test]
    fn test_bump_alloc_reset_reuses_memory() {
        let mut arena = BumpAllocator::<64>::new();
        let _ = arena.alloc_bytes(64, 1).expect("first fill");
        assert_eq!(arena.remaining(), 0);

        arena.reset();
        assert_eq!(arena.used(), 0);
        assert_eq!(arena.remaining(), 64);

        // Should be able to allocate again after reset
        let slice = arena
            .alloc_bytes(32, 1)
            .expect("should allocate after reset");
        assert_eq!(slice.len(), 32);
    }

    #[test]
    fn test_bump_alloc_write_and_read() {
        let mut arena = BumpAllocator::<128>::new();
        let slice = arena.alloc_bytes(4, 1).expect("4 bytes");
        slice.copy_from_slice(&[1, 2, 3, 4]);
        assert_eq!(slice, &[1, 2, 3, 4]);
    }
}
