// 063: Stack Module
// Stack abstraction with struct + impl encapsulation

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

// Approach 2: Immutable (persistent) stack
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

// Approach 3: From iterator
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Stack {
            elements: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_stack() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.size(), 3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.peek(), Some(&2));
    }

    #[test]
    fn test_immutable_stack() {
        let s = FnStack::empty().push(1).push(2).push(3);
        assert_eq!(s.peek(), Some(&3));
        let s2 = s.pop().unwrap();
        assert_eq!(s2.peek(), Some(&2));
        // Original unchanged
        assert_eq!(s.peek(), Some(&3));
    }

    #[test]
    fn test_from_iter() {
        let s: Stack<i32> = vec![1, 2, 3].into_iter().collect();
        assert_eq!(s.size(), 3);
        assert_eq!(s.peek(), Some(&3));
    }
}
