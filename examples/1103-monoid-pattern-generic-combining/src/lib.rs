/// Monoid trait — a type with an identity element and an associative combine.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

/// Fold a list using a Monoid, starting from the identity element.
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Monoid for i32 {
        fn empty() -> Self {
            0
        }
        fn combine(self, other: Self) -> Self {
            self + other
        }
    }

    impl Monoid for String {
        fn empty() -> Self {
            String::new()
        }
        fn combine(self, other: Self) -> Self {
            self + &other
        }
    }

    #[test]
    fn test_sum() {
        assert_eq!(concat_all([1i32, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_empty_sum() {
        assert_eq!(concat_all(std::iter::empty::<i32>()), 0);
    }

    #[test]
    fn test_concat_strings() {
        let words = ["hello".to_string(), " ".to_string(), "world".to_string()];
        assert_eq!(concat_all(words), "hello world");
    }

    #[test]
    fn test_single_element() {
        assert_eq!(concat_all([42i32]), 42);
    }
}
