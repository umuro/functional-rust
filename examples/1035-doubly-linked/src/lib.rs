#![allow(dead_code)]
#![allow(clippy::all)]
// 1035: Doubly-Linked List — Rc<RefCell<Node>> Approach
// Safe doubly-linked list using reference counting + interior mutability

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<DNode<T>>>>;

struct DNode<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DNode {
            value,
            prev: self.tail.clone(),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
        self.tail = Some(new_node);
        self.len += 1;
    }

    fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DNode {
            value,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
            }
            None => {
                self.tail = Some(new_node.clone());
            }
        }
        self.head = Some(new_node);
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T>
    where
        T: Default,
    {
        self.head.take().map(|old_head| {
            let mut old_head_ref = old_head.borrow_mut();
            match old_head_ref.next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            std::mem::take(&mut old_head_ref.value)
        })
    }

    fn pop_back(&mut self) -> Option<T>
    where
        T: Default,
    {
        self.tail.take().map(|old_tail| {
            let mut old_tail_ref = old_tail.borrow_mut();
            match old_tail_ref.prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            self.len -= 1;
            std::mem::take(&mut old_tail_ref.value)
        })
    }

    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            result.push(node.borrow().value.clone());
            current = node.borrow().next.clone();
        }
        result
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_vec())
    }
}

fn basic_operations() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(0);

    assert_eq!(list.to_vec(), vec![0, 1, 2, 3]);
    assert_eq!(list.len(), 4);

    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.to_vec(), vec![1, 2]);
}

fn bidirectional_traversal() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    for i in 1..=5 {
        list.push_back(i);
    }

    // Forward traversal
    let forward = list.to_vec();
    assert_eq!(forward, vec![1, 2, 3, 4, 5]);

    // Backward traversal
    let mut backward = Vec::new();
    let mut current = list.tail.clone();
    while let Some(node) = current {
        backward.push(node.borrow().value);
        current = node.borrow().prev.clone();
    }
    assert_eq!(backward, vec![5, 4, 3, 2, 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_operations();
    }

    #[test]
    fn test_bidirectional() {
        bidirectional_traversal();
    }

    #[test]
    fn test_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_single() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        list.push_back(42);
        assert_eq!(list.pop_front(), Some(42));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }
}
