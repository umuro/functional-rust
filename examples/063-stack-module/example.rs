// 063: Stack Module — Demonstration
// Shows both mutable and immutable stack approaches

use std::iter::FromIterator;

// Approach 1: Mutable stack wrapping Vec
#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

// Approach 2: From iterator
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Stack {
            elements: iter.into_iter().collect(),
        }
    }
}

// Approach 3: Immutable (persistent) stack
#[derive(Debug, Clone)]
enum FnStack<T: Clone> {
    Empty,
    Cons(T, Box<FnStack<T>>),
}

impl<T: Clone> FnStack<T> {
    fn empty() -> Self {
        FnStack::Empty
    }

    fn is_empty(&self) -> bool {
        matches!(self, FnStack::Empty)
    }

    fn push(&self, item: T) -> Self {
        FnStack::Cons(item, Box::new(self.clone()))
    }

    fn pop(&self) -> Option<FnStack<T>> {
        match self {
            FnStack::Empty => None,
            FnStack::Cons(_, rest) => Some(*rest.clone()),
        }
    }

    fn peek(&self) -> Option<&T> {
        match self {
            FnStack::Empty => None,
            FnStack::Cons(x, _) => Some(x),
        }
    }
}

fn main() {
    println!("=== Stack Module Demonstration ===\n");
    
    // Mutable stack demonstration
    println!("1. Mutable Stack (Vec-based):");
    let mut s = Stack::new();
    println!("   Creating empty stack: {:?}", s);
    println!("   Is empty? {}", s.is_empty());
    
    s.push(10);
    s.push(20);
    s.push(30);
    println!("   After pushing 10, 20, 30: {:?}", s);
    println!("   Peek: {:?}", s.peek());
    println!("   Size: {}", s.size());
    
    println!("   Popping...");
    while let Some(val) = s.pop() {
        println!("     Popped: {}", val);
    }
    println!("   Stack empty: {}", s.is_empty());
    
    // FromIterator demonstration
    println!("\n2. Stack from iterator:");
    let s2: Stack<i32> = vec![100, 200, 300].into_iter().collect();
    println!("   Created from vec![100, 200, 300]: {:?}", s2);
    println!("   Peek: {:?}", s2.peek());
    
    // Immutable stack demonstration
    println!("\n3. Immutable Stack (persistent):");
    let fs = FnStack::empty();
    println!("   Empty immutable stack: {:?}", fs);
    
    let fs1 = fs.push(42);
    let fs2 = fs1.push(84);
    let fs3 = fs2.push(126);
    
    println!("   After three pushes:");
    println!("     fs1 (pushed 42): {:?}", fs1);
    println!("     fs2 (pushed 84): {:?}", fs2);
    println!("     fs3 (pushed 126): {:?}", fs3);
    
    println!("   Peek at fs3: {:?}", fs3.peek());
    
    let fs4 = fs3.pop().unwrap();
    println!("   After popping fs3 (fs4): {:?}", fs4);
    println!("   Peek at fs4: {:?}", fs4.peek());
    
    // Original immutable stack unchanged
    println!("   Original fs3 unchanged: {:?}", fs3);
    println!("   Peek at fs3 still: {:?}", fs3.peek());
    
    println!("\n✓ All demonstrations completed successfully!");
}