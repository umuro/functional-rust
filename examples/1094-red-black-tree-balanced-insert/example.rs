// Red-Black Tree — Okasaki's Functional Balanced Insert
//
// A persistent (immutable) red-black tree following Okasaki's elegant
// functional approach. The `balance` function captures all four rotation
// cases in a single pattern match — a direct translation of the OCaml original.

/// Node color: Red or Black
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A persistent red-black tree.
///
/// Uses `Box` for heap-allocated children — Rust's closest analog to
/// OCaml's garbage-collected recursive variants.
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

    fn balance(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> Self {
        match (color, &left, &right) {
            (Black, Node(Red, ref ll, _, _), _)
                if matches!(ll.as_ref(), Node(Red, _, _, _)) =>
            {
                if let Node(Red, ll, y, c) = left {
                    if let Node(Red, a, x, b) = *ll {
                        return Node(
                            Red,
                            Box::new(Node(Black, a, x, b)),
                            y,
                            Box::new(Node(Black, c, value, Box::new(right))),
                        );
                    }
                }
                unreachable!()
            }
            (Black, Node(Red, _, _, ref lr), _)
                if matches!(lr.as_ref(), Node(Red, _, _, _)) =>
            {
                if let Node(Red, a, x, lr) = left {
                    if let Node(Red, b, y, c) = *lr {
                        return Node(
                            Red,
                            Box::new(Node(Black, a, x, b)),
                            y,
                            Box::new(Node(Black, c, value, Box::new(right))),
                        );
                    }
                }
                unreachable!()
            }
            (Black, _, Node(Red, ref rl, _, _))
                if matches!(rl.as_ref(), Node(Red, _, _, _)) =>
            {
                if let Node(Red, rl, z, d) = right {
                    if let Node(Red, b, y, c) = *rl {
                        return Node(
                            Red,
                            Box::new(Node(Black, Box::new(left), value, b)),
                            y,
                            Box::new(Node(Black, c, z, d)),
                        );
                    }
                }
                unreachable!()
            }
            (Black, _, Node(Red, _, _, ref rr))
                if matches!(rr.as_ref(), Node(Red, _, _, _)) =>
            {
                if let Node(Red, b, y, rr) = right {
                    if let Node(Red, c, z, d) = *rr {
                        return Node(
                            Red,
                            Box::new(Node(Black, Box::new(left), value, b)),
                            y,
                            Box::new(Node(Black, c, z, d)),
                        );
                    }
                }
                unreachable!()
            }
            _ => Node(color, Box::new(left), value, Box::new(right)),
        }
    }

    pub fn insert(&self, x: T) -> Self {
        fn ins<T: Ord + Clone>(tree: &RBTree<T>, x: T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), x, Box::new(Empty)),
                Node(color, a, y, b) => {
                    if x < *y {
                        RBTree::balance(*color, ins(a, x), y.clone(), b.as_ref().clone())
                    } else if x > *y {
                        RBTree::balance(*color, a.as_ref().clone(), y.clone(), ins(b, x))
                    } else {
                        tree.clone()
                    }
                }
            }
        }

        match ins(self, x) {
            Node(_, a, y, b) => Node(Black, a, y, b),
            Empty => Empty,
        }
    }

    pub fn contains(&self, x: &T) -> bool {
        match self {
            Empty => false,
            Node(_, a, y, b) => {
                if *x == *y {
                    true
                } else if *x < *y {
                    a.contains(x)
                } else {
                    b.contains(x)
                }
            }
        }
    }

    pub fn to_sorted_vec(&self) -> Vec<&T> {
        match self {
            Empty => vec![],
            Node(_, a, v, b) => {
                let mut result = a.to_sorted_vec();
                result.push(v);
                result.extend(b.to_sorted_vec());
                result
            }
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for RBTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RBTree::new(), |t, x| t.insert(x))
    }
}

fn main() {
    // Mirrors the OCaml example: fold_left insert E [5;3;7;1;4;6;8;2;9]
    let tree: RBTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();

    let sorted: Vec<&i32> = tree.to_sorted_vec();
    println!("In-order traversal: {:?}", sorted);

    println!("contains(5) = {}", tree.contains(&5));
    println!("contains(10) = {}", tree.contains(&10));

    // Persistence: inserting returns a new tree, original unchanged
    let tree2 = tree.insert(10);
    println!("After insert(10): {:?}", tree2.to_sorted_vec());
    println!("Original unchanged: {:?}", tree.to_sorted_vec());

    // Duplicates are ignored
    let tree3 = tree.insert(5);
    println!("After insert(5) (dup): {:?}", tree3.to_sorted_vec());
}

/* Output:
   In-order traversal: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   contains(5) = true
   contains(10) = false
   After insert(10): [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   Original unchanged: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   After insert(5) (dup): [1, 2, 3, 4, 5, 6, 7, 8, 9]
*/
