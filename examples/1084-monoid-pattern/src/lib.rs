pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct Sum(pub i32);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)
    }

    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Product(pub i32);

impl Monoid for Product {
    fn empty() -> Self {
        Product(1)
    }

    fn combine(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

pub fn reduce_monoid<T: Monoid + Clone>(items: &[T]) -> T {
    items.iter().cloned().fold(T::empty(), |acc, x| acc.combine(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_monoid() {
        let nums = vec![Sum(1), Sum(2), Sum(3)];
        assert_eq!(reduce_monoid(&nums), Sum(6));
        assert_eq!(Sum::empty().combine(Sum(5)), Sum(5));
    }

    #[test]
    fn test_product_monoid() {
        let nums = vec![Product(1), Product(2), Product(3)];
        assert_eq!(reduce_monoid(&nums), Product(6));
        assert_eq!(Product::empty().combine(Product(5)), Product(5));
    }

    #[test]
    fn test_empty_sum() {
        let nums: Vec<Sum> = vec![];
        assert_eq!(reduce_monoid(&nums), Sum(0));
    }

    #[test]
    fn test_empty_product() {
        let nums: Vec<Product> = vec![];
        assert_eq!(reduce_monoid(&nums), Product(1));
    }
}
