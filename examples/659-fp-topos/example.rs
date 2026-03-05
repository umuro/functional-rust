// Topos concepts in Rust

// Subobject classifier
#[derive(Debug, PartialEq)]
enum Omega { True, False }

// Characteristic function (subset membership)
fn char_fn<T: PartialEq>(subset: &[T]) -> impl Fn(&T) -> Omega + '_ {
    move |x| if subset.contains(x) { Omega::True } else { Omega::False }
}

// Power object operations
fn union<T: Clone + Eq>(a: &[T], b: &[T]) -> Vec<T> {
    let mut result = a.to_vec();
    for x in b {
        if !result.contains(x) { result.push(x.clone()); }
    }
    result
}

fn intersection<T: Clone + Eq>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().filter(|x| b.contains(x)).cloned().collect()
}

fn main() {
    let subset = vec![1, 3, 5];
    let chi = char_fn(&subset);
    
    for x in 1..=5 {
        println!("χ({}) = {:?}", x, chi(&x));
    }
    
    let a = vec![1, 2, 3];
    let b = vec![2, 3, 4];
    println!("Union: {:?}", union(&a, &b));
    println!("Intersection: {:?}", intersection(&a, &b));
}
