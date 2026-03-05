//! Rc and Weak for Shared Ownership
//!
//! Reference counting with weak references.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Node in a tree with parent backpointer.
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    pub fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }

    pub fn parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }
}

/// Simple shared data.
pub fn shared_data_demo() -> (Rc<String>, Rc<String>) {
    let data = Rc::new(String::from("shared"));
    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);
    (clone1, clone2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_data() {
        let (a, b) = shared_data_demo();
        assert_eq!(*a, "shared");
        assert_eq!(*b, "shared");
        assert!(Rc::ptr_eq(&a, &b));
    }

    #[test]
    fn test_node_tree() {
        let root = Node::new(1);
        let child = Node::new(2);
        Node::add_child(&root, child.clone());

        assert_eq!(root.children.borrow().len(), 1);
        assert_eq!(child.parent().unwrap().value, 1);
    }

    #[test]
    fn test_weak_upgrade() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        assert_eq!(*weak.upgrade().unwrap(), 42);
        drop(strong);
        assert!(weak.upgrade().is_none());
    }
}
