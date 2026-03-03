// A monoid viewed as a single-object category:
//   - One object: ()
//   - Morphisms: the monoid elements
//   - Composition: monoid append (associative)
//   - Identity: monoid empty

pub trait Monoid {
    fn empty() -> Self;
    fn append(self, other: Self) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringMonoid(pub String);

impl Monoid for StringMonoid {
    fn empty() -> Self {
        StringMonoid(String::new())
    }
    fn append(self, other: Self) -> Self {
        StringMonoid(self.0 + &other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SumMonoid(pub i64);

impl Monoid for SumMonoid {
    fn empty() -> Self {
        SumMonoid(0)
    }
    fn append(self, other: Self) -> Self {
        SumMonoid(self.0 + other.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListMonoid<T>(pub Vec<T>);

impl<T: Clone> Monoid for ListMonoid<T> {
    fn empty() -> Self {
        ListMonoid(Vec::new())
    }
    fn append(self, other: Self) -> Self {
        let mut v = self.0;
        v.extend(other.0);
        ListMonoid(v)
    }
}

pub fn check_left_identity<M: Monoid + PartialEq + Clone>(x: M) -> bool {
    M::empty().append(x.clone()) == x
}

pub fn check_right_identity<M: Monoid + PartialEq + Clone>(x: M) -> bool {
    x.clone().append(M::empty()) == x
}

pub fn check_associativity<M: Monoid + PartialEq + Clone>(x: M, y: M, z: M) -> bool {
    x.clone()
        .append(y.clone())
        .append(z.clone())
        == x.append(y.append(z))
}

/// Compose a sequence of morphisms (monoid elements) — mirrors OCaml's fold_left.
pub fn compose_morphisms<M: Monoid>(morphisms: impl IntoIterator<Item = M>) -> M {
    morphisms
        .into_iter()
        .fold(M::empty(), |acc, m| acc.append(m))
}

/// Recursive style, closer to OCaml pattern matching.
pub fn compose_morphisms_recursive<M: Monoid + Clone>(morphisms: &[M]) -> M {
    match morphisms {
        [] => M::empty(),
        [x] => x.clone(),
        [head, rest @ ..] => head.clone().append(compose_morphisms_recursive(rest)),
    }
}

fn main() {
    // Verify monoid laws
    println!(
        "String left-identity:  {}",
        check_left_identity(StringMonoid("hello".into()))
    );
    println!(
        "String right-identity: {}",
        check_right_identity(StringMonoid("hello".into()))
    );
    println!(
        "String associativity:  {}",
        check_associativity(
            StringMonoid("a".into()),
            StringMonoid("b".into()),
            StringMonoid("c".into())
        )
    );

    println!(
        "Sum identity:          {}",
        check_left_identity(SumMonoid(42)) && check_right_identity(SumMonoid(42))
    );

    // Monoid as category: compose morphisms (fold of morphisms)
    let words = vec![
        StringMonoid("hello".into()),
        StringMonoid(", ".into()),
        StringMonoid("world".into()),
        StringMonoid("!".into()),
    ];
    let composed = compose_morphisms(words);
    println!("Composed string:       {:?}", composed.0);

    let nums = vec![SumMonoid(1), SumMonoid(2), SumMonoid(3), SumMonoid(4)];
    let sum = compose_morphisms_recursive(&nums);
    println!("Sum of [1,2,3,4]:      {}", sum.0);

    let lists = vec![
        ListMonoid(vec![1, 2]),
        ListMonoid(vec![3, 4]),
        ListMonoid(vec![5]),
    ];
    let merged = compose_morphisms(lists);
    println!("Merged lists:          {:?}", merged.0);
}

/* Output:
   String left-identity:  true
   String right-identity: true
   String associativity:  true
   Sum identity:          true
   Composed string:       "hello, world!"
   Sum of [1,2,3,4]:      10
   Merged lists:          [1, 2, 3, 4, 5]
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_left_identity() {
        assert!(check_left_identity(StringMonoid("hello".into())));
    }

    #[test]
    fn test_string_right_identity() {
        assert!(check_right_identity(StringMonoid("hello".into())));
    }

    #[test]
    fn test_string_associativity() {
        assert!(check_associativity(
            StringMonoid("a".into()),
            StringMonoid("b".into()),
            StringMonoid("c".into()),
        ));
    }

    #[test]
    fn test_string_compose_morphisms() {
        let words = vec![
            StringMonoid("hello".into()),
            StringMonoid(", ".into()),
            StringMonoid("world".into()),
            StringMonoid("!".into()),
        ];
        assert_eq!(
            compose_morphisms(words),
            StringMonoid("hello, world!".into())
        );
    }
}
