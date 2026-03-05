//! # 551. Rc and Weak for Cycles
//! Breaking reference cycles with weak references.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Tree node: parent owns children (Rc), children reference parent (Weak)
#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: Option<Weak<RefCell<TreeNode>>>,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            parent: None,
            children: Vec::new(),
        }))
    }

    fn add_child(
        parent: &Rc<RefCell<TreeNode>>,
        child: Rc<RefCell<TreeNode>>,
    ) {
        // Child gets weak ref to parent (avoids cycle)
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        // Parent owns child (strong Rc)
        parent.borrow_mut().children.push(child);
    }

    fn parent_value(&self) -> Option<i32> {
        self.parent.as_ref()?.upgrade().map(|p| p.borrow().value)
    }
}

/// Doubly-linked list with Weak for back-links
struct ListNode<T> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
    prev: Option<Weak<RefCell<ListNode<T>>>>, // weak to break cycle
}

impl<T: std::fmt::Display> ListNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode { value, next: None, prev: None }))
    }
}

/// Show Rc/Weak reference counting
fn reference_counting_demo() {
    let a = Rc::new(5i32);
    println!("strong count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("strong count after cloning b: {}", Rc::strong_count(&a));

    let weak_a = Rc::downgrade(&a);
    println!("strong count after downgrade: {}", Rc::strong_count(&a));
    println!("weak count: {}", Rc::weak_count(&a));

    // Weak::upgrade returns Option — None after strong refs drop
    println!("upgrade: {:?}", weak_a.upgrade());
    drop(a);
    drop(b);
    println!("upgrade after drops: {:?}", weak_a.upgrade()); // None!
}

fn main() {
    println!("=== Tree with parent Weak refs ===");
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    let grandchild = TreeNode::new(4);

    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    TreeNode::add_child(&child1, grandchild.clone());

    println!("root children: {}", root.borrow().children.len());
    println!("child1 parent: {:?}", child1.borrow().parent_value());
    println!("grandchild parent: {:?}", grandchild.borrow().parent_value());

    // Drop root — children's Weak<parent> becomes invalid
    println!("strong count of root: {}", Rc::strong_count(&root));
    drop(root); // drops strong count — but child1 and child2 still hold it...
    // Actually: root strong_count = 1 (only root var), dropping it goes to 0

    println!("\n=== Reference counting demo ===");
    reference_counting_demo();

    // Cycle prevention: demonstrate Weak becomes None
    let outer;
    {
        let inner = Rc::new(42i32);
        outer = Rc::downgrade(&inner);
        println!("upgrade inside: {:?}", outer.upgrade());
    } // inner dropped here
    println!("upgrade after inner dropped: {:?}", outer.upgrade()); // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_strong_count() {
        let a = Rc::new(1i32);
        let b = a.clone();
        assert_eq!(Rc::strong_count(&a), 2);
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn test_weak_upgrades_to_none_after_drop() {
        let weak;
        {
            let strong = Rc::new(42i32);
            weak = Rc::downgrade(&strong);
            assert!(weak.upgrade().is_some());
        }
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_tree_parent_ref() {
        let root = TreeNode::new(10);
        let child = TreeNode::new(20);
        TreeNode::add_child(&root, child.clone());
        assert_eq!(child.borrow().parent_value(), Some(10));
    }
}
