// Natural transformation: Vec → Option (take the head element)
pub fn list_to_option<T>(list: &[T]) -> Option<&T> {
    list.first()
}

// Natural transformation: Vec → Option (functional/recursive style)
pub fn list_to_option_rec<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [head, ..] => Some(head),
    }
}

// Natural transformation: Option → Vec
pub fn option_to_list<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],
        Some(x) => vec![x],
    }
}

// Verify the naturality condition for `list_to_option`:
//   list_to_option(map f lst)  ==  map f (list_to_option lst)
pub fn naturality_holds<T, U, F>(list: &[T], f: F) -> bool
where
    T: Clone,
    U: PartialEq,
    F: Fn(T) -> U,
{
    let mapped: Vec<U> = list.iter().cloned().map(&f).collect();
    let lhs = mapped.first();
    let rhs = list.first().cloned().map(f);
    lhs == rhs.as_ref()
}

fn main() {
    // Natural transformation: list → option
    println!(
        "list_to_option([1,2,3]) = {:?}",
        list_to_option(&[1, 2, 3])
    );
    println!("list_to_option([]) = {:?}", list_to_option::<i32>(&[]));

    // Natural transformation: option → list
    println!("option_to_list(Some(42)) = {:?}", option_to_list(Some(42)));
    println!("option_to_list(None) = {:?}", option_to_list::<i32>(None));

    // Verify the naturality condition: map f . nat = nat . map f
    let holds = naturality_holds(&[1i32, 2, 3], |x| x * 2);
    println!("Naturality condition holds for [1,2,3] with f=(*2): {holds}");

    let holds_empty = naturality_holds::<i32, i32, _>(&[], |x| x * 2);
    println!("Naturality condition holds for [] with f=(*2): {holds_empty}");

    // Recursive version matches idiomatic
    println!(
        "list_to_option_rec([1,2,3]) = {:?}",
        list_to_option_rec(&[1, 2, 3])
    );
}

/* Output:
   list_to_option([1,2,3]) = Some(1)
   list_to_option([]) = None
   option_to_list(Some(42)) = [42]
   option_to_list(None) = []
   Naturality condition holds for [1,2,3] with f=(*2): true
   Naturality condition holds for [] with f=(*2): true
   list_to_option_rec([1,2,3]) = Some(1)
*/

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
        assert!(naturality_holds(&[1i32, 2, 3], |x| x * 2));
    }

    #[test]
    fn test_naturality_empty() {
        assert!(naturality_holds::<i32, i32, _>(&[], |x| x * 2));
    }
}
