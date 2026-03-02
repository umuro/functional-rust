// Run-length Encoding
// Rust translation from OCaml 99 Problems #10

// Run-length encoding
fn encode<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    pack(list)
        .into_iter()
        .map(|group| (group.len(), group[0].clone()))
        .collect()
}

// Helper: pack function (from example 009)
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
    fn test_encode() {
        assert_eq!(
            encode(&["a","a","b","c","c","c"]),
            vec![(2,"a"), (1,"b"), (3,"c")]
        );
        assert_eq!(encode::<i32>(&[]), Vec::<(usize, i32)>::new());
        assert_eq!(encode(&[1]), vec![(1, 1)]);
    }
}

fn main() {
    let list = vec!["a","a","b","c","c","c"];
    println!("encode({:?}) = {:?}", list, encode(&list));
    println!("✓ Rust tests passed");
}
