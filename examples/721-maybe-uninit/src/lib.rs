#![allow(clippy::all)]
//! # 721: `MaybeUninit` — Safe Uninitialized Memory
//!
//! `MaybeUninit<T>` lets you allocate memory without initialising it.
//! The type makes the contract explicit: `.write()` initialises a slot,
//! `.assume_init()` (unsafe) asserts every byte is valid.

use std::mem::MaybeUninit;

// ── Pattern 1: Single-value "C output parameter" ──────────────────────────────

/// Writes a computed value into a caller-provided `MaybeUninit` slot —
/// the canonical "output parameter" pattern seen in C FFI.
pub fn fill_value(out: &mut MaybeUninit<u32>, x: u32) {
    out.write(x * 2);
}

/// Allocate a single uninitialised slot, write into it, then read back.
pub fn single_value_demo() -> u32 {
    let mut slot = MaybeUninit::<u32>::uninit();
    fill_value(&mut slot, 21);
    // SAFETY: `fill_value` unconditionally calls `.write()`, so the slot
    // is fully initialised before we call `assume_init`.
    unsafe { slot.assume_init() }
}

// ── Pattern 2: Fixed-size array built element-by-element ──────────────────────

/// Build a `[u32; N]` by initialising each element individually,
/// avoiding the `Default` bound that `[T; N]` initialisation would require.
pub fn build_array<const N: usize>(f: impl Fn(usize) -> u32) -> [u32; N] {
    // Allocate an array of uninitialised slots.
    let mut arr: [MaybeUninit<u32>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    for (i, slot) in arr.iter_mut().enumerate() {
        slot.write(f(i));
    }

    // SAFETY: Every element has been written by the loop above.
    // We transmute the fully-initialised `[MaybeUninit<u32>; N]` to `[u32; N]`.
    //
    // Using `ptr::read` + cast is the standard pattern pre-1.80;
    // from 1.80+ `MaybeUninit::array_assume_init` can be used instead.
    unsafe { std::mem::transmute_copy(&arr) }
}

// ── Pattern 3: Partial fill tracked by index ─────────────────────────────────

/// A buffer that tracks how many slots have been initialised.
/// Elements 0..len are valid; elements len..CAP are uninitialised.
pub struct PartialBuf<T, const CAP: usize> {
    data: [MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> PartialBuf<T, CAP> {
    pub fn new() -> Self {
        Self {
            // SAFETY: An array of `MaybeUninit` is always safe to "uninit".
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Push a value into the next slot. Returns `false` if full.
    pub fn push(&mut self, value: T) -> bool {
        if self.len >= CAP {
            return false;
        }
        self.data[self.len].write(value);
        self.len += 1;
        true
    }

    /// Return a slice over the initialised portion.
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: `data[0..len]` has been written by `push`.
        unsafe { &*(std::ptr::slice_from_raw_parts(self.data.as_ptr().cast::<T>(), self.len)) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T, const CAP: usize> Default for PartialBuf<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAP: usize> Drop for PartialBuf<T, CAP> {
    fn drop(&mut self) {
        // SAFETY: `data[0..len]` are initialised; drop them in place.
        for slot in &mut self.data[..self.len] {
            unsafe { slot.assume_init_drop() };
        }
    }
}

// ── Pattern 4: Idiomatic safe wrapper (no unsafe exposed) ────────────────────

/// Zero-cost helper: initialise a `MaybeUninit<T>` via a closure and
/// return the initialised value — entirely in safe code for the caller.
pub fn init_with<T>(f: impl FnOnce(&mut MaybeUninit<T>)) -> T {
    let mut slot = MaybeUninit::<T>::uninit();
    f(&mut slot);
    // SAFETY: caller's closure is required to call `.write()` before returning.
    // The doc-contract makes this clear; callers who skip the write have UB.
    unsafe { slot.assume_init() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_value_demo() {
        // 21 * 2 == 42
        assert_eq!(single_value_demo(), 42);
    }

    #[test]
    fn test_fill_value_writes_doubled() {
        let mut slot = MaybeUninit::<u32>::uninit();
        fill_value(&mut slot, 5);
        let v = unsafe { slot.assume_init() };
        assert_eq!(v, 10);
    }

    #[test]
    fn test_build_array_squares() {
        let arr: [u32; 5] = build_array(|i| (i * i) as u32);
        assert_eq!(arr, [0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_build_array_identity() {
        let arr: [u32; 3] = build_array(|i| i as u32);
        assert_eq!(arr, [0, 1, 2]);
    }

    #[test]
    fn test_partial_buf_push_and_slice() {
        let mut buf = PartialBuf::<u32, 4>::new();
        assert!(buf.is_empty());

        assert!(buf.push(10));
        assert!(buf.push(20));
        assert!(buf.push(30));

        assert_eq!(buf.len(), 3);
        assert_eq!(buf.as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn test_partial_buf_full_returns_false() {
        let mut buf = PartialBuf::<u32, 2>::new();
        assert!(buf.push(1));
        assert!(buf.push(2));
        assert!(!buf.push(3)); // full
        assert_eq!(buf.as_slice(), &[1, 2]);
    }

    #[test]
    fn test_partial_buf_with_string_drops_correctly() {
        let mut buf = PartialBuf::<String, 3>::new();
        buf.push("hello".to_owned());
        buf.push("world".to_owned());
        assert_eq!(buf.as_slice(), &["hello", "world"]);
        // Drop runs here — verifies no double-free or leak via Miri / sanitisers.
    }

    #[test]
    fn test_init_with() {
        let v = init_with(|slot| {
            slot.write(99_u32);
        });
        assert_eq!(v, 99);
    }
}
