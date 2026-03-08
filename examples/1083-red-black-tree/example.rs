/// Red-Black Tree — Okasaki's purely functional balanced BST

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RBTree<T> {
    Empty,
    Node {
        color: Color,
        left: Box<RBTree<T>>,
        value: T,
        right: Box<RBTree<T>>,
    },
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

impl<T> RBTree<T> {
    fn node(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> Self {
        Node {
            color,
            left: Box::new(left),
            value,
            right: Box::new(right),
        }
    }

    fn is_red_node(&self) -> bool {
        matches!(self, Node { color: Red, .. })
    }
}

impl<T: Ord> RBTree<T> {
    fn new() -> Self {
        Empty
    }

    fn mem(&self, x: &T) -> bool {
        match self {
            Empty => false,
            Node {
                left, value, right, ..
            } => match x.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.mem(x),
                Ordering::Greater => right.mem(x),
            },
        }
    }

    fn insert(self, x: T) -> Self
    where
        T: Clone,
    {
        fn ins<T: Ord + Clone>(tree: RBTree<T>, x: &T) -> RBTree<T> {
            match tree {
                Empty => RBTree::node(Red, Empty, x.clone(), Empty),
                Node {
                    color,
                    left,
                    value,
                    right,
                } => match x.cmp(&value) {
                    Ordering::Less => balance(color, ins(*left, x), value, *right),
                    Ordering::Greater => balance(color, *left, value, ins(*right, x)),
                    Ordering::Equal => RBTree::node(color, *left, value, *right),
                },
            }
        }

        match ins(self, &x) {
            Node {
                left,
                value,
                right,
                ..
            } => Node {
                color: Black,
                left,
                value,
                right,
            },
            Empty => Empty,
        }
    }

    fn to_sorted_vec(&self) -> Vec<&T> {
        match self {
            Empty => vec![],
            Node {
                left, value, right, ..
            } => {
                let mut result = left.to_sorted_vec();
                result.push(value);
                result.extend(right.to_sorted_vec());
                result
            }
        }
    }
}

fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    if color != Black {
        return RBTree::node(color, left, value, right);
    }

    // Case 1: left-left
    if left.is_red_node() {
        if let Node { left: ref ll, .. } = left {
            if ll.is_red_node() {
                if let Node {
                    left: ll_box,
                    value: y,
                    right: c,
                    ..
                } = left
                {
                    if let Node {
                        left: a,
                        value: x,
                        right: b,
                        ..
                    } = *ll_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, *a, x, *b),
                            y,
                            RBTree::node(Black, *c, value, right),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Case 2: left-right
    if left.is_red_node() {
        if let Node { right: ref lr, .. } = left {
            if lr.is_red_node() {
                if let Node {
                    left: a,
                    value: x,
                    right: lr_box,
                    ..
                } = left
                {
                    if let Node {
                        left: b,
                        value: y,
                        right: c,
                        ..
                    } = *lr_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, *a, x, *b),
                            y,
                            RBTree::node(Black, *c, value, right),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Case 3: right-left
    if right.is_red_node() {
        if let Node { left: ref rl, .. } = right {
            if rl.is_red_node() {
                if let Node {
                    left: rl_box,
                    value: z,
                    right: d,
                    ..
                } = right
                {
                    if let Node {
                        left: b,
                        value: y,
                        right: c,
                        ..
                    } = *rl_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, left, value, *b),
                            y,
                            RBTree::node(Black, *c, z, *d),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    // Case 4: right-right
    if right.is_red_node() {
        if let Node { right: ref rr, .. } = right {
            if rr.is_red_node() {
                if let Node {
                    left: b,
                    value: y,
                    right: rr_box,
                    ..
                } = right
                {
                    if let Node {
                        left: c,
                        value: z,
                        right: d,
                        ..
                    } = *rr_box
                    {
                        return RBTree::node(
                            Red,
                            RBTree::node(Black, left, value, *b),
                            y,
                            RBTree::node(Black, *c, z, *d),
                        );
                    }
                }
                unreachable!();
            }
        }
    }

    RBTree::node(Black, left, value, right)
}

fn from_iter<T: Ord + Clone>(iter: impl IntoIterator<Item = T>) -> RBTree<T> {
    iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
}

fn main() {
    let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);

    print!("Sorted: ");
    for v in tree.to_sorted_vec() {
        print!("{v} ");
    }
    println!();

    println!("mem(5) = {}", tree.mem(&5));
    println!("mem(10) = {}", tree.mem(&10));

    // Duplicate insert — should not increase size
    let tree2 = from_iter([3, 1, 3, 2, 1, 3]);
    print!("Deduped: ");
    for v in tree2.to_sorted_vec() {
        print!("{v} ");
    }
    println!();
}

/* Output:
   Sorted: 1 2 3 4 5 6 7 8 9
   mem(5) = true
   mem(10) = false
   Deduped: 1 2 3
*/
