// Red-Black Tree with Okasaki's Functional Balancing (Method Style)
//
// A purely functional red-black tree using Okasaki's four-case balance rule,
// expressed with methods on the tree type rather than free functions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RBTree<V> {
    E,
    T(Color, Box<RBTree<V>>, V, Box<RBTree<V>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

impl<V: Ord> RBTree<V> {
    fn new() -> Self {
        E
    }

    fn insert(self, value: V) -> Self {
        match Self::ins(value, self) {
            T(_, left, v, right) => T(Black, left, v, right),
            E => E,
        }
    }

    fn from_iter(iter: impl IntoIterator<Item = V>) -> Self {
        iter.into_iter().fold(E, |tree, x| tree.insert(x))
    }

    fn mem(&self, value: &V) -> bool {
        match self {
            E => false,
            T(_, left, v, right) => {
                if value == v {
                    true
                } else if value < v {
                    left.mem(value)
                } else {
                    right.mem(value)
                }
            }
        }
    }

    fn ins(x: V, tree: Self) -> Self {
        match tree {
            E => T(Red, Box::new(E), x, Box::new(E)),
            T(color, left, y, right) => {
                if x < y {
                    Self::balance(color, Self::ins(x, *left), y, *right)
                } else if x > y {
                    Self::balance(color, *left, y, Self::ins(x, *right))
                } else {
                    T(color, left, y, right)
                }
            }
        }
    }

    fn balance(color: Color, left: Self, val: V, right: Self) -> Self {
        let balanced =
            |a: Self, x: V, b: Self, y: V, c: Self, z: V, d: Self| -> Self {
                T(
                    Red,
                    Box::new(T(Black, Box::new(a), x, Box::new(b))),
                    y,
                    Box::new(T(Black, Box::new(c), z, Box::new(d))),
                )
            };

        match (color, left, val, right) {
            (Black, T(Red, ll, y, c), z, d) if matches!(*ll, T(Red, ..)) => {
                let T(_, a, x, b) = *ll else { unreachable!() };
                balanced(*a, x, *b, y, *c, z, d)
            }
            (Black, T(Red, a, x, lr), z, d) if matches!(*lr, T(Red, ..)) => {
                let T(_, b, y, c) = *lr else { unreachable!() };
                balanced(*a, x, *b, y, *c, z, d)
            }
            (Black, a, x, T(Red, rl, z, d)) if matches!(*rl, T(Red, ..)) => {
                let T(_, b, y, c) = *rl else { unreachable!() };
                balanced(a, x, *b, y, *c, z, *d)
            }
            (Black, a, x, T(Red, b, y, rr)) if matches!(*rr, T(Red, ..)) => {
                let T(_, c, z, d) = *rr else { unreachable!() };
                balanced(a, x, *b, y, *c, z, *d)
            }
            (col, a, x, b) => T(col, Box::new(a), x, Box::new(b)),
        }
    }
}

impl<V: Clone> RBTree<V> {
    fn to_sorted_vec(&self) -> Vec<V> {
        let mut result = Vec::new();
        self.collect_inorder(&mut result);
        result
    }

    fn collect_inorder(&self, acc: &mut Vec<V>) {
        if let T(_, left, v, right) = self {
            left.collect_inorder(acc);
            acc.push(v.clone());
            right.collect_inorder(acc);
        }
    }
}

impl<V: Ord> FromIterator<V> for RBTree<V> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        iter.into_iter().fold(E, |tree, x| tree.insert(x))
    }
}

fn main() {
    let tree: RBTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();
    let sorted = tree.to_sorted_vec();
    println!("Sorted: {:?}", sorted);
    println!("mem(5) = {}", tree.mem(&5));
    println!("mem(0) = {}", tree.mem(&0));

    let tree2: RBTree<&str> = ["cherry", "apple", "banana"].into_iter().collect();
    println!("Strings: {:?}", tree2.to_sorted_vec());
}

/* Output:
   Sorted: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   mem(5) = true
   mem(0) = false
   Strings: ["apple", "banana", "cherry"]
*/
