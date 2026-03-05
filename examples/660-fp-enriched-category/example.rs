// Enriched Categories in Rust

// Bool-enriched: Preorder
struct Preorder<T> {
    elements: Vec<T>,
    leq: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T> Preorder<T> {
    fn new<F: Fn(&T, &T) -> bool + 'static>(elements: Vec<T>, leq: F) -> Self {
        Preorder { elements, leq: Box::new(leq) }
    }
    
    fn is_related(&self, a: &T, b: &T) -> bool {
        (self.leq)(a, b)
    }
}

// [0,∞]-enriched: Metric space
struct Metric<T> {
    dist: Box<dyn Fn(&T, &T) -> f64>,
}

impl<T> Metric<T> {
    fn new<F: Fn(&T, &T) -> f64 + 'static>(dist: F) -> Self {
        Metric { dist: Box::new(dist) }
    }
    
    fn distance(&self, a: &T, b: &T) -> f64 {
        (self.dist)(a, b)
    }
}

fn main() {
    // Preorder on integers
    let pre = Preorder::new(vec![1, 2, 3], |a: &i32, b: &i32| a <= b);
    println!("1 ≤ 2: {}", pre.is_related(&1, &2));
    println!("2 ≤ 1: {}", pre.is_related(&2, &1));
    
    // Euclidean metric
    let metric = Metric::new(|a: &f64, b: &f64| (a - b).abs());
    println!("d(3, 7) = {}", metric.distance(&3.0, &7.0));
}
