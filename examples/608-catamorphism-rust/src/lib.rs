//! # Catamorphism (Fold)
//! Generalized fold over recursive structures.

pub fn cata_list<A, B>(list: &[A], nil: B, cons: impl Fn(&A, B) -> B) -> B {
    list.iter().rev().fold(nil, |acc, x| cons(x, acc))
}

pub fn sum(xs: &[i32]) -> i32 {
    cata_list(xs, 0, |x, acc| x + acc)
}
pub fn product(xs: &[i32]) -> i32 {
    cata_list(xs, 1, |x, acc| x * acc)
}
pub fn length<A>(xs: &[A]) -> usize {
    cata_list(xs, 0, |_, acc| acc + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3]), 6);
    }
    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4]), 24);
    }
    #[test]
    fn test_length() {
        assert_eq!(length(&[1, 2, 3]), 3);
    }
}
