//! # 552. Arena with Lifetimes
//! Bump allocator enabling batch allocation and deallocation.

/// Simple arena that bump-allocates into a Vec<Box<T>>
/// Returns indices instead of references to avoid borrow conflicts
struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    fn new() -> Self { Arena { items: Vec::new() } }

    /// Allocate value, return index for later retrieval
    fn alloc(&mut self, value: T) -> usize {
        self.items.push(value);
        self.items.len() - 1
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.items.get(idx)
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.items.get_mut(idx)
    }

    fn len(&self) -> usize { self.items.len() }
    fn is_empty(&self) -> bool { self.items.is_empty() }
}

/// Safe reference-returning arena using separate scope
struct BorrowArena<T>(Vec<Box<T>>);

impl<T> BorrowArena<T> {
    fn new() -> Self { BorrowArena(Vec::new()) }

    /// SAFETY: The lifetime of the returned reference is tied to &self
    /// This pattern requires that allocations happen before reads
    fn alloc_and_get(&mut self, value: T) -> &T {
        self.0.push(Box::new(value));
        self.0.last().unwrap()
    }

    fn get_all(&self) -> Vec<&T> {
        self.0.iter().map(|b| b.as_ref()).collect()
    }
}

/// AST node that borrows from source string with arena lifetime
#[derive(Debug)]
enum AstNode<'src> {
    Literal(&'src str),
    Ident(&'src str),
    BinOp {
        op: &'src str,
        left: usize,
        right: usize,
    },
}

fn main() {
    // Index-based arena (avoids borrow conflicts)
    let mut arena: Arena<i32> = Arena::new();
    let idx1 = arena.alloc(42);
    let idx2 = arena.alloc(100);
    let idx3 = arena.alloc(200);
    println!("arena has {} items", arena.len());
    println!("r1={:?}, r2={:?}", arena.get(idx1), arena.get(idx2));
    println!("r3={:?}", arena.get(idx3));

    // All allocations done — now safe to get references simultaneously
    let all: Vec<_> = (0..arena.len()).filter_map(|i| arena.get(i)).collect();
    println!("all: {:?}", all);

    // String arena
    let mut string_arena: Arena<String> = Arena::new();
    let nodes = ["start", "middle", "end", "branch"];
    let indices: Vec<usize> = nodes.iter().map(|&n| string_arena.alloc(n.to_string())).collect();
    println!("\nString arena nodes:");
    for (name, &idx) in nodes.iter().zip(indices.iter()) {
        println!("  {} -> idx={}, stored={:?}", name, idx, string_arena.get(idx));
    }

    // AST arena — nodes borrow from source string
    let source = "x + y * 42";
    let mut ast_arena: Vec<AstNode<'_>> = Vec::new();
    ast_arena.push(AstNode::Ident(&source[0..1]));
    ast_arena.push(AstNode::Ident(&source[4..5]));
    ast_arena.push(AstNode::Literal(&source[8..10]));
    ast_arena.push(AstNode::BinOp { op: "*", left: 1, right: 2 });
    ast_arena.push(AstNode::BinOp { op: "+", left: 0, right: 3 });

    println!("\nAST:");
    for (i, node) in ast_arena.iter().enumerate() {
        println!("  [{}] {:?}", i, node);
    }
    // All AST nodes freed when ast_arena drops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_alloc() {
        let mut arena: Arena<i32> = Arena::new();
        let i1 = arena.alloc(42);
        let i2 = arena.alloc(100);
        assert_eq!(arena.get(i1), Some(&42));
        assert_eq!(arena.get(i2), Some(&100));
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn test_arena_sequential_alloc() {
        let mut arena: Arena<String> = Arena::new();
        for i in 0..5 {
            arena.alloc(format!("item_{}", i));
        }
        assert_eq!(arena.len(), 5);
        assert_eq!(arena.get(2), Some(&"item_2".to_string()));
    }

    #[test]
    fn test_arena_get_oob() {
        let arena: Arena<i32> = Arena::new();
        assert_eq!(arena.get(0), None);
    }
}
