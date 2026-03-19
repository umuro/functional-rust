// Example 069: Traversable for Binary Tree
// Map over a tree with effects (Option/Result)

#[derive(Debug, PartialEq, Clone)]
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    fn node(l: Tree<T>, v: T, r: Tree<T>) -> Self {
        Tree::Node(Box::new(l), v, Box::new(r))
    }

    // Approach 1: Traverse with Option
    fn traverse_option<U>(&self, f: &impl Fn(&T) -> Option<U>) -> Option<Tree<U>> {
        match self {
            Tree::Leaf => Some(Tree::Leaf),
            Tree::Node(l, v, r) => {
                let l2 = l.traverse_option(f)?;
                let v2 = f(v)?;
                let r2 = r.traverse_option(f)?;
                Some(Tree::node(l2, v2, r2))
            }
        }
    }

    // Approach 2: Traverse with Result
    fn traverse_result<U, E>(&self, f: &impl Fn(&T) -> Result<U, E>) -> Result<Tree<U>, E> {
        match self {
            Tree::Leaf => Ok(Tree::Leaf),
            Tree::Node(l, v, r) => {
                let l2 = l.traverse_result(f)?;
                let v2 = f(v)?;
                let r2 = r.traverse_result(f)?;
                Ok(Tree::node(l2, v2, r2))
            }
        }
    }

    // Approach 3: Pure map (no effect)
    fn map<U>(&self, f: &impl Fn(&T) -> U) -> Tree<U> {
        match self {
            Tree::Leaf => Tree::Leaf,
            Tree::Node(l, v, r) => Tree::node(l.map(f), f(v), r.map(f)),
        }
    }

    fn to_vec(&self) -> Vec<&T> {
        match self {
            Tree::Leaf => vec![],
            Tree::Node(l, v, r) => {
                let mut result = l.to_vec();
                result.push(v);
                result.extend(r.to_vec());
                result
            }
        }
    }
}

fn safe_double(x: &i32) -> Option<i32> {
    if *x > 50 {
        None
    } else {
        Some(x * 2)
    }
}

fn parse_positive(x: &i32) -> Result<i32, String> {
    if *x > 0 {
        Ok(*x)
    } else {
        Err(format!("Not positive: {}", x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Tree<i32> {
        Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        )
    }

    #[test]
    fn test_traverse_option_success() {
        let result = sample().traverse_option(&safe_double);
        let expected = Tree::node(
            Tree::node(Tree::Leaf, 2, Tree::Leaf),
            4,
            Tree::node(Tree::Leaf, 6, Tree::Leaf),
        );
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_traverse_option_failure() {
        let tree = Tree::node(Tree::node(Tree::Leaf, 10, Tree::Leaf), 60, Tree::Leaf);
        assert_eq!(tree.traverse_option(&safe_double), None);
    }

    #[test]
    fn test_traverse_result_success() {
        assert_eq!(sample().traverse_result(&parse_positive), Ok(sample()));
    }

    #[test]
    fn test_traverse_result_failure() {
        let tree = Tree::node(Tree::Leaf, -1, Tree::Leaf);
        assert_eq!(
            tree.traverse_result(&parse_positive),
            Err("Not positive: -1".into())
        );
    }

    #[test]
    fn test_map() {
        let doubled = sample().map(&|x| x * 2);
        assert_eq!(doubled.to_vec(), vec![&2, &4, &6]);
    }

    #[test]
    fn test_traverse_leaf() {
        assert_eq!(
            Tree::<i32>::Leaf.traverse_option(&safe_double),
            Some(Tree::Leaf)
        );
    }
}
