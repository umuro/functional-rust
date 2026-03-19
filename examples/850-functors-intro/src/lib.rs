// Example 051: Functors Introduction
// A Functor is a type that supports mapping a function over its contents

// Approach 1: Custom Maybe type with map
#[derive(Debug, PartialEq, Clone)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(f(x)),
        }
    }

    fn pure(x: T) -> Maybe<T> {
        Maybe::Just(x)
    }
}

// Approach 2: Functor trait (simulating OCaml's module type)
trait Functor {
    type Inner;
    type Mapped<U>: Functor;
    fn fmap<U, F: FnOnce(Self::Inner) -> U>(self, f: F) -> Self::Mapped<U>;
}

#[derive(Debug, PartialEq, Clone)]
struct Box_<T>(T);

impl<T> Functor for Box_<T> {
    type Inner = T;
    type Mapped<U> = Box_<U>;
    fn fmap<U, F: FnOnce(T) -> U>(self, f: F) -> Box_<U> {
        Box_(f(self.0))
    }
}

impl<T> Functor for Maybe<T> {
    type Inner = T;
    type Mapped<U> = Maybe<U>;
    fn fmap<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        self.map(f)
    }
}

// Approach 3: Functor for a tree type
#[derive(Debug, PartialEq, Clone)]
enum Tree<T> {
    Leaf,
    Node(std::boxed::Box<Tree<T>>, T, std::boxed::Box<Tree<T>>),
}

impl<T> Tree<T> {
    fn map<U, F: Fn(&T) -> U>(&self, f: &F) -> Tree<U>
    where
        T: Clone,
    {
        match self {
            Tree::Leaf => Tree::Leaf,
            Tree::Node(l, v, r) => Tree::Node(
                std::boxed::Box::new(l.map(f)),
                f(v),
                std::boxed::Box::new(r.map(f)),
            ),
        }
    }

    fn node(l: Tree<T>, v: T, r: Tree<T>) -> Tree<T> {
        Tree::Node(std::boxed::Box::new(l), v, std::boxed::Box::new(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_map_just() {
        assert_eq!(Maybe::Just(5).map(|n| n * 2), Maybe::Just(10));
    }

    #[test]
    fn test_maybe_map_nothing() {
        let n: Maybe<i32> = Maybe::Nothing;
        assert_eq!(n.map(|x| x * 2), Maybe::Nothing);
    }

    #[test]
    fn test_maybe_map_type_change() {
        assert_eq!(Maybe::Just("hello").map(|s| s.len()), Maybe::Just(5));
    }

    #[test]
    fn test_box_functor() {
        let b = Box_(42);
        assert_eq!(b.fmap(|x| x + 1), Box_(43));
    }

    #[test]
    fn test_tree_map() {
        let t = Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        );
        let expected = Tree::node(
            Tree::node(Tree::Leaf, 10, Tree::Leaf),
            20,
            Tree::node(Tree::Leaf, 30, Tree::Leaf),
        );
        assert_eq!(t.map(&|x: &i32| x * 10), expected);
    }

    #[test]
    fn test_chained_maps() {
        let result = Maybe::Just(3)
            .map(|x| x + 1)
            .map(|x| x * 2)
            .map(|x| x.to_string());
        assert_eq!(result, Maybe::Just("8".to_string()));
    }
}
