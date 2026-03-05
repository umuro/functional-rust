use std::cell::Cell;

// Simple bump allocator arena
struct Arena {
    data: Vec<u8>,
    offset: Cell<usize>,
    allocations: Cell<usize>,
}

impl Arena {
    fn new(capacity: usize) -> Self {
        Self {
            data: vec![0u8; capacity],
            offset: Cell::new(0),
            allocations: Cell::new(0),
        }
    }

    // Allocate space for a value (returns index / offset)
    fn alloc_bytes(&self, size: usize, align: usize) -> Option<usize> {
        let offset = self.offset.get();
        let aligned = (offset + align - 1) & !(align - 1);
        let new_offset = aligned + size;
        if new_offset > self.data.len() { return None; }
        self.offset.set(new_offset);
        self.allocations.set(self.allocations.get() + 1);
        Some(aligned)
    }

    fn allocated(&self) -> usize { self.offset.get() }
    fn allocation_count(&self) -> usize { self.allocations.get() }
    fn reset(&self) { self.offset.set(0); self.allocations.set(0); }
    fn capacity(&self) -> usize { self.data.len() }
    fn utilization(&self) -> f64 { self.allocated() as f64 / self.capacity() as f64 }
}

// Arena allocating simple typed values by storing them in a Vec<Box<T>>
struct TypedArena<T> {
    items: Vec<Box<T>>,
}

impl<T> TypedArena<T> {
    fn new() -> Self { Self { items: Vec::new() } }
    fn alloc(&mut self, val: T) -> &T {
        self.items.push(Box::new(val));
        self.items.last().unwrap()
    }
    fn count(&self) -> usize { self.items.len() }
}

fn main() {
    let arena = Arena::new(1024);

    // Allocate several objects
    for i in 0..10 {
        let offset = arena.alloc_bytes(std::mem::size_of::<i64>(), std::mem::align_of::<i64>());
        println!("Alloc {i}: offset={offset:?}");
    }

    println!("Used: {} / {} bytes ({:.1}%)", arena.allocated(), arena.capacity(), arena.utilization()*100.0);
    println!("Allocations: {}", arena.allocation_count());
    arena.reset();
    println!("After reset: {} bytes used", arena.allocated());

    // Typed arena
    let mut typed: TypedArena<String> = TypedArena::new();
    for i in 0..5 {
        let s = typed.alloc(format!("string-{i}"));
        println!("Allocated: {s}");
    }
    println!("Typed arena count: {}", typed.count());
    // All strings freed when typed drops here
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn bump_alloc() {
        let a = Arena::new(256);
        let o1 = a.alloc_bytes(8, 8).unwrap();
        let o2 = a.alloc_bytes(8, 8).unwrap();
        assert_eq!(o1, 0); assert_eq!(o2, 8);
    }
    #[test] fn reset_clears() {
        let a = Arena::new(64);
        a.alloc_bytes(32, 1).unwrap();
        a.reset();
        assert_eq!(a.allocated(), 0);
    }
    #[test] fn typed_arena() {
        let mut a: TypedArena<i32> = TypedArena::new();
        let v = a.alloc(42); assert_eq!(*v, 42);
        assert_eq!(a.count(), 1);
    }
}
