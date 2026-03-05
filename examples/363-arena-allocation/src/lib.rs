//! Arena / Bump Allocation Pattern
//!
//! Allocate many objects into a single memory region and free them all at once.

use std::cell::Cell;

// === Approach 1: Simple Bump Allocator ===

/// A simple bump allocator arena
pub struct Arena {
    data: Vec<u8>,
    offset: Cell<usize>,
    allocations: Cell<usize>,
}

impl Arena {
    /// Create a new arena with the given capacity in bytes
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0u8; capacity],
            offset: Cell::new(0),
            allocations: Cell::new(0),
        }
    }

    /// Allocate space for bytes with given size and alignment
    /// Returns the offset into the arena, or None if out of space
    pub fn alloc_bytes(&self, size: usize, align: usize) -> Option<usize> {
        let offset = self.offset.get();
        let aligned = (offset + align - 1) & !(align - 1);
        let new_offset = aligned + size;
        if new_offset > self.data.len() {
            return None;
        }
        self.offset.set(new_offset);
        self.allocations.set(self.allocations.get() + 1);
        Some(aligned)
    }

    /// Get the number of bytes currently allocated
    pub fn allocated(&self) -> usize {
        self.offset.get()
    }

    /// Get the number of allocations made
    pub fn allocation_count(&self) -> usize {
        self.allocations.get()
    }

    /// Reset the arena, freeing all allocations at once
    pub fn reset(&self) {
        self.offset.set(0);
        self.allocations.set(0);
    }

    /// Get the total capacity of the arena
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Get the remaining space in the arena
    pub fn remaining(&self) -> usize {
        self.capacity() - self.allocated()
    }

    /// Get the utilization ratio (0.0 to 1.0)
    pub fn utilization(&self) -> f64 {
        self.allocated() as f64 / self.capacity() as f64
    }
}

// === Approach 2: Typed Arena ===

/// A typed arena that stores values of a single type
pub struct TypedArena<T> {
    items: Vec<Box<T>>,
}

impl<T> TypedArena<T> {
    /// Create a new empty typed arena
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Create a typed arena with pre-allocated capacity
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
        }
    }

    /// Allocate a value in the arena, returning a reference
    pub fn alloc(&mut self, val: T) -> &T {
        self.items.push(Box::new(val));
        self.items.last().unwrap()
    }

    /// Get the count of allocated items
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Check if the arena is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all allocations
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T> Default for TypedArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

// === Approach 3: Scoped Arena Pattern ===

/// Execute a function with a fresh arena, automatically freed afterwards
pub fn with_arena<T, F>(capacity: usize, f: F) -> T
where
    F: FnOnce(&Arena) -> T,
{
    let arena = Arena::new(capacity);
    let result = f(&arena);
    // arena is automatically dropped here, freeing all memory
    result
}

/// Execute a function with a typed arena
pub fn with_typed_arena<T, R, F>(f: F) -> R
where
    F: FnOnce(&mut TypedArena<T>) -> R,
{
    let mut arena = TypedArena::new();
    let result = f(&mut arena);
    // arena is automatically dropped here
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_alloc_basic() {
        let arena = Arena::new(256);
        let o1 = arena.alloc_bytes(8, 8).unwrap();
        let o2 = arena.alloc_bytes(8, 8).unwrap();
        assert_eq!(o1, 0);
        assert_eq!(o2, 8);
        assert_eq!(arena.allocation_count(), 2);
    }

    #[test]
    fn test_alignment() {
        let arena = Arena::new(256);
        arena.alloc_bytes(1, 1).unwrap(); // offset now 1
        let aligned = arena.alloc_bytes(8, 8).unwrap();
        assert_eq!(aligned, 8); // aligned to 8-byte boundary
    }

    #[test]
    fn test_reset_clears() {
        let arena = Arena::new(64);
        arena.alloc_bytes(32, 1).unwrap();
        assert_eq!(arena.allocated(), 32);
        arena.reset();
        assert_eq!(arena.allocated(), 0);
        assert_eq!(arena.allocation_count(), 0);
    }

    #[test]
    fn test_out_of_space() {
        let arena = Arena::new(16);
        assert!(arena.alloc_bytes(8, 1).is_some());
        assert!(arena.alloc_bytes(8, 1).is_some());
        assert!(arena.alloc_bytes(1, 1).is_none()); // out of space
    }

    #[test]
    fn test_utilization() {
        let arena = Arena::new(100);
        arena.alloc_bytes(50, 1).unwrap();
        assert!((arena.utilization() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_typed_arena_basic() {
        let mut arena: TypedArena<i32> = TypedArena::new();
        let v = arena.alloc(42);
        assert_eq!(*v, 42);
        assert_eq!(arena.count(), 1);
    }

    #[test]
    fn test_typed_arena_strings() {
        let mut arena: TypedArena<String> = TypedArena::new();
        let s1 = arena.alloc("hello".to_string());
        let s2 = arena.alloc("world".to_string());
        assert_eq!(s1, "hello");
        assert_eq!(s2, "world");
        assert_eq!(arena.count(), 2);
    }

    #[test]
    fn test_typed_arena_clear() {
        let mut arena: TypedArena<i32> = TypedArena::new();
        arena.alloc(1);
        arena.alloc(2);
        arena.alloc(3);
        assert_eq!(arena.count(), 3);
        arena.clear();
        assert!(arena.is_empty());
    }

    #[test]
    fn test_with_arena_scoped() {
        let result = with_arena(1024, |arena| {
            let _ = arena.alloc_bytes(100, 1);
            let _ = arena.alloc_bytes(200, 1);
            arena.allocation_count()
        });
        assert_eq!(result, 2);
        // arena is freed after this point
    }

    #[test]
    fn test_remaining_space() {
        let arena = Arena::new(100);
        assert_eq!(arena.remaining(), 100);
        arena.alloc_bytes(40, 1).unwrap();
        assert_eq!(arena.remaining(), 60);
    }
}
