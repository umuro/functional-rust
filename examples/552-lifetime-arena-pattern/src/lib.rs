#![allow(clippy::all)]
//! Arena Allocation Pattern
//!
//! All allocations tied to arena lifetime.

/// Simple arena for strings.
pub struct StringArena {
    storage: Vec<String>,
}

impl StringArena {
    pub fn new() -> Self {
        StringArena {
            storage: Vec::new(),
        }
    }

    pub fn alloc(&mut self, s: &str) -> &str {
        self.storage.push(s.to_string());
        self.storage.last().unwrap()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

impl Default for StringArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Typed arena.
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena { items: Vec::new() }
    }

    pub fn alloc(&mut self, item: T) -> &T {
        self.items.push(item);
        self.items.last().unwrap()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_arena() {
        let mut arena = StringArena::new();
        arena.alloc("hello");
        arena.alloc("world");
        assert_eq!(arena.len(), 2);
        // Note: can't keep refs across alloc calls with &mut self
    }

    #[test]
    fn test_typed_arena() {
        let mut arena: Arena<i32> = Arena::new();
        arena.alloc(1);
        arena.alloc(2);
        // Refs would require interior mutability for real arena
    }
}
