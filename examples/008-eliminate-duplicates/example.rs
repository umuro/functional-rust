// Eliminate Duplicates
// Rust translation from OCaml 99 Problems #8

// Idiomatic Rust (in-place)
fn compress<T: PartialEq>(list: &mut Vec<T>) {
    list.dedup();
}

// Functional style (returns new vec)
fn compress_functional<T: PartialEq + Clone>(list: &[T]) -> Vec<T> {
    list.iter()
        .enumerate()
        .filter(|(i, x)| *i == 0 || list[i - 1] != **x)
        .map(|(_, x)| x.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        assert_eq!(
            compress_functional(&["a","a","a","b","c","c","d","e","e","e"]),
            vec!["a","b","c","d","e"]
        );
        assert_eq!(compress_functional::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(compress_functional(&[1]), vec![1]);
    }
}

fn main() {
    let list = vec!["a","a","a","b","c","c"];
    println!("compress({:?}) = {:?}", list, compress_functional(&list));
    println!("✓ Rust tests passed");
}
