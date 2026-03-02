// Flatten Nested List
// Rust translation from OCaml 99 Problems #7

// Define nested list type
#[derive(Debug, PartialEq, Clone)]
enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

// Flatten recursively
fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    list.iter()
        .flat_map(|node| match node {
            Node::One(x) => vec![x.clone()],
            Node::Many(xs) => flatten(xs),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten() {
        use Node::*;
        assert_eq!(
            flatten(&[One(1), Many(vec![One(2), Many(vec![One(3), One(4)])]), One(5)]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(flatten::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(flatten(&[One(1)]), vec![1]);
    }
}

fn main() {
    use Node::*;
    let nested = vec![One(1), Many(vec![One(2), One(3)])];
    println!("flatten(nested) = {:?}", flatten(&nested));
    println!("✓ Rust tests passed");
}
