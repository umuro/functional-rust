// 079: Associated Types
// type Item in trait definitions

// Approach 1: Trait with associated type
trait Container {
    type Item;
    fn new() -> Self;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
}

struct IntStack {
    elements: Vec<i32>,
}

impl Container for IntStack {
    type Item = i32;
    fn new() -> Self {
        IntStack {
            elements: Vec::new(),
        }
    }
    fn push(&mut self, item: i32) {
        self.elements.push(item);
    }
    fn pop(&mut self) -> Option<i32> {
        self.elements.pop()
    }
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

struct StringQueue {
    elements: Vec<String>,
}

impl Container for StringQueue {
    type Item = String;
    fn new() -> Self {
        StringQueue {
            elements: Vec::new(),
        }
    }
    fn push(&mut self, item: String) {
        self.elements.push(item);
    }
    fn pop(&mut self) -> Option<String> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

// Approach 2: Generic function using associated types
fn drain_all<C: Container>(container: &mut C) -> Vec<C::Item> {
    let mut result = Vec::new();
    while let Some(item) = container.pop() {
        result.push(item);
    }
    result
}

// Approach 3: Custom iterator with associated type
struct RangeIter {
    current: i32,
    stop: i32,
}

impl Iterator for RangeIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.stop {
            None
        } else {
            let v = self.current;
            self.current += 1;
            Some(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_stack() {
        let mut s = IntStack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        s.push(3);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Some(3));
    }

    #[test]
    fn test_string_queue() {
        let mut q = StringQueue::new();
        q.push("a".into());
        q.push("b".into());
        assert_eq!(q.pop(), Some("a".into()));
    }

    #[test]
    fn test_drain() {
        let mut s = IntStack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(drain_all(&mut s), vec![3, 2, 1]);
        assert!(s.is_empty());
    }

    #[test]
    fn test_range_iter() {
        let items: Vec<i32> = (RangeIter {
            current: 0,
            stop: 5,
        })
        .collect();
        assert_eq!(items, vec![0, 1, 2, 3, 4]);
    }
}
