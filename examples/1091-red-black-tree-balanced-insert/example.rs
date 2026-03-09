// Red-Black Tree — Balanced Insert (Iterator + free-function style)
//
// Demonstrates Okasaki's functional red-black tree with:
// - Free functions mirroring OCaml's module-level style
// - Stack-based lazy iterator (Solution 3)
// - Method wrappers for idiomatic Rust API (Solution 2)

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

// ── Solution 1: Free functions — mirrors OCaml's module-level style ──

pub fn balance<T: Ord + Clone>(
    color: Color,
    left: RBTree<T>,
    value: T,
    right: RBTree<T>,
) -> RBTree<T> {
    match (color, left, right) {
        (Black, Node(Red, ll, y, c), d) if matches!(*ll, Node(Red, _, _, _)) => {
            let Node(Red, a, x, b) = *ll else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, a, x, b)),
                y,
                Box::new(Node(Black, c, value, Box::new(d))),
            )
        }
        (Black, Node(Red, a, x, lr), d) if matches!(*lr, Node(Red, _, _, _)) => {
            let Node(Red, b, y, c) = *lr else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, a, x, b)),
                y,
                Box::new(Node(Black, c, value, Box::new(d))),
            )
        }
        (Black, a, Node(Red, rl, z, d)) if matches!(*rl, Node(Red, _, _, _)) => {
            let Node(Red, b, y, c) = *rl else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, Box::new(a), value, b)),
                y,
                Box::new(Node(Black, c, z, d)),
            )
        }
        (Black, a, Node(Red, b, y, rr)) if matches!(*rr, Node(Red, _, _, _)) => {
            let Node(Red, c, z, d) = *rr else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, Box::new(a), value, b)),
                y,
                Box::new(Node(Black, c, z, d)),
            )
        }
        (c, left, right) => Node(c, Box::new(left), value, Box::new(right)),
    }
}

pub fn insert<T: Ord + Clone>(x: T, tree: &RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord + Clone>(x: &T, tree: &RBTree<T>) -> RBTree<T> {
        match tree {
            Empty => Node(Red, Box::new(Empty), x.clone(), Box::new(Empty)),
            Node(color, a, y, b) => match x.cmp(y) {
                Ordering::Less => balance(*color, ins(x, a), y.clone(), b.as_ref().clone()),
                Ordering::Greater => balance(*color, a.as_ref().clone(), y.clone(), ins(x, b)),
                Ordering::Equal => tree.clone(),
            },
        }
    }
    match ins(&x, tree) {
        Node(_, a, y, b) => Node(Black, a, y, b),
        Empty => Empty,
    }
}

pub fn mem<T: Ord>(x: &T, tree: &RBTree<T>) -> bool {
    match tree {
        Empty => false,
        Node(_, a, y, b) => match x.cmp(y) {
            Ordering::Equal => true,
            Ordering::Less => mem(x, a),
            Ordering::Greater => mem(x, b),
        },
    }
}

// ── Solution 2: Method wrappers ──

impl<T: Ord + Clone> RBTree<T> {
    pub fn new() -> Self {
        Empty
    }
    pub fn insert(&self, x: T) -> Self {
        insert(x, self)
    }
    pub fn contains(&self, x: &T) -> bool {
        mem(x, self)
    }
}

impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
    }
}

// ── Solution 3: Stack-based in-order iterator ──

pub struct InOrder<'a, T> {
    stack: Vec<&'a RBTree<T>>,
}

impl<'a, T> InOrder<'a, T> {
    fn new(tree: &'a RBTree<T>) -> Self {
        let mut iter = InOrder { stack: Vec::new() };
        iter.push_left_spine(tree);
        iter
    }
    fn push_left_spine(&mut self, mut tree: &'a RBTree<T>) {
        while let Node(_, left, _, _) = tree {
            self.stack.push(tree);
            tree = left;
        }
    }
}

impl<'a, T> Iterator for InOrder<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Node(_, _, value, right) = node {
            self.push_left_spine(right);
            Some(value)
        } else {
            None
        }
    }
}

impl<T: Ord + Clone> RBTree<T> {
    pub fn iter(&self) -> InOrder<'_, T> {
        InOrder::new(self)
    }
}

fn main() {
    // Build tree from [5,3,7,1,4,6,8,2,9] — mirrors the OCaml example
    let tree: RBTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();

    // In-order traversal via iterator
    let sorted: Vec<&i32> = tree.iter().collect();
    println!("in-order: {sorted:?}");

    // Membership checks
    println!("contains 5? {}", tree.contains(&5));
    println!("contains 10? {}", tree.contains(&10));

    // Persistence: insert into existing tree
    let tree2 = tree.insert(10);
    println!("after insert 10: {:?}", tree2.iter().collect::<Vec<_>>());
    println!("original still has 9 elements: {}", tree.iter().count());

    // Free function style (OCaml-like)
    let t = insert(42, &RBTree::new());
    println!("mem 42? {}", mem(&42, &t));
    println!("mem 99? {}", mem(&99, &t));
}

/* Output:
   in-order: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   contains 5? true
   contains 10? false
   after insert 10: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   original still has 9 elements: 9
   mem 42? true
   mem 99? false
*/
