#![allow(clippy::all)]
//! # Stack Allocation Patterns
//!
//! Demonstrates how to keep small, fixed-size data on the stack to avoid
//! heap allocator overhead, gain automatic cleanup, and maximise L1-cache
//! locality.

// ── Fixed-size stack arrays ───────────────────────────────────────────────────

/// Sum a fixed-size array that lives entirely in the stack frame.
/// No allocator call, no pointer indirection.
pub fn sum_stack_array() -> f64 {
    let data: [f64; 16] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    data.iter().copied().sum()
}

/// 4×4 matrix multiply — all 96 floats live on the stack.
/// No allocator, no pointer chasing; everything fits in L1 cache.
pub fn matmul4(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut c = [[0.0f32; 4]; 4];
    for i in 0..4 {
        for k in 0..4 {
            for j in 0..4 {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

// ── Inline string buffer (stack-allocated) ────────────────────────────────────

/// A fixed-capacity string buffer backed by a stack array.
///
/// Holds up to `CAP` bytes of UTF-8. No heap allocation whatsoever.
/// Push ASCII bytes; `as_str()` returns a `&str` slice of the filled portion.
pub struct InlineStr<const CAP: usize> {
    buf: [u8; CAP],
    len: usize,
}

impl<const CAP: usize> InlineStr<CAP> {
    pub const fn new() -> Self {
        Self {
            buf: [0u8; CAP],
            len: 0,
        }
    }

    /// Append a string slice. Returns `false` if there is not enough room.
    pub fn push_str(&mut self, s: &str) -> bool {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > CAP {
            return false;
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        true
    }

    /// View the filled portion as a `&str`.
    pub fn as_str(&self) -> &str {
        // Safety: we only accept valid UTF-8 through `push_str`.
        std::str::from_utf8(&self.buf[..self.len]).expect("always valid UTF-8")
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        CAP
    }
}

impl<const CAP: usize> Default for InlineStr<CAP> {
    fn default() -> Self {
        Self::new()
    }
}

// ── ArrayVec — stack-backed push/pop collection ───────────────────────────────

/// A fixed-capacity vector whose storage is a stack-allocated array.
///
/// Provides `push` / `pop` / `as_slice` without any heap allocation.
/// Useful when you need to accumulate a small, bounded number of items.
pub struct ArrayVec<T, const CAP: usize> {
    // `MaybeUninit` lets us avoid requiring `T: Default` or `T: Copy`.
    data: [std::mem::MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    pub fn new() -> Self {
        Self {
            // SAFETY: an array of `MaybeUninit` is always safe to initialise
            // this way — we never read uninitialised slots.
            data: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Push an element. Returns `Err(value)` if capacity is exhausted.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == CAP {
            return Err(value);
        }
        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// Pop the last element, if any.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: slot `self.len` was initialised by a prior `push`.
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: the first `self.len` slots are initialised.
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<T>(), self.len) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        CAP
    }
}

impl<T, const CAP: usize> Default for ArrayVec<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAP: usize> Drop for ArrayVec<T, CAP> {
    fn drop(&mut self) {
        // Drop every initialised element to avoid leaking resources.
        for slot in &mut self.data[..self.len] {
            // SAFETY: slots 0..len are initialised.
            unsafe { slot.assume_init_drop() };
        }
    }
}

// ── Stack-local ring buffer ───────────────────────────────────────────────────

/// A fixed-capacity ring buffer backed by a stack array.
///
/// When full, the oldest element is overwritten (lossy, like a hardware FIFO).
pub struct RingBuf<T: Copy + Default, const CAP: usize> {
    buf: [T; CAP],
    head: usize, // index of the next write slot
    count: usize,
}

impl<T: Copy + Default, const CAP: usize> RingBuf<T, CAP> {
    pub fn new() -> Self {
        Self {
            buf: [T::default(); CAP], // `Default` required so we can init the array
            head: 0,
            count: 0,
        }
    }

    /// Write one element, overwriting the oldest if full.
    pub fn push(&mut self, value: T) {
        self.buf[self.head % CAP] = value;
        self.head = (self.head + 1) % CAP;
        self.count = (self.count + 1).min(CAP);
    }

    /// Collect the current contents in insertion order (oldest first).
    pub fn to_vec(&self) -> Vec<T> {
        if self.count < CAP || self.head == 0 {
            self.buf[..self.count].to_vec()
        } else {
            let mut out = Vec::with_capacity(CAP);
            out.extend_from_slice(&self.buf[self.head..]);
            out.extend_from_slice(&self.buf[..self.head]);
            out
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<T: Copy + Default, const CAP: usize> Default for RingBuf<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

// ── const-fn new for InlineStr (pure const, no Default call) — already const ──

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── sum_stack_array ───────────────────────────────────────────────────────

    #[test]
    fn test_sum_stack_array() {
        // 1 + 2 + … + 16 = 136
        assert_eq!(sum_stack_array(), 136.0);
    }

    // ── matmul4 ───────────────────────────────────────────────────────────────

    #[test]
    fn test_matmul4_identity() {
        let id = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0f32],
        ];
        assert_eq!(matmul4(&a, &id), a);
        assert_eq!(matmul4(&id, &a), a);
    }

    #[test]
    fn test_matmul4_zero() {
        let zero = [[0.0f32; 4]; 4];
        let a = [[1.0f32; 4]; 4];
        assert_eq!(matmul4(&a, &zero), zero);
    }

    // ── InlineStr ─────────────────────────────────────────────────────────────

    #[test]
    fn test_inline_str_basic() {
        let mut s = InlineStr::<32>::new();
        assert!(s.is_empty());
        assert!(s.push_str("hello, "));
        assert!(s.push_str("world"));
        assert_eq!(s.as_str(), "hello, world");
        assert_eq!(s.len(), 12);
    }

    #[test]
    fn test_inline_str_overflow_rejected() {
        let mut s = InlineStr::<8>::new();
        assert!(s.push_str("12345678")); // exactly 8 bytes — fits
        assert!(!s.push_str("x")); // one more — rejected
        assert_eq!(s.as_str(), "12345678");
    }

    #[test]
    fn test_inline_str_empty() {
        let s = InlineStr::<16>::new();
        assert!(s.is_empty());
        assert_eq!(s.as_str(), "");
        assert_eq!(s.capacity(), 16);
    }

    // ── ArrayVec ──────────────────────────────────────────────────────────────

    #[test]
    fn test_arrayvec_push_pop() {
        let mut v = ArrayVec::<i32, 4>::new();
        assert!(v.push(10).is_ok());
        assert!(v.push(20).is_ok());
        assert!(v.push(30).is_ok());
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[10, 20, 30]);
        assert_eq!(v.pop(), Some(30));
        assert_eq!(v.pop(), Some(20));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_arrayvec_capacity_enforced() {
        let mut v = ArrayVec::<u8, 2>::new();
        assert!(v.push(1).is_ok());
        assert!(v.push(2).is_ok());
        assert_eq!(v.push(3), Err(3)); // full — rejected
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_arrayvec_empty_pop() {
        let mut v = ArrayVec::<String, 4>::new();
        assert_eq!(v.pop(), None);
        assert!(v.is_empty());
    }

    #[test]
    fn test_arrayvec_drops_elements() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let counter = Arc::new(AtomicUsize::new(0));

        struct Counted(Arc<AtomicUsize>);
        impl Drop for Counted {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }

        {
            let mut v = ArrayVec::<Counted, 4>::new();
            let _ = v.push(Counted(Arc::clone(&counter)));
            let _ = v.push(Counted(Arc::clone(&counter)));
            // v drops here
        }
        assert_eq!(counter.load(Ordering::Relaxed), 2);
    }

    // ── RingBuf ───────────────────────────────────────────────────────────────

    #[test]
    fn test_ringbuf_basic() {
        let mut rb = RingBuf::<i32, 4>::new();
        rb.push(1);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_ringbuf_wraps_around() {
        let mut rb = RingBuf::<i32, 3>::new();
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4); // overwrites 1
                    // Oldest remaining is 2
        let v = rb.to_vec();
        assert_eq!(v.len(), 3);
        assert!(v.contains(&2));
        assert!(v.contains(&3));
        assert!(v.contains(&4));
        assert!(!v.contains(&1));
    }

    #[test]
    fn test_ringbuf_empty() {
        let rb = RingBuf::<u8, 8>::new();
        assert!(rb.is_empty());
        assert_eq!(rb.to_vec(), Vec::<u8>::new());
    }
}
