#![allow(clippy::all)]
// 972: Persistent Binary Search Tree
// Functional update: insert/delete return new Rc-shared trees
// Shared nodes between versions via Rc<BstNode<T>>

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Bst<T> {
    Empty,
    Node(Rc<Bst<T>>, T, Rc<Bst<T>>),
}

impl<T: Ord + Clone> Bst<T> {
    pub fn empty() -> Self {
        Bst::Empty
    }

    /// Insert returns a new tree sharing unchanged subtrees
    pub fn insert(&self, x: T) -> Self {
        match self {
            Bst::Empty => Bst::Node(Rc::new(Bst::Empty), x, Rc::new(Bst::Empty)),
            Bst::Node(l, v, r) => {
                if x < *v {
                    Bst::Node(Rc::new(l.insert(x)), v.clone(), Rc::clone(r))
                } else if x > *v {
                    Bst::Node(Rc::clone(l), v.clone(), Rc::new(r.insert(x)))
                } else {
                    self.clone() // duplicate: return same (Rc-shared)
                }
            }
        }
    }

    pub fn member(&self, x: &T) -> bool {
        match self {
            Bst::Empty => false,
            Bst::Node(l, v, r) => {
                if x == v {
                    true
                } else if x < v {
                    l.member(x)
                } else {
                    r.member(x)
                }
            }
        }
    }

    pub fn min_val(&self) -> Option<&T> {
        match self {
            Bst::Empty => None,
            Bst::Node(l, v, _) => {
                if matches!(l.as_ref(), Bst::Empty) {
                    Some(v)
                } else {
                    l.min_val()
                }
            }
        }
    }

    /// Delete returns a new tree, old tree unchanged
    pub fn delete(&self, x: &T) -> Self {
        match self {
            Bst::Empty => Bst::Empty,
            Bst::Node(l, v, r) => {
                if x < v {
                    Bst::Node(Rc::new(l.delete(x)), v.clone(), Rc::clone(r))
                } else if x > v {
                    Bst::Node(Rc::clone(l), v.clone(), Rc::new(r.delete(x)))
                } else {
                    // Found node: replace with min of right subtree
                    match r.min_val() {
                        None => (**l).clone(), // no right subtree
                        Some(m) => {
                            let m = m.clone();
                            let new_r = r.delete(&m);
                            Bst::Node(Rc::clone(l), m, Rc::new(new_r))
                        }
                    }
                }
            }
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            Bst::Empty => vec![],
            Bst::Node(l, v, r) => {
                let mut result = l.to_vec();
                result.push(v.clone());
                result.extend(r.to_vec());
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tree() -> Bst<i32> {
        Bst::empty()
            .insert(5)
            .insert(3)
            .insert(7)
            .insert(1)
            .insert(4)
    }

    #[test]
    fn test_insert_sorted() {
        let t = make_tree();
        assert_eq!(t.to_vec(), vec![1, 3, 4, 5, 7]);
    }

    #[test]
    fn test_persistence() {
        let t0 = Bst::empty().insert(5).insert(3).insert(7).insert(1);
        let t1 = t0.insert(4);
        // t0 is unchanged
        assert_eq!(t0.to_vec(), vec![1, 3, 5, 7]);
        assert_eq!(t1.to_vec(), vec![1, 3, 4, 5, 7]);
    }

    #[test]
    fn test_member() {
        let t = make_tree();
        assert!(t.member(&4));
        assert!(t.member(&5));
        assert!(!t.member(&2));
        assert!(!t.member(&6));
    }

    #[test]
    fn test_delete_leaf() {
        let t = make_tree();
        let t2 = t.delete(&1);
        assert_eq!(t2.to_vec(), vec![3, 4, 5, 7]);
        assert_eq!(t.to_vec(), vec![1, 3, 4, 5, 7]); // unchanged
    }

    #[test]
    fn test_delete_internal() {
        let t = make_tree();
        let t2 = t.delete(&3);
        assert_eq!(t2.to_vec(), vec![1, 4, 5, 7]);

        let t3 = t.delete(&5); // delete root
        assert_eq!(t3.to_vec(), vec![1, 3, 4, 7]);
    }
}
