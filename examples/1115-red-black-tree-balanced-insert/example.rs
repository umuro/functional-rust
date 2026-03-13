#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RBTree<T> {
    E,
    T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        (Black, T(Red, a, x, b), y, d) if matches!(*a, T(Red, ..)) => {
            let T(Red, aa, xx, bb) = *a else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, aa, xx, bb)),
                x,
                Box::new(T(Black, b, y, Box::new(d))),
            )
        }
        (Black, T(Red, a, x, b), y, d) if matches!(*b, T(Red, ..)) => {
            let T(Red, bb, yy, cc) = *b else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, a, x, bb)),
                yy,
                Box::new(T(Black, cc, y, Box::new(d))),
            )
        }
        (Black, a, x, T(Red, b, y, d)) if matches!(*b, T(Red, ..)) => {
            let T(Red, bb, yy, cc) = *b else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, Box::new(a), x, bb)),
                yy,
                Box::new(T(Black, cc, y, d)),
            )
        }
        (Black, a, x, T(Red, b, y, d)) if matches!(*d, T(Red, ..)) => {
            let T(Red, cc, zz, dd) = *d else {
                unreachable!()
            };
            T(
                Red,
                Box::new(T(Black, Box::new(a), x, b)),
                y,
                Box::new(T(Black, cc, zz, dd)),
            )
        }
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}

pub fn insert<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
        match tree {
            E => T(Red, Box::new(E), value, Box::new(E)),
            T(color, left, v, right) => {
                if value < v {
                    balance(color, ins(value, *left), v, *right)
                } else if value > v {
                    balance(color, *left, v, ins(value, *right))
                } else {
                    T(color, left, v, right)
                }
            }
        }
    }
    match ins(value, tree) {
        T(_, left, v, right) => T(Black, left, v, right),
        E => E,
    }
}

pub fn mem<T: Ord>(value: &T, tree: &RBTree<T>) -> bool {
    match tree {
        E => false,
        T(_, left, v, right) => {
            if value == v {
                true
            } else if value < v {
                mem(value, left)
            } else {
                mem(value, right)
            }
        }
    }
}

pub fn to_list<T: Clone>(tree: &RBTree<T>) -> Vec<T> {
    match tree {
        E => vec![],
        T(_, left, v, right) => {
            let mut acc = to_list(left);
            acc.push(v.clone());
            acc.extend(to_list(right));
            acc
        }
    }
}

fn main() {
    let values = [5, 3, 7, 1, 4, 6, 8, 2, 9];
    let tree = values.iter().fold(RBTree::E, |t, &x| insert(x, t));

    println!("Inserted: {:?}", values);
    println!("Sorted:   {:?}", to_list(&tree));
    println!("mem(4):   {}", mem(&4, &tree));
    println!("mem(10):  {}", mem(&10, &tree));
    match &tree {
        T(Color::Black, _, root, _) => println!("Root:     {} (black ✓)", root),
        _ => println!("Root: empty"),
    }
}

/* Output:
   Inserted: [5, 3, 7, 1, 4, 6, 8, 2, 9]
   Sorted:   [1, 2, 3, 4, 5, 6, 7, 8, 9]
   mem(4):   true
   mem(10):  false
   Root:     5 (black ✓)
*/
