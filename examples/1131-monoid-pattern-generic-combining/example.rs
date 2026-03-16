pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

/// Fold a collection of monoidal values into one.
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

/// Recursive version — closer to OCaml's `List.fold_left`.
pub fn concat_all_recursive<M: Monoid>(items: Vec<M>) -> M {
    fn go<M: Monoid>(acc: M, mut rest: Vec<M>) -> M {
        if rest.is_empty() {
            acc
        } else {
            let head = rest.remove(0);
            go(M::combine(acc, head), rest)
        }
    }
    go(M::empty(), items)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Concat(pub String);
impl Monoid for Concat {
    fn empty() -> Self { Concat(String::new()) }
    fn combine(self, other: Self) -> Self { Concat(self.0 + &other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct All(pub bool);
impl Monoid for All {
    fn empty() -> Self { All(true) }
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}

fn main() {
    let sum = concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
    println!("sum: {}", sum.0);

    let product = concat_all([Product(1), Product(2), Product(3), Product(4), Product(5)]);
    println!("product: {}", product.0);

    let concat = concat_all([Concat("hello".into()), Concat(" ".into()), Concat("world".into())]);
    println!("concat: {}", concat.0);

    let all = concat_all([All(true), All(true), All(false)]);
    println!("all: {}", all.0);

    // Recursive variant
    let sum_rec = concat_all_recursive(vec![Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
    println!("sum (recursive): {}", sum_rec.0);
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
    fn test_sum_empty_list() {
        assert_eq!(concat_all::<Sum>([]), Sum(0));
    }

    #[test]
    fn test_sum_multiple_elements() {
        assert_eq!(concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]), Sum(15));
    }

    #[test]
    fn test_product_multiple_elements() {
        assert_eq!(
            concat_all([Product(1), Product(2), Product(3), Product(4), Product(5)]),
            Product(120)
        );
    }

    #[test]
    fn test_concat_multiple_strings() {
        assert_eq!(
            concat_all([Concat("hello".into()), Concat(" ".into()), Concat("world".into())]),
            Concat("hello world".into())
        );
    }

    #[test]
    fn test_all_contains_false() {
        assert_eq!(concat_all([All(true), All(true), All(false)]), All(false));
    }

    #[test]
    fn test_recursive_matches_fold() {
        assert_eq!(
            concat_all_recursive(vec![Sum(1), Sum(2), Sum(3)]),
            concat_all([Sum(1), Sum(2), Sum(3)])
        );
    }
}
