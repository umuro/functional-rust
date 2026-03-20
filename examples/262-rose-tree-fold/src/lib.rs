#![allow(clippy::all)]
//! Rose Tree — Multi-Way Tree with Fold
//!
//! OCaml: `type 'a rose = Rose of 'a * 'a rose list`
//! Rust: `struct Rose<T> { value: T, children: Vec<Rose<T>> }`
//!
//! A rose tree (multi-way tree) where each node holds a value and
//! an arbitrary number of children. The fold operation processes
//! the tree bottom-up, combining child results with the node value.

//! A rose tree: each node has a value and zero or more children.
#[derive(Debug, Clone, PartialEq)]
pub struct Rose<T> {
    pub value: T,
    pub children: Vec<Rose<T>>,
}

impl<T> Rose<T> {
    /// Creates a leaf node (no children).
    pub fn leaf(value: T) -> Self {
        Rose {
            value,
            children: vec![],
        }
    }

    /// Creates a node with children.
    pub fn node(value: T, children: Vec<Rose<T>>) -> Self {
        Rose { value, children }
    }

    /// Recursive fold over the rose tree.
    ///
    /// OCaml: `let rec fold f (Rose (x, children)) = f x (List.map (fold f) children)`
    ///
    /// The function `f` receives the node value and a Vec of results
    /// from folding all children. This processes the tree bottom-up.
    pub fn fold<R>(&self, f: &dyn Fn(&T, Vec<R>) -> R) -> R {
        let child_results: Vec<R> = self.children.iter().map(|c| c.fold(f)).collect();
        f(&self.value, child_results)
    }
}

/// Counts total nodes in the tree.
/// OCaml: `let size = fold (fun _ sizes -> 1 + List.fold_left (+) 0 sizes)`
pub fn size<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, sizes: Vec<usize>| 1 + sizes.iter().sum::<usize>())
}

/// Computes the depth (height) of the tree.
/// OCaml: `let depth = fold (fun _ depths -> 1 + List.fold_left max 0 depths)`
pub fn depth<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, depths: Vec<usize>| 1 + depths.iter().copied().max().unwrap_or(0))
}

/// Converts tree to a string representation.
/// OCaml: `let to_string = fold (fun x strs -> match strs with | [] -> x | _ -> ...)`
pub fn to_string_repr(tree: &Rose<&str>) -> String {
    tree.fold(&|&x, strs: Vec<String>| {
        if strs.is_empty() {
            x.to_string()
        } else {
            format!("{}({})", x, strs.join(","))
        }
    })
}

/// Iterator-based approach: collects all values in pre-order.
pub fn preorder<T: Clone>(tree: &Rose<T>) -> Vec<T> {
    let mut result = vec![tree.value.clone()];
    for child in &tree.children {
        result.extend(preorder(child));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Rose<&'static str> {
        Rose::node(
            "a",
            vec![
                Rose::node("b", vec![Rose::leaf("d"), Rose::leaf("e")]),
                Rose::node("c", vec![Rose::leaf("f")]),
            ],
        )
    }

    #[test]
    fn test_empty_leaf() {
        let tree = Rose::leaf(42);
        assert_eq!(size(&tree), 1);
        assert_eq!(depth(&tree), 1);
    }

    #[test]
    fn test_size() {
        let tree = sample_tree();
        assert_eq!(size(&tree), 6);
    }

    #[test]
    fn test_depth() {
        let tree = sample_tree();
        assert_eq!(depth(&tree), 3);
    }

    #[test]
    fn test_to_string() {
        let tree = sample_tree();
        assert_eq!(to_string_repr(&tree), "a(b(d,e),c(f))");
    }

    #[test]
    fn test_preorder() {
        let tree = sample_tree();
        assert_eq!(preorder(&tree), vec!["a", "b", "d", "e", "c", "f"]);
    }

    #[test]
    fn test_single_branch() {
        let tree = Rose::node("x", vec![Rose::node("y", vec![Rose::leaf("z")])]);
        assert_eq!(size(&tree), 3);
        assert_eq!(depth(&tree), 3);
        assert_eq!(to_string_repr(&tree), "x(y(z))");
    }
}
