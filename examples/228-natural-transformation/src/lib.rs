//! Natural Transformations: structure-preserving maps between functors.
//!
//! A natural transformation η: F → G between functors F, G: C → D
//! assigns to each object A in C a morphism η_A: F(A) → G(A),
//! such that for every morphism f: A → B in C, the naturality square commutes:
//!   G(f) ∘ η_A = η_B ∘ F(f)
//!
//! In programming terms: `nat(fmap f xs) == fmap f (nat xs)`

/// Safe head: `&[T]` → `Option<T>` (a natural transformation from List to Option).
///
/// Idiomatic Rust: delegate to `slice::first()` and clone the element.
pub fn safe_head<T: Clone>(list: &[T]) -> Option<T> {
    list.first().cloned()
}

/// Safe head — recursive, OCaml-style pattern matching.
pub fn safe_head_recursive<T: Clone>(list: &[T]) -> Option<T> {
    match list {
        [] => None,
        [x, ..] => Some(x.clone()),
    }
}

/// Safe last: `&[T]` → `Option<T>` (another natural transformation from List to Option).
pub fn safe_last<T: Clone>(list: &[T]) -> Option<T> {
    list.last().cloned()
}

/// `Option<T>` → `Vec<T>` (natural transformation: singleton list or empty list).
///
/// This is the unit of the List monad restricted to Option.
pub fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],
        Some(x) => vec![x],
    }
}

/// Verify the naturality condition for a natural transformation `nat: &[T] → Option<T>`.
///
/// The naturality square commutes iff:
///   `nat_u(list.map(f)) == nat_t(list).map(f)`
///
/// Both `nat_t` and `nat_u` must be the same natural transformation,
/// instantiated at types `T` and `U` respectively.
pub fn verify_naturality<T, U>(
    f: impl Fn(T) -> U,
    nat_t: impl Fn(&[T]) -> Option<T>,
    nat_u: impl Fn(&[U]) -> Option<U>,
    list: &[T],
) -> bool
where
    T: Clone,
    U: PartialEq,
{
    // LHS: map f over the list first, then apply the nat transformation
    let mapped: Vec<U> = list.iter().map(|x| f(x.clone())).collect();
    let lhs = nat_u(&mapped);
    // RHS: apply the nat transformation first, then map f over the result
    // `f` is moved here after the shared borrow in the closure above was released
    let rhs = nat_t(list).map(f);
    lhs == rhs
}

/// Composed natural transformation: `&[T]` → `Vec<T>`
/// via `&[T]` -[safe_head]→ `Option<T>` -[option_to_vec]→ `Vec<T>`.
///
/// Demonstrates that natural transformations compose to yield another natural transformation.
pub fn nat_composed<T: Clone>(list: &[T]) -> Vec<T> {
    option_to_vec(safe_head(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_head_empty() {
        assert_eq!(safe_head::<i32>(&[]), None);
    }

    #[test]
    fn test_safe_head_single() {
        assert_eq!(safe_head(&[42]), Some(42));
    }

    #[test]
    fn test_safe_head_multiple() {
        assert_eq!(safe_head(&[1, 2, 3]), Some(1));
    }

    #[test]
    fn test_safe_head_recursive_agrees_with_idiomatic() {
        let cases: &[&[i32]] = &[&[], &[7], &[1, 2, 3], &[42, 0, -1]];
        for list in cases {
            assert_eq!(safe_head(list), safe_head_recursive(list));
        }
    }

    #[test]
    fn test_safe_last_empty() {
        assert_eq!(safe_last::<i32>(&[]), None);
    }

    #[test]
    fn test_safe_last_single() {
        assert_eq!(safe_last(&[99]), Some(99));
    }

    #[test]
    fn test_safe_last_multiple() {
        assert_eq!(safe_last(&[1, 2, 3]), Some(3));
    }

    #[test]
    fn test_option_to_vec_none() {
        assert_eq!(option_to_vec::<i32>(None), vec![]);
    }

    #[test]
    fn test_option_to_vec_some() {
        assert_eq!(option_to_vec(Some(5)), vec![5]);
    }

    #[test]
    fn test_naturality_safe_head_int_to_string() {
        let cases: &[&[i32]] = &[&[], &[1], &[1, 2, 3], &[42, 0, -1]];
        for list in cases {
            assert!(
                verify_naturality(|x: i32| x.to_string(), safe_head, safe_head, list),
                "naturality failed for {list:?}"
            );
        }
    }

    #[test]
    fn test_naturality_safe_last_int_to_int() {
        let cases: &[&[i32]] = &[&[], &[1], &[1, 2, 3], &[42, 0, -1]];
        for list in cases {
            assert!(
                verify_naturality(|x: i32| x * 2, safe_last, safe_last, list),
                "naturality failed for {list:?}"
            );
        }
    }

    #[test]
    fn test_nat_composed_nonempty() {
        assert_eq!(nat_composed(&[1, 2, 3]), vec![1]);
    }

    #[test]
    fn test_nat_composed_empty() {
        assert_eq!(nat_composed::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_nat_composed_single() {
        assert_eq!(nat_composed(&[42]), vec![42]);
    }
}
