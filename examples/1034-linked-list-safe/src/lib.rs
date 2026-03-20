#![allow(clippy::all)]
// 1034: Safe Linked List — Option<Box<Node<T>>>
// Building a singly-linked list using only safe Rust

/// A singly-linked list node
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// A singly-linked list (stack-like: push/pop at head)
#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None, len: 0 }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // Take ownership of old head
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_deref(),
        }
    }
}

/// Iterator over list references
struct ListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

/// Implement Drop to avoid stack overflow on large lists
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

fn basic_operations() {
    let mut list = List::new();
    assert!(list.is_empty());

    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek(), Some(&3));

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

fn iterator_demo() {
    let mut list = List::new();
    for i in (1..=5).rev() {
        list.push(i);
    }

    let collected: Vec<_> = list.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);

    let sum: i32 = list.iter().sum();
    assert_eq!(sum, 15);

    let evens: Vec<_> = list.iter().filter(|&&x| x % 2 == 0).copied().collect();
    assert_eq!(evens, vec![2, 4]);
}

fn from_vec() {
    let mut list = List::new();
    for &x in [5, 4, 3, 2, 1].iter() {
        list.push(x);
    }
    let items: Vec<_> = list.iter().copied().collect();
    assert_eq!(items, vec![1, 2, 3, 4, 5]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_operations();
    }

    #[test]
    fn test_iterator() {
        iterator_demo();
    }

    #[test]
    fn test_from_vec() {
        from_vec();
    }

    #[test]
    fn test_empty() {
        let list: List<i32> = List::new();
        assert!(list.is_empty());
        assert_eq!(list.peek(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_single_element() {
        let mut list = List::new();
        list.push(42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
        assert!(list.is_empty());
    }
}
