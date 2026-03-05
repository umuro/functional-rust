//! 718 — Soundness, Undefined Behaviour, and Safety Invariants
//!
//! An `unsafe` block is not the end of the story — it is the beginning of a
//! responsibility. *Soundness* means no safe caller can ever produce undefined
//! behaviour by using your API, regardless of what sequence of valid safe calls
//! they make.
//!
//! Four patterns are demonstrated here:
//!
//! 1. **`SortedVec`** — invariant enforced through encapsulation (private field).
//! 2. **`NonEmpty`** — type-level proof that a collection is never empty.
//! 3. **`checked_index`** — safe wrapper around raw pointer arithmetic with
//!    `// SAFETY:` justification.
//! 4. **`split_at_mut_demo`** — non-overlapping mutable sub-slices from one
//!    allocation, the canonical pattern for defeating the borrow checker
//!    soundly.

// ── Pattern 1: SortedVec ─────────────────────────────────────────────────────

/// A `Vec<T>` whose elements are always sorted in ascending order.
///
/// # Invariant
/// `self.0[i] <= self.0[j]` for all `i < j`.
///
/// The inner `Vec` is private, so **no safe code** can break this invariant —
/// only `SortedVec`'s own methods touch the storage, and each preserves order.
pub struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Insert `val` at the correct sorted position (binary search → O(log n)).
    pub fn insert(&mut self, val: T) {
        // `partition_point` returns the first index where the predicate is false.
        // For a "less-than-or-equal" predicate this gives stable sort order.
        let pos = self.0.partition_point(|x| x <= &val);
        self.0.insert(pos, val);
    }

    /// View the underlying sorted slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// O(log n) search — valid only because the invariant guarantees sorted order.
    pub fn contains(&self, val: &T) -> bool {
        self.0.binary_search(val).is_ok()
    }
}

impl<T: Ord> Default for SortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Pattern 2: NonEmpty ───────────────────────────────────────────────────────

/// A non-empty collection.
///
/// # Invariant
/// `len() >= 1` always holds. `first()` and `last()` therefore return `&T`,
/// not `Option<&T>` — the type system carries the proof.
pub struct NonEmpty<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmpty<T> {
    /// The only constructor — always produces a valid, non-empty collection.
    pub fn singleton(val: T) -> Self {
        Self {
            head: val,
            tail: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.tail.push(val);
    }

    /// Always succeeds. The invariant (len >= 1) is maintained by construction.
    pub fn first(&self) -> &T {
        &self.head
    }

    /// Always succeeds for the same reason.
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }

    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }

    /// `NonEmpty` is never empty; this always returns `false`.
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }
}

// ── Pattern 3: Safe wrapper around raw pointer arithmetic ─────────────────────

/// Read a value at `offset` into `slice` using raw pointer arithmetic.
///
/// This exists to demonstrate the `// SAFETY:` comment convention — in
/// production code you would simply write `&slice[offset]`.
///
/// # Safety
/// `offset` must be strictly less than `slice.len()`.
///
/// # Undefined Behaviour if violated
/// Forming a reference to memory outside the slice is UB: the pointer may
/// be invalid, it may alias other live references, and the compiler is free
/// to assume it never happens.
pub unsafe fn raw_index<T>(slice: &[T], offset: usize) -> &T {
    // SAFETY: The caller guarantees `offset < slice.len()`, so
    // `slice.as_ptr().add(offset)` is within the allocation and the lifetime
    // of the returned reference is bounded by `slice`'s borrow.
    &*slice.as_ptr().add(offset)
}

/// A checked, **sound** wrapper around `raw_index` — callable from safe code.
///
/// The bounds check here is what converts an *unsafe* operation into a *safe*
/// API: the caller can never trigger UB because we verify the precondition
/// ourselves before delegating to the unsafe function.
pub fn checked_index<T>(slice: &[T], offset: usize) -> Option<&T> {
    if offset < slice.len() {
        // SAFETY: `offset < slice.len()` is checked on the line above.
        Some(unsafe { raw_index(slice, offset) })
    } else {
        None
    }
}

// ── Pattern 4: Non-overlapping mutable sub-slices ────────────────────────────

/// Split `slice` at `mid`, returning two **non-overlapping** mutable sub-slices.
///
/// This mirrors the stdlib's `<[T]>::split_at_mut` and is the canonical example
/// of using `unsafe` to defeat the borrow checker *soundly*: the borrow checker
/// cannot prove the two halves do not alias, but we can by construction.
///
/// # Panics
/// Panics if `mid > slice.len()`.
pub fn split_at_mut_demo<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    assert!(mid <= slice.len(), "mid ({mid}) > len ({})", slice.len());
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    // SAFETY:
    //   • `ptr` is valid for `len` elements — it is derived from a live `&mut [T]`.
    //   • `mid <= len` is asserted above, so both sub-ranges stay within bounds.
    //   • Left sub-slice covers `[ptr, ptr+mid)` and right covers `[ptr+mid, ptr+len)`;
    //     the ranges are disjoint, so the two `&mut` slices do not alias.
    //   • Pointer arithmetic preserves alignment because we add a multiple of
    //     `size_of::<T>()` to a `*mut T`.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── SortedVec tests ───────────────────────────────────────────────────────

    #[test]
    fn sorted_vec_empty_is_sorted() {
        let sv: SortedVec<i32> = SortedVec::new();
        assert!(sv.is_empty());
        assert_eq!(sv.as_slice(), &[] as &[i32]);
    }

    #[test]
    fn sorted_vec_maintains_invariant_after_random_inserts() {
        let mut sv = SortedVec::new();
        for &n in &[5, 1, 8, 3, 7, 2, 6, 4] {
            sv.insert(n);
        }
        let s = sv.as_slice();
        // Every adjacent pair must be in non-decreasing order.
        assert!(s.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(s, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sorted_vec_contains_uses_binary_search() {
        let mut sv = SortedVec::new();
        sv.insert(10);
        sv.insert(20);
        sv.insert(30);
        assert!(sv.contains(&10));
        assert!(sv.contains(&20));
        assert!(sv.contains(&30));
        assert!(!sv.contains(&15));
    }

    #[test]
    fn sorted_vec_allows_duplicates_in_stable_order() {
        let mut sv = SortedVec::new();
        sv.insert(2);
        sv.insert(2);
        sv.insert(1);
        assert_eq!(sv.as_slice(), &[1, 2, 2]);
    }

    // ── NonEmpty tests ────────────────────────────────────────────────────────

    #[test]
    fn non_empty_singleton_first_and_last_same() {
        let ne = NonEmpty::singleton(42_i32);
        assert_eq!(ne.first(), &42);
        assert_eq!(ne.last(), &42);
        assert_eq!(ne.len(), 1);
    }

    #[test]
    fn non_empty_push_updates_last() {
        let mut ne = NonEmpty::singleton("alpha");
        ne.push("beta");
        ne.push("gamma");
        assert_eq!(ne.first(), &"alpha");
        assert_eq!(ne.last(), &"gamma");
        assert_eq!(ne.len(), 3);
    }

    #[test]
    fn non_empty_iter_yields_all_elements() {
        let mut ne = NonEmpty::singleton(1_u32);
        ne.push(2);
        ne.push(3);
        let collected: Vec<u32> = ne.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    // ── checked_index tests ───────────────────────────────────────────────────

    #[test]
    fn checked_index_returns_some_for_valid_offset() {
        let data = [10, 20, 30, 40];
        assert_eq!(checked_index(&data, 0), Some(&10));
        assert_eq!(checked_index(&data, 3), Some(&40));
    }

    #[test]
    fn checked_index_returns_none_for_out_of_bounds() {
        let data = [1, 2, 3];
        assert_eq!(checked_index(&data, 3), None);
        assert_eq!(checked_index(&data, 100), None);
    }

    #[test]
    fn checked_index_empty_slice_always_none() {
        let data: [i32; 0] = [];
        assert_eq!(checked_index(&data, 0), None);
    }

    // ── split_at_mut_demo tests ───────────────────────────────────────────────

    #[test]
    fn split_at_mut_demo_halves_are_independent() {
        let mut data = [1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut_demo(&mut data, 3);
        left.iter_mut().for_each(|x| *x *= 10);
        right.iter_mut().for_each(|x| *x += 100);
        assert_eq!(data, [10, 20, 30, 104, 105, 106]);
    }

    #[test]
    fn split_at_mut_demo_mid_zero_yields_empty_left() {
        let mut data = [1, 2, 3];
        let (left, right) = split_at_mut_demo(&mut data, 0);
        assert!(left.is_empty());
        assert_eq!(right, &mut [1, 2, 3]);
    }

    #[test]
    fn split_at_mut_demo_mid_len_yields_empty_right() {
        let mut data = [1, 2, 3];
        let len = data.len();
        let (left, right) = split_at_mut_demo(&mut data, len);
        assert_eq!(left, &mut [1, 2, 3]);
        assert!(right.is_empty());
    }

    #[test]
    #[should_panic(expected = "mid")]
    fn split_at_mut_demo_panics_on_out_of_bounds_mid() {
        let mut data = [1, 2];
        let _ = split_at_mut_demo(&mut data, 5);
    }
}
