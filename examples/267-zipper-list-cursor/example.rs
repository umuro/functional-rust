#[derive(Debug, Clone, PartialEq)]
pub struct Zipper<T> {
    pub left: Vec<T>,
    pub focus: T,
    pub right: Vec<T>,
}

pub fn of_slice<T: Clone>(slice: &[T]) -> Option<Zipper<T>> {
    let mut iter = slice.iter().cloned();
    let focus = iter.next()?;
    Some(Zipper { left: vec![], focus, right: iter.collect() })
}

pub fn go_right<T>(z: Zipper<T>) -> Option<Zipper<T>> {
    let mut right = z.right;
    if right.is_empty() { return None; }
    let new_focus = right.remove(0);
    let mut left = z.left;
    left.insert(0, z.focus);
    Some(Zipper { left, focus: new_focus, right })
}

pub fn go_left<T>(z: Zipper<T>) -> Option<Zipper<T>> {
    let mut left = z.left;
    if left.is_empty() { return None; }
    let new_focus = left.remove(0);
    let mut right = z.right;
    right.insert(0, z.focus);
    Some(Zipper { left, focus: new_focus, right })
}

pub fn update<T, F: FnOnce(T) -> T>(z: Zipper<T>, f: F) -> Zipper<T> {
    Zipper { focus: f(z.focus), ..z }
}

pub fn to_vec<T>(z: Zipper<T>) -> Vec<T> {
    let mut result: Vec<T> = z.left.into_iter().rev().collect();
    result.push(z.focus);
    result.extend(z.right);
    result
}

fn main() {
    let z = of_slice(&[1, 2, 3, 4, 5]).unwrap();
    println!("Initial focus: {}", z.focus);

    let z = go_right(z).unwrap();
    println!("After go_right: focus = {}", z.focus);

    let z = go_right(z).unwrap();
    println!("After go_right: focus = {}", z.focus);

    let z = update(z, |x| x * 10);
    println!("After update (*10): focus = {}", z.focus);

    let list = to_vec(z);
    println!("Full list: {:?}", list);

    // Demonstrate go_left
    let z2 = of_slice(&[10, 20, 30]).unwrap();
    let z2 = go_right(z2).unwrap();
    let z2 = go_right(z2).unwrap();
    let z2 = go_left(z2).unwrap();
    println!("After two rights then left: focus = {}", z2.focus);
}

/* Output:
   Initial focus: 1
   After go_right: focus = 2
   After go_right: focus = 3
   After update (*10): focus = 30
   Full list: [1, 2, 30, 4, 5]
   After two rights then left: focus = 20
*/
