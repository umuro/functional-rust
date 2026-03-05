// 973: Finger Tree (Simplified)
// Deque with O(1) amortized push/pop both ends
// 2-3 finger tree: digits (1-4 elements) at each end, nodes spine

#[derive(Debug, Clone)]
pub enum Digit<T> {
    One(T),
    Two(T, T),
    Three(T, T, T),
    Four(T, T, T, T),
}

#[derive(Debug, Clone)]
pub enum Node<T> {
    Node2(T, T),
    Node3(T, T, T),
}

#[derive(Debug, Clone)]
pub enum FingerTree<T> {
    Empty,
    Single(T),
    Deep(Box<Digit<T>>, Box<FingerTree<Node<T>>>, Box<Digit<T>>),
}

impl<T: Clone> Digit<T> {
    fn to_vec(&self) -> Vec<T> {
        match self {
            Digit::One(a) => vec![a.clone()],
            Digit::Two(a, b) => vec![a.clone(), b.clone()],
            Digit::Three(a, b, c) => vec![a.clone(), b.clone(), c.clone()],
            Digit::Four(a, b, c, d) => vec![a.clone(), b.clone(), c.clone(), d.clone()],
        }
    }
}

impl<T: Clone> Node<T> {
    fn to_vec(&self) -> Vec<T> {
        match self {
            Node::Node2(a, b) => vec![a.clone(), b.clone()],
            Node::Node3(a, b, c) => vec![a.clone(), b.clone(), c.clone()],
        }
    }
}

impl<T: Clone> FingerTree<T> {
    pub fn empty() -> Self {
        FingerTree::Empty
    }

    pub fn push_front(self, x: T) -> Self {
        match self {
            FingerTree::Empty => FingerTree::Single(x),
            FingerTree::Single(y) => FingerTree::Deep(
                Box::new(Digit::One(x)),
                Box::new(FingerTree::Empty),
                Box::new(Digit::One(y)),
            ),
            FingerTree::Deep(l, spine, r) => match *l {
                Digit::One(a) => FingerTree::Deep(
                    Box::new(Digit::Two(x, a)), spine, r,
                ),
                Digit::Two(a, b) => FingerTree::Deep(
                    Box::new(Digit::Three(x, a, b)), spine, r,
                ),
                Digit::Three(a, b, c) => FingerTree::Deep(
                    Box::new(Digit::Four(x, a, b, c)), spine, r,
                ),
                Digit::Four(a, b, c, d) => {
                    let new_spine = spine.push_front(Node::Node3(b, c, d));
                    FingerTree::Deep(
                        Box::new(Digit::Two(x, a)),
                        Box::new(new_spine),
                        r,
                    )
                }
            },
        }
    }

    pub fn push_back(self, x: T) -> Self {
        match self {
            FingerTree::Empty => FingerTree::Single(x),
            FingerTree::Single(y) => FingerTree::Deep(
                Box::new(Digit::One(y)),
                Box::new(FingerTree::Empty),
                Box::new(Digit::One(x)),
            ),
            FingerTree::Deep(l, spine, r) => match *r {
                Digit::One(a) => FingerTree::Deep(
                    l, spine, Box::new(Digit::Two(a, x)),
                ),
                Digit::Two(a, b) => FingerTree::Deep(
                    l, spine, Box::new(Digit::Three(a, b, x)),
                ),
                Digit::Three(a, b, c) => FingerTree::Deep(
                    l, spine, Box::new(Digit::Four(a, b, c, x)),
                ),
                Digit::Four(a, b, c, d) => {
                    let new_spine = spine.push_back(Node::Node3(b, c, d));
                    FingerTree::Deep(
                        l,
                        Box::new(new_spine),
                        Box::new(Digit::Two(a, x)),
                    )
                }
            },
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            FingerTree::Empty => vec![],
            FingerTree::Single(x) => vec![x.clone()],
            FingerTree::Deep(l, spine, r) => {
                let mut result = l.to_vec();
                for node in spine.to_vec() {
                    result.extend(node.to_vec());
                }
                result.extend(r.to_vec());
                result
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back_order() {
        let t = (1..=5).fold(FingerTree::empty(), |acc, x| acc.push_back(x));
        assert_eq!(t.to_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_push_front_order() {
        let t = (1..=5).fold(FingerTree::empty(), |acc, x| acc.push_front(x));
        assert_eq!(t.to_vec(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_mixed_push() {
        let t = FingerTree::empty()
            .push_back(1)
            .push_back(2)
            .push_back(3)
            .push_front(0)
            .push_back(4)
            .push_front(-1);
        assert_eq!(t.to_vec(), vec![-1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_longer_sequence() {
        let t = (1..=10).fold(FingerTree::empty(), |acc, x| acc.push_back(x));
        assert_eq!(t.to_vec(), (1..=10).collect::<Vec<_>>());
    }

    #[test]
    fn test_empty() {
        let t: FingerTree<i32> = FingerTree::empty();
        assert_eq!(t.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_single() {
        let t = FingerTree::empty().push_back(42);
        assert_eq!(t.to_vec(), vec![42]);
    }
}
