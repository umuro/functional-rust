// Find the last two elements of a list

// Idiomatic Rust with slice patterns
fn last_two<T>(list: &[T]) -> Option<(&T, &T)> {
    match list {
        [.., a, b] => Some((a, b)),
        _ => None,
    }
}

// Alternative: using split_last
fn last_two_split<T>(list: &[T]) -> Option<(&T, &T)> {
    let (last, rest) = list.split_last()?;
    let (second_last, _) = rest.split_last()?;
    Some((second_last, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(last_two::<i32>(&[]), None);
        assert_eq!(last_two(&[1]), None);
        assert_eq!(last_two(&[1, 2]), Some((&1, &2)));
        assert_eq!(last_two(&[1, 2, 3, 4]), Some((&3, &4)));
    }
}

fn main() {
    println!("last_two([1,2,3,4]) = {:?}", last_two(&[1, 2, 3, 4]));
    println!("✓ Rust tests passed");
}
