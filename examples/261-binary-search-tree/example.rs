/// Immutable Binary Search Tree — Insert, Membership, In-Order Traversal

#[derive(Debug, Clone, PartialEq)]
pub enum Bst<T> {
    Leaf,
    Node(Box<Bst<T>>, T, Box<Bst<T>>),
}

impl<T: Ord + Clone> Bst<T> {
    pub fn new() -> Self {
        Bst::Leaf
    }

    /// Functional insert — returns a new tree, original unchanged.
    pub fn insert(&self, x: T) -> Self {
        match self {
            Bst::Leaf => Bst::Node(Box::new(Bst::Leaf), x, Box::new(Bst::Leaf)),
            Bst::Node(left, val, right) => match x.cmp(val) {
                std::cmp::Ordering::Less => {
                    Bst::Node(Box::new(left.insert(x)), val.clone(), right.clone())
                }
                std::cmp::Ordering::Greater => {
                    Bst::Node(left.clone(), val.clone(), Box::new(right.insert(x)))
                }
                std::cmp::Ordering::Equal => self.clone(),
            },
        }
    }

    /// Membership check — borrows the tree, no allocation.
    pub fn mem(&self, x: &T) -> bool {
        match self {
            Bst::Leaf => false,
            Bst::Node(left, val, right) => match x.cmp(val) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => left.mem(x),
                std::cmp::Ordering::Greater => right.mem(x),
            },
        }
    }

    /// In-order traversal — returns sorted elements.
    pub fn inorder(&self) -> Vec<T> {
        match self {
            Bst::Leaf => vec![],
            Bst::Node(left, val, right) => {
                let mut result = left.inorder();
                result.push(val.clone());
                result.extend(right.inorder());
                result
            }
        }
    }

    pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
        items
            .into_iter()
            .fold(Bst::new(), |tree, x| tree.insert(x))
    }
}

fn main() {
    let tree = Bst::build([5, 3, 7, 1, 4, 6, 8]);
    println!("inorder: {:?}", tree.inorder());
    println!("mem 4 = {}", tree.mem(&4));
    println!("mem 9 = {}", tree.mem(&9));

    // Persistence: original unchanged after insert
    let tree2 = tree.insert(10);
    println!("after insert 10: {:?}", tree2.inorder());
    println!("original still: {:?}", tree.inorder());
}

/* Output:
   inorder: [1, 3, 4, 5, 6, 7, 8]
   mem 4 = true
   mem 9 = false
   after insert 10: [1, 3, 4, 5, 6, 7, 8, 10]
   original still: [1, 3, 4, 5, 6, 7, 8]
*/
