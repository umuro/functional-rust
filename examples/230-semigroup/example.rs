// A semigroup is a set with an associative binary operation.
// Like a monoid but without requiring an identity element.

pub trait Semigroup {
    fn append(self, other: Self) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyList<T>(pub Vec<T>);

impl<T: Clone> Semigroup for NonEmptyList<T> {
    fn append(self, other: Self) -> Self {
        let mut v = self.0;
        v.extend(other.0);
        NonEmptyList(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Min(pub i64);

impl Semigroup for Min {
    fn append(self, other: Self) -> Self {
        Min(self.0.min(other.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Max(pub i64);

impl Semigroup for Max {
    fn append(self, other: Self) -> Self {
        Max(self.0.max(other.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct First<T>(pub T);

impl<T> Semigroup for First<T> {
    fn append(self, _other: Self) -> Self {
        self
    }
}

pub fn sconcat<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    let (head, tail) = items.split_first()?;
    Some(
        tail.iter()
            .cloned()
            .fold(head.clone(), |acc, x| acc.append(x)),
    )
}

pub fn sconcat_recursive<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    match items {
        [] => None,
        [x] => Some(x.clone()),
        [head, rest @ ..] => sconcat_recursive(rest).map(|tail| head.clone().append(tail)),
    }
}

pub fn check_associativity<S: Semigroup + PartialEq + Clone>(a: S, b: S, c: S) -> bool {
    a.clone().append(b.clone()).append(c.clone()) == a.append(b.append(c))
}

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Min);
    println!("min of nums: {:?}", sconcat(&nums));

    let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Max);
    println!("max of nums: {:?}", sconcat(&nums));

    let words = [First("first"), First("second"), First("third")];
    println!("first word: {:?}", sconcat(&words));

    let lists = [
        NonEmptyList(vec![1, 2]),
        NonEmptyList(vec![3]),
        NonEmptyList(vec![4, 5]),
    ];
    println!("list concat: {:?}", sconcat(&lists));

    // Verify associativity law
    let holds = check_associativity(Min(3), Min(1), Min(4));
    println!("Associativity law holds: {holds}");

    // Recursive fold agrees with iterative (by associativity)
    let nums = [3, 1, 4, 1, 5].map(Min);
    println!(
        "sconcat == sconcat_recursive: {}",
        sconcat(&nums) == sconcat_recursive(&nums)
    );
}

/* Output:
   min of nums: Some(Min(1))
   max of nums: Some(Max(9))
   first word: Some(First("first"))
   list concat: Some(NonEmptyList([1, 2, 3, 4, 5]))
   Associativity law holds: true
   sconcat == sconcat_recursive: true
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sconcat() {
        let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Min);
        assert_eq!(sconcat(&nums), Some(Min(1)));
    }

    #[test]
    fn test_max_sconcat() {
        let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Max);
        assert_eq!(sconcat(&nums), Some(Max(9)));
    }

    #[test]
    fn test_first_sconcat() {
        let words = [First("first"), First("second"), First("third")];
        assert_eq!(sconcat(&words), Some(First("first")));
    }

    #[test]
    fn test_sconcat_empty_is_none() {
        let empty: &[Min] = &[];
        assert_eq!(sconcat(empty), None);
    }
}
