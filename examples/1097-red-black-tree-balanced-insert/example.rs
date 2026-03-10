use std::fmt;

/// Node color — mirrors OCaml's `type color = Red | Black`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// Red-black tree — mirrors OCaml's `type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

impl<T: Ord + Clone> Default for RBTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> RBTree<T> {
    pub fn new() -> Self {
        Empty
    }

    /// Insert a value, returning a new tree with the value added.
    /// Duplicate values are ignored (set semantics).
    pub fn insert(self, value: T) -> Self {
        fn ins<T: Ord + Clone>(tree: RBTree<T>, value: &T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), value.clone(), Box::new(Empty)),
                Node(color, left, v, right) => {
                    if *value < v {
                        balance(color, ins(*left, value), v, *right)
                    } else if *value > v {
                        balance(color, *left, v, ins(*right, value))
                    } else {
                        Node(color, left, v, right)
                    }
                }
            }
        }

        match ins(self, &value) {
            Node(_, left, v, right) => Node(Black, left, v, right),
            Empty => Empty,
        }
    }

    /// Check membership.
    pub fn mem(&self, value: &T) -> bool {
        match self {
            Empty => false,
            Node(_, left, v, right) => {
                if *value == *v {
                    true
                } else if *value < *v {
                    left.mem(value)
                } else {
                    right.mem(value)
                }
            }
        }
    }

    /// Collect all elements in sorted (in-order) order.
    pub fn to_sorted_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.collect_inorder(&mut result);
        result
    }

    fn collect_inorder(&self, acc: &mut Vec<T>) {
        if let Node(_, left, v, right) = self {
            left.collect_inorder(acc);
            acc.push(v.clone());
            right.collect_inorder(acc);
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(Self::new(), |tree, v| tree.insert(v))
    }
}

impl<T: fmt::Display> fmt::Display for RBTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Empty => write!(f, "."),
            Node(color, left, v, right) => {
                let c = match color {
                    Red => "R",
                    Black => "B",
                };
                write!(f, "({c} {left} {v} {right})")
            }
        }
    }
}

/// Balance restores red-black invariants after insertion.
fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        (Black, Node(Red, ll, lv, lr), v, r) if matches!(ll.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, a, x, b) = *ll else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, a, x, b)),
                lv,
                Box::new(Node(Black, lr, v, Box::new(r))),
            )
        }
        (Black, Node(Red, a, x, lr), v, r) if matches!(lr.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, b, y, c) = *lr else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, a, x, b)),
                y,
                Box::new(Node(Black, c, v, Box::new(r))),
            )
        }
        (Black, l, v, Node(Red, rl, rv, rr)) if matches!(rl.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, b, y, c) = *rl else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, Box::new(l), v, b)),
                y,
                Box::new(Node(Black, c, rv, rr)),
            )
        }
        (Black, l, v, Node(Red, rl, rv, rr)) if matches!(rr.as_ref(), Node(Red, _, _, _)) => {
            let Node(Red, c, z, d) = *rr else {
                unreachable!()
            };
            Node(
                Red,
                Box::new(Node(Black, Box::new(l), v, rl)),
                rv,
                Box::new(Node(Black, c, z, d)),
            )
        }
        (color, left, value, right) => Node(color, Box::new(left), value, Box::new(right)),
    }
}

fn main() {
    // Build tree from [5,3,7,1,4,6,8,2,9] — same as the OCaml example
    let tree: RBTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();

    println!("In-order: {:?}", tree.to_sorted_vec());
    println!("Tree: {tree}");
    println!("mem(5) = {}", tree.mem(&5));
    println!("mem(10) = {}", tree.mem(&10));

    // Ascending insertion — worst case for naive BST, balanced here
    let asc_tree: RBTree<i32> = (1..=7).collect();
    println!("\nAscending 1..=7: {:?}", asc_tree.to_sorted_vec());
    println!("Tree: {asc_tree}");
}

/* Output:
   In-order: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   Tree: (B (B (B . 1 (R . 2 .)) 3 (B (R . 4 .) 5 .)) 6 (B (B . 7 .) 8 (B . 9 .)))
   mem(5) = true
   mem(10) = false

   Ascending 1..=7: [1, 2, 3, 4, 5, 6, 7]
   Tree: (B (B (B . 1 .) 2 (B . 3 .)) 4 (B (B . 5 .) 6 (B . 7 .)))
*/
