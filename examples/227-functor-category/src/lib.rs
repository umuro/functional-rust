// In a functor category, objects are functors and morphisms are natural transformations.
// Rust models functors as generic types with map-like operations (Vec, Option).
// Natural transformations are polymorphic functions between two such types.

// ---------------------------------------------------------------------------
// Natural transformation: Vec → Option  (take the head element)
// ---------------------------------------------------------------------------

/// Solution 1: Idiomatic Rust — delegates to `slice::first`, borrows in/out.
pub fn list_to_option<T>(list: &[T]) -> Option<&T> {
    list.first()
}

/// Solution 2: Functional/recursive style, mirroring OCaml pattern match.
pub fn list_to_option_rec<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [head, ..] => Some(head),
    }
}

// ---------------------------------------------------------------------------
// Natural transformation: Option → Vec  (wrap or empty)
// ---------------------------------------------------------------------------

/// Both idiomatic and functional: a single match is already the Rust idiom.
pub fn option_to_list<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],
        Some(x) => vec![x],
    }
}

// ---------------------------------------------------------------------------
// Naturality condition
//
// A natural transformation η : F → G must commute with fmap:
//   G.map(f) ∘ η  =  η ∘ F.map(f)
//
// Concretely for list_to_option:
//   list_to_option(list.map(f))  ==  Option::map(f)(list_to_option(list))
// ---------------------------------------------------------------------------

/// Returns `true` when the naturality square commutes for `list_to_option` and `f`.
pub fn naturality_holds<T, U, F>(list: &[T], f: F) -> bool
where
    T: Clone,
    U: PartialEq,
    F: Fn(T) -> U,
{
    // lhs: apply f to every element, then take the head
    let mapped: Vec<U> = list.iter().cloned().map(&f).collect();
    let lhs = mapped.first();

    // rhs: take the head first, then apply f
    let rhs = list.first().cloned().map(f);

    // compare Option<&U>  vs  Option<&U>  (rhs.as_ref() gives &U)
    lhs == rhs.as_ref()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_to_option_empty() {
        assert_eq!(list_to_option::<i32>(&[]), None);
    }

    #[test]
    fn test_list_to_option_single() {
        assert_eq!(list_to_option(&[42]), Some(&42));
    }

    #[test]
    fn test_list_to_option_multiple() {
        assert_eq!(list_to_option(&[1, 2, 3]), Some(&1));
    }

    #[test]
    fn test_list_to_option_rec_matches_idiomatic() {
        let cases: &[&[i32]] = &[&[], &[1], &[1, 2, 3]];
        for &list in cases {
            assert_eq!(list_to_option(list), list_to_option_rec(list));
        }
    }

    #[test]
    fn test_option_to_list_none() {
        assert_eq!(option_to_list::<i32>(None), vec![]);
    }

    #[test]
    fn test_option_to_list_some() {
        assert_eq!(option_to_list(Some(42)), vec![42]);
    }

    #[test]
    fn test_naturality_nonempty() {
        // list_to_option([2,4,6]) == map(*2)(list_to_option([1,2,3]))
        assert!(naturality_holds(&[1i32, 2, 3], |x| x * 2));
    }

    #[test]
    fn test_naturality_empty() {
        // Both sides are None — naturality trivially holds
        assert!(naturality_holds::<i32, i32, _>(&[], |x| x * 2));
    }
}
