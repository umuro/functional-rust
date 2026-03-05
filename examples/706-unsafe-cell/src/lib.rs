//! 706 — UnsafeCell: The Foundation of Interior Mutability
//!
//! `UnsafeCell<T>` is the only primitive that lets you mutate data through
//! a shared reference (`&T`) without invoking undefined behaviour.
//! Every interior-mutability type in std — `Cell`, `RefCell`, `Mutex` — is
//! built on top of it.  Here we build a `Cell<T>`-like type from scratch
//! to expose the mechanics.

use std::cell::UnsafeCell;

// ─── MyCell ──────────────────────────────────────────────────────────────────

/// A single-threaded mutable cell built directly on `UnsafeCell<T>`.
///
/// Mirrors `std::cell::Cell<T>` in behaviour but exposes the raw
/// `UnsafeCell` machinery so the seams are visible.
///
/// `UnsafeCell<T>` is `!Sync`, so `MyCell<T>` inherits `!Sync`
/// automatically — it cannot be shared across threads.
pub struct MyCell<T> {
    inner: UnsafeCell<T>,
}

impl<T: Copy> MyCell<T> {
    /// Wrap a value in the cell.
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// Replace the stored value.
    ///
    /// # Safety rationale
    /// `MyCell` is `!Sync`, so only one thread can hold a reference.
    /// `UnsafeCell` disables the compiler's "shared ref ⇒ frozen memory"
    /// assumption, making the raw-pointer write defined behaviour.
    pub fn set(&self, value: T) {
        // SAFETY: single-threaded (MyCell: !Sync), no aliased mutable refs.
        unsafe { *self.inner.get() = value }
    }

    /// Return a copy of the stored value.
    pub fn get(&self) -> T {
        // SAFETY: same guarantee as `set`; we only read, so no data race.
        unsafe { *self.inner.get() }
    }

    /// Apply a function to the stored value and write the result back.
    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let v = self.get();
        self.set(f(v));
    }
}

// ─── MyOnceCell ──────────────────────────────────────────────────────────────

/// A write-once cell: after the first `set`, all subsequent writes are
/// silently ignored.  Demonstrates a different usage pattern for `UnsafeCell`.
pub struct MyOnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}

impl<T> MyOnceCell<T> {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    /// Store `value` if the cell is still empty; return `false` otherwise.
    pub fn set(&self, value: T) -> bool {
        // SAFETY: single-threaded, no concurrent access.
        let slot = unsafe { &mut *self.inner.get() };
        if slot.is_none() {
            *slot = Some(value);
            true
        } else {
            false
        }
    }

    /// Return a reference to the value if it has been set.
    pub fn get(&self) -> Option<&T> {
        // SAFETY: we only hand out shared references; the value is never
        // mutated after initialisation, so this is sound.
        unsafe { (*self.inner.get()).as_ref() }
    }
}

impl<T> Default for MyOnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── MyCell tests ─────────────────────────────────────────────────────────

    #[test]
    fn cell_new_and_get() {
        let c = MyCell::new(42_i32);
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn cell_set_overwrites() {
        let c = MyCell::new(0_i32);
        c.set(7);
        assert_eq!(c.get(), 7);
        c.set(99);
        assert_eq!(c.get(), 99);
    }

    #[test]
    fn cell_update_accumulates() {
        let c = MyCell::new(0_i32);
        c.update(|v| v + 5);
        c.update(|v| v + 3);
        assert_eq!(c.get(), 8);
    }

    #[test]
    fn cell_mutation_through_shared_ref() {
        // Verify interior mutability: `cell` is not declared `mut`
        // yet we can write to it through a shared reference.
        let cell = MyCell::new(100_i32);
        let r: &MyCell<i32> = &cell;
        r.set(200);
        assert_eq!(cell.get(), 200);
    }

    #[test]
    fn cell_copy_types_work() {
        let c = MyCell::new(true);
        assert!(c.get());
        c.set(false);
        assert!(!c.get());
    }

    // ── MyOnceCell tests ─────────────────────────────────────────────────────

    #[test]
    fn once_cell_empty_on_creation() {
        let c = MyOnceCell::<i32>::new();
        assert!(c.get().is_none());
    }

    #[test]
    fn once_cell_first_set_succeeds() {
        let c = MyOnceCell::new();
        assert!(c.set(42_i32));
        assert_eq!(c.get(), Some(&42));
    }

    #[test]
    fn once_cell_second_set_is_ignored() {
        let c = MyOnceCell::new();
        c.set(1_i32);
        let accepted = c.set(2);
        assert!(!accepted);
        assert_eq!(c.get(), Some(&1));
    }

    #[test]
    fn once_cell_write_once_through_shared_ref() {
        let cell = MyOnceCell::new();
        let r: &MyOnceCell<&str> = &cell;
        r.set("hello");
        assert_eq!(cell.get(), Some(&"hello"));
    }
}
