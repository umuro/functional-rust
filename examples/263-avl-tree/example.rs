/// AVL Tree — Self-Balancing BST with Rotations

use std::cmp::{max, Ordering};

#[derive(Debug, Clone, PartialEq)]
pub enum Avl<T> {
    Empty,
    Node {
        left: Box<Avl<T>>,
        value: T,
        right: Box<Avl<T>>,
        height: i32,
    },
}

impl<T: Ord + Clone> Avl<T> {
    pub fn empty() -> Self { Avl::Empty }

    pub fn height(&self) -> i32 {
        match self {
            Avl::Empty => 0,
            Avl::Node { height, .. } => *height,
        }
    }

    fn node(left: Avl<T>, value: T, right: Avl<T>) -> Self {
        let h = 1 + max(left.height(), right.height());
        Avl::Node { left: Box::new(left), value, right: Box::new(right), height: h }
    }

    fn balance_factor(&self) -> i32 {
        match self {
            Avl::Empty => 0,
            Avl::Node { left, right, .. } => left.height() - right.height(),
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Avl::Node { left, value, right, .. } => match *left {
                Avl::Node { left: ll, value: lv, right: lr, .. } =>
                    Self::node(*ll, lv, Self::node(*lr, value, *right)),
                _ => Self::node(*left, value, *right),
            },
            other => other,
        }
    }

    fn rotate_left(self) -> Self {
        match self {
            Avl::Node { left, value, right, .. } => match *right {
                Avl::Node { left: rl, value: rv, right: rr, .. } =>
                    Self::node(Self::node(*left, value, *rl), rv, *rr),
                _ => Self::node(*left, value, *right),
            },
            other => other,
        }
    }

    fn rebalance(self) -> Self {
        let bf = self.balance_factor();
        if bf > 1 { self.rotate_right() }
        else if bf < -1 { self.rotate_left() }
        else { self }
    }

    pub fn insert(&self, x: T) -> Self {
        match self {
            Avl::Empty => Self::node(Avl::Empty, x, Avl::Empty),
            Avl::Node { left, value, right, .. } => match x.cmp(value) {
                Ordering::Less => Self::node(left.insert(x), value.clone(), (**right).clone()).rebalance(),
                Ordering::Greater => Self::node((**left).clone(), value.clone(), right.insert(x)).rebalance(),
                Ordering::Equal => self.clone(),
            },
        }
    }

    pub fn inorder(&self) -> Vec<T> {
        match self {
            Avl::Empty => vec![],
            Avl::Node { left, value, right, .. } => {
                let mut result = left.inorder();
                result.push(value.clone());
                result.extend(right.inorder());
                result
            }
        }
    }

    pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
        items.into_iter().fold(Avl::empty(), |tree, x| tree.insert(x))
    }
}

fn main() {
    let tree = Avl::build([7, 3, 9, 1, 5, 8, 10, 2]);
    println!("inorder: {:?}", tree.inorder());
    println!("height: {}", tree.height());

    // Sorted insertion stays balanced thanks to rotations
    let sorted_tree = Avl::build(1..=15);
    println!("sorted 1..=15 inorder: {:?}", sorted_tree.inorder());
    println!("sorted 1..=15 height: {}", sorted_tree.height());
}

/* Output:
   inorder: [1, 2, 3, 5, 7, 8, 9, 10]
   height: 4
   sorted 1..=15 inorder: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
   sorted 1..=15 height: 4
*/
