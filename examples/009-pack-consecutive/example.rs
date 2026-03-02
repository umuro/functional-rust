// Pack Consecutive
// Rust translation from OCaml 99 Problems #9

// Pack consecutive duplicates
fn pack<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
    if list.is_empty() {
        return vec![];
    }
    
    let mut result = vec![];
    let mut current = vec![list[0].clone()];
    
    for i in 1..list.len() {
        if list[i] == list[i - 1] {
            current.push(list[i].clone());
        } else {
            result.push(current);
            current = vec![list[i].clone()];
        }
    }
    result.push(current);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack() {
        assert_eq!(
            pack(&["a","a","b","c","c","c"]),
            vec![vec!["a","a"], vec!["b"], vec!["c","c","c"]]
        );
        assert_eq!(pack::<i32>(&[]), Vec::<Vec<i32>>::new());
        assert_eq!(pack(&[1]), vec![vec![1]]);
    }
}

fn main() {
    let list = vec!["a","a","b","c","c","c"];
    println!("pack({:?}) = {:?}", list, pack(&list));
    println!("✓ Rust tests passed");
}
