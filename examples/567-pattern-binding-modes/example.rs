#[derive(Debug)]
enum Tree { Leaf, Node(i32, Box<Tree>, Box<Tree>) }

fn sum(t: &Tree) -> i32 {
    match t {
        Tree::Leaf          => 0,
        Tree::Node(v, l, r) => v + sum(l) + sum(r), // auto-deref through Box
    }
}

// Explicit ref in let binding
fn borrow_without_move() {
    let s = String::from("hello");
    let ref r = s;      // r: &String, s still owned
    println!("borrowed: {}, still own: {}", r, s);
}

// ref in slice pattern
fn first_and_rest(v: &[String]) -> Option<(&str, &[String])> {
    match v {
        [ref head, ref rest @ ..] => Some((head, rest)),
        [] => None,
    }
}

// ref mut
fn double_first(v: &mut [i32]) {
    if let [ref mut first, ..] = v { *first *= 2; }
}

fn main() {
    let t = Tree::Node(1,
        Box::new(Tree::Node(2, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
        Box::new(Tree::Node(3, Box::new(Tree::Leaf), Box::new(Tree::Leaf))));
    println!("sum = {}", sum(&t));
    borrow_without_move();
    let words: Vec<String> = ["a","b","c"].iter().map(|s|s.to_string()).collect();
    if let Some((h,rest)) = first_and_rest(&words) {
        println!("head={} rest_len={}", h, rest.len());
    }
    let mut nums = vec![10, 20, 30];
    double_first(&mut nums);
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_sum() {
        let t = Tree::Node(5, Box::new(Tree::Node(3,Box::new(Tree::Leaf),Box::new(Tree::Leaf))), Box::new(Tree::Leaf));
        assert_eq!(sum(&t), 8);
    }
    #[test] fn test_double() {
        let mut v = vec![5,6,7]; double_first(&mut v); assert_eq!(v[0], 10);
    }
}
