// 1109: Monoid Pattern — Generic Combining
//
// Demonstrates translating OCaml's first-class module types (MONOID) to
// Rust traits + newtype wrappers.

pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [head, tail @ ..] => head.clone().combine(concat_all_recursive(tail)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum(pub i32);

impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product(pub i32);

impl Monoid for Product {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct All(pub bool);

impl Monoid for All {
    fn empty() -> Self { All(true) }
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}

impl Monoid for String {
    fn empty() -> Self { String::new() }
    fn combine(self, other: Self) -> Self { self + &other }
}

fn main() {
    let sum = concat_all([1, 2, 3, 4, 5].map(Sum));
    println!("sum: {}", sum.0);

    let product = concat_all([1, 2, 3, 4, 5].map(Product));
    println!("product: {}", product.0);

    let concat = concat_all(["hello", " ", "world"].map(str::to_string));
    println!("concat: {}", concat);

    let all = concat_all([true, true, false].map(All));
    println!("all: {}", all.0);

    // Recursive version produces the same result
    let items = [1, 2, 3, 4, 5].map(Sum);
    let recursive_sum = concat_all_recursive(&items);
    println!("sum (recursive): {}", recursive_sum.0);
}

/* Output:
   sum: 15
   product: 120
   concat: hello world
   all: false
   sum (recursive): 15
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiple() {
        assert_eq!(concat_all([1, 2, 3, 4, 5].map(Sum)), Sum(15));
    }

    #[test]
    fn test_product_multiple() {
        assert_eq!(concat_all([1, 2, 3, 4, 5].map(Product)), Product(120));
    }

    #[test]
    fn test_concat_strings() {
        assert_eq!(
            concat_all(["hello", " ", "world"].map(str::to_string)),
            "hello world"
        );
    }

    #[test]
    fn test_all_with_false() {
        assert_eq!(concat_all([true, true, false].map(All)), All(false));
    }

    #[test]
    fn test_empty_returns_identity() {
        assert_eq!(concat_all(std::iter::empty::<Sum>()), Sum(0));
        assert_eq!(concat_all(std::iter::empty::<Product>()), Product(1));
        assert_eq!(concat_all(std::iter::empty::<All>()), All(true));
    }
}
