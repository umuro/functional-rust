// Example 083: Display Trait
// OCaml to_string → Rust fmt::Display

use std::fmt;

// === Approach 1: Simple Display implementations ===
enum Color {
    Red,
    Green,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// Debug is for developers, Display is for users
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point{{ x: {}, y: {} }}", self.x, self.y)
    }
}

// === Approach 2: Multi-line Display (matrix) ===
struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        Matrix { data }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            write!(f, "| ")?;
            for (j, val) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:6.2}", val)?;
            }
            write!(f, " |")?;
            if i < self.data.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

// === Approach 3: Recursive Display (tree) ===
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T: fmt::Display> Tree<T> {
    fn node(left: Tree<T>, value: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), value, Box::new(right))
    }
}

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Leaf => write!(f, "."),
            Tree::Node(l, v, r) => write!(f, "({} {} {})", l, v, r),
        }
    }
}

// Display enables .to_string() automatically
fn print_all<T: fmt::Display>(items: &[T]) -> String {
    items
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!(Color::Red.to_string(), "Red");
        assert_eq!(format!("{}", Color::Blue), "Blue");
    }

    #[test]
    fn test_point_display() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(format!("{}", p), "(1.00, 2.00)");
    }

    #[test]
    fn test_point_debug() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(format!("{:?}", p), "Point{ x: 1, y: 2 }");
    }

    #[test]
    fn test_matrix_display() {
        let m = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let s = format!("{}", m);
        assert!(s.contains("1.00"));
        assert!(s.contains("4.00"));
    }

    #[test]
    fn test_tree_display() {
        let tree = Tree::node(Tree::Leaf, 42, Tree::Leaf);
        assert_eq!(format!("{}", tree), "(. 42 .)");
    }

    #[test]
    fn test_nested_tree() {
        let tree = Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        );
        assert_eq!(format!("{}", tree), "((. 1 .) 2 (. 3 .))");
    }

    #[test]
    fn test_to_string() {
        let p = Point { x: 0.0, y: 0.0 };
        let s: String = p.to_string();
        assert_eq!(s, "(0.00, 0.00)");
    }

    #[test]
    fn test_print_all() {
        let colors = vec![Color::Red, Color::Green];
        assert_eq!(print_all(&colors), "Red, Green");
    }
}
