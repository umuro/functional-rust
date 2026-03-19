// Example 068: Foldable for Binary Tree
// Left/right fold, collect all values, various aggregations

#[derive(Debug)]
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    fn node(l: Tree<T>, v: T, r: Tree<T>) -> Self {
        Tree::Node(Box::new(l), v, Box::new(r))
    }

    // Approach 1: In-order fold
    fn fold_inorder<B>(&self, init: B, f: &mut impl FnMut(B, &T) -> B) -> B {
        match self {
            Tree::Leaf => init,
            Tree::Node(l, v, r) => {
                let acc = l.fold_inorder(init, f);
                let acc = f(acc, v);
                r.fold_inorder(acc, f)
            }
        }
    }

    // Approach 2: Pre-order and post-order
    fn fold_preorder<B>(&self, init: B, f: &mut impl FnMut(B, &T) -> B) -> B {
        match self {
            Tree::Leaf => init,
            Tree::Node(l, v, r) => {
                let acc = f(init, v);
                let acc = l.fold_preorder(acc, f);
                r.fold_preorder(acc, f)
            }
        }
    }

    fn fold_postorder<B>(&self, init: B, f: &mut impl FnMut(B, &T) -> B) -> B {
        match self {
            Tree::Leaf => init,
            Tree::Node(l, v, r) => {
                let acc = l.fold_postorder(init, f);
                let acc = r.fold_postorder(acc, f);
                f(acc, v)
            }
        }
    }
}

// Approach 3: Derived operations
impl Tree<i32> {
    fn to_vec_inorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.fold_inorder((), &mut |(), x| {
            result.push(*x);
        });
        result
    }

    fn sum(&self) -> i32 {
        self.fold_inorder(0, &mut |acc, x| acc + x)
    }

    fn max_val(&self) -> Option<i32> {
        self.fold_inorder(None, &mut |acc: Option<i32>, x| {
            Some(acc.map_or(*x, |a| a.max(*x)))
        })
    }

    fn all(&self, pred: impl Fn(&i32) -> bool) -> bool {
        self.fold_inorder(true, &mut |acc, x| acc && pred(x))
    }

    fn any(&self, pred: impl Fn(&i32) -> bool) -> bool {
        self.fold_inorder(false, &mut |acc, x| acc || pred(x))
    }

    fn count(&self, pred: impl Fn(&i32) -> bool) -> usize {
        self.fold_inorder(0, &mut |acc, x| if pred(x) { acc + 1 } else { acc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Tree<i32> {
        Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(
                Tree::node(Tree::Leaf, 3, Tree::Leaf),
                4,
                Tree::node(Tree::Leaf, 5, Tree::Leaf),
            ),
        )
    }

    #[test]
    fn test_inorder() {
        assert_eq!(sample_tree().to_vec_inorder(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sample_tree().sum(), 15);
    }

    #[test]
    fn test_max() {
        assert_eq!(sample_tree().max_val(), Some(5));
    }

    #[test]
    fn test_all() {
        assert!(sample_tree().all(|x| *x > 0));
        assert!(!sample_tree().all(|x| *x > 2));
    }

    #[test]
    fn test_any() {
        assert!(sample_tree().any(|x| *x == 3));
        assert!(!sample_tree().any(|x| *x == 99));
    }

    #[test]
    fn test_count() {
        assert_eq!(sample_tree().count(|x| x % 2 == 0), 2);
    }

    #[test]
    fn test_preorder() {
        let mut result = Vec::new();
        sample_tree().fold_preorder((), &mut |(), x| {
            result.push(*x);
        });
        assert_eq!(result, vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_postorder() {
        let mut result = Vec::new();
        sample_tree().fold_postorder((), &mut |(), x| {
            result.push(*x);
        });
        assert_eq!(result, vec![1, 3, 5, 4, 2]);
    }

    #[test]
    fn test_empty_tree() {
        let t = Tree::<i32>::Leaf;
        assert_eq!(t.sum(), 0);
        assert_eq!(t.max_val(), None);
    }
}
