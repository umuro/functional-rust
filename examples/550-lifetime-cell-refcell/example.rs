//! # 550. Interior Mutability: Cell and RefCell
//! Shared mutation patterns with runtime borrow checking.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Cell<T>: for Copy types, no borrow needed
fn cell_demo() {
    let x = Cell::new(5i32);
    let y = &x; // shared reference
    let z = &x; // another shared reference

    // Both can mutate through shared refs!
    y.set(10);
    z.set(20);
    println!("x = {} (mutated through two shared refs)", x.get());

    // Real use case: stats counter in shared struct
    struct Stats {
        calls: Cell<usize>,
        sum: Cell<i64>,
    }

    impl Stats {
        fn new() -> Self { Stats { calls: Cell::new(0), sum: Cell::new(0) } }
        fn record(&self, value: i64) { // &self, not &mut self!
            self.calls.set(self.calls.get() + 1);
            self.sum.set(self.sum.get() + value);
        }
        fn mean(&self) -> f64 { self.sum.get() as f64 / self.calls.get() as f64 }
    }

    let stats = Stats::new();
    stats.record(10);
    stats.record(20);
    stats.record(30);
    println!("calls: {}, mean: {:.1}", stats.calls.get(), stats.mean());
}

/// RefCell<T>: for non-Copy types, runtime borrow check
fn refcell_demo() {
    let data = RefCell::new(vec![1, 2, 3]);

    // Borrow immutably
    {
        let r = data.borrow();
        println!("data: {:?}", *r);
        // Another immutable borrow — OK
        let r2 = data.borrow();
        println!("data again: {:?}", *r2);
        // drop r2 first, then r
    } // both borrows end here

    // Borrow mutably
    {
        let mut r = data.borrow_mut();
        r.push(4);
        r.push(5);
    }
    println!("after push: {:?}", *data.borrow());

    // Runtime panic example:
    // let _r = data.borrow();
    // let _w = data.borrow_mut(); // PANICS: already borrowed!
}

/// Rc<RefCell<T>>: shared ownership + interior mutability
fn rc_refcell_demo() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<RefCell<Node>>>,
    }

    impl Node {
        fn new(v: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node { value: v, children: Vec::new() }))
        }

        fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
            parent.borrow_mut().children.push(child);
        }
    }

    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1.clone());
    Node::add_child(&root, child2.clone());

    println!("root children: {}", root.borrow().children.len());
    child1.borrow_mut().value = 20; // mutate through shared ref
    println!("child1 value: {}", child1.borrow().value);
}

fn main() {
    println!("=== Cell<T> ===");
    cell_demo();

    println!("\n=== RefCell<T> ===");
    refcell_demo();

    println!("\n=== Rc<RefCell<T>> ===");
    rc_refcell_demo();

    // try_borrow — non-panicking version
    {
        let r = RefCell::new(42i32);
        let b1 = r.borrow();
        match r.try_borrow_mut() {
            Ok(_) => println!("got mut borrow"),
            Err(e) => println!("borrow failed (expected): {}", e),
        }
        drop(b1);
        if let Ok(mut m) = r.try_borrow_mut() {
            *m = 100;
            println!("mutated to {}", *m);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_basic() {
        let c = Cell::new(0i32);
        c.set(42);
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn test_cell_through_shared_ref() {
        let c = Cell::new(0i32);
        let r1 = &c;
        let r2 = &c;
        r1.set(10);
        r2.set(20);
        assert_eq!(c.get(), 20);
    }

    #[test]
    fn test_refcell_borrow() {
        let rc = RefCell::new(vec![1, 2]);
        rc.borrow_mut().push(3);
        assert_eq!(*rc.borrow(), vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_refcell_double_mut_panics() {
        let rc = RefCell::new(0i32);
        let _m1 = rc.borrow_mut();
        let _m2 = rc.borrow_mut(); // panics!
    }
}
