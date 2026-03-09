// Red-Black Tree — Balanced Insert (Okasaki)
//
// A persistent (immutable) red-black tree following Okasaki's functional
// balancing. Four rotation cases collapse into one rebalanced shape.

/// Node color in a red-black tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree parameterized over `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{Empty, Node};

impl<T: Ord + Clone> RBTree<T> {
    pub fn new() -> Self {
        Empty
    }

    /// Okasaki's balance — four red-red violation cases → one uniform shape.
    fn balance(color: Color, left: Self, value: T, right: Self) -> Self {
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

    /// Inserts a value, returning a new tree. Root is always painted black.
    pub fn insert(&self, x: T) -> Self {
        fn ins<T: Ord + Clone>(tree: &RBTree<T>, x: T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), x, Box::new(Empty)),
                Node(color, a, y, b) => match x.cmp(y) {
                    std::cmp::Ordering::Less => {
                        RBTree::balance(*color, ins(a, x), y.clone(), b.as_ref().clone())
                    }
                    std::cmp::Ordering::Greater => {
                        RBTree::balance(*color, a.as_ref().clone(), y.clone(), ins(b, x))
                    }
                    std::cmp::Ordering::Equal => tree.clone(),
                },
            }
        }
        match ins(self, x) {
            Node(_, a, y, b) => Node(Black, a, y, b),
            Empty => Empty,
        }
    }

    /// Checks whether a value exists in the tree.
    pub fn contains(&self, x: &T) -> bool {
        match self {
            Empty => false,
            Node(_, a, y, b) => match x.cmp(y) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => a.contains(x),
                std::cmp::Ordering::Greater => b.contains(x),
            },
        }
    }

    /// Collects elements in sorted order via in-order traversal.
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        fn collect<'a, T>(tree: &'a RBTree<T>, acc: &mut Vec<&'a T>) {
            if let Node(_, a, v, b) = tree {
                collect(a, acc);
                acc.push(v);
                collect(b, acc);
            }
        }
        let mut result = Vec::new();
        collect(self, &mut result);
        result
    }
}

impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
    }
}

fn main() {
    // Mirrors the OCaml example: fold insert over [5;3;7;1;4;6;8;2;9]
    let tree: RBTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();

    let sorted = tree.to_sorted_vec();
    println!("in-order: {:?}", sorted);
    println!("contains(5) = {}", tree.contains(&5));
    println!("contains(0) = {}", tree.contains(&0));

    // Persistence: inserting returns a new tree
    let tree2 = tree.insert(10);
    println!("after insert(10): {:?}", tree2.to_sorted_vec());
    println!("original unchanged: {:?}", tree.to_sorted_vec());
}

/* Output:
   in-order: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   contains(5) = true
   contains(0) = false
   after insert(10): [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   original unchanged: [1, 2, 3, 4, 5, 6, 7, 8, 9]
*/
