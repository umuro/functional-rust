//! # 559. Region Inference Basics
//! How Rust infers lifetime regions without explicit annotations.

/// Compiler infers 'a as the intersection of input lifetimes
fn infer_regions(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
    // No lifetimes in output — compiler doesn't need to track
}

/// Region inferred from usage context
fn use_inferred(data: &[i32]) -> i32 {
    // The lifetime of data is inferred from the call site
    // Compiler creates a "region" spanning data's borrow
    data.iter().sum()
}

/// Demonstrating region scopes
fn region_scopes() {
    let x = 5i32;
    let r1;
    {
        let y = 10i32;
        r1 = &x; // region of r1 starts here, tied to x's scope
        let r2 = &y; // region of r2 tied to y's scope
        println!("r2: {}", r2);
        // r2's region ends here (y about to drop)
    }
    // r1 still valid — x still alive
    println!("r1: {}", r1);

    // Compiler infers regions automatically:
    let mut v = vec![1, 2, 3];
    {
        let sum: i32 = v.iter().sum(); // region: within this block
        println!("sum: {}", sum);
        // region of sum ends here
    }
    v.push(4); // can mutate — no active borrows
    println!("after push: {:?}", v);
}

/// Region inference for struct lifetimes
struct View<'a> {
    data: &'a [i32],
}

impl<'a> View<'a> {
    fn new(data: &'a [i32]) -> Self { View { data } }
    fn sum(&self) -> i32 { self.data.iter().sum() }
    // Region of self is 'a — returned &self values have sub-lifetime
}

fn main() {
    // Inferred regions in simple cases
    let text = "hello world";
    let prefix = "hello";
    println!("starts with prefix: {}", infer_regions(text, prefix));

    let nums = vec![1, 2, 3, 4, 5];
    println!("sum: {}", use_inferred(&nums));

    // Region scopes
    println!("\nRegion scopes:");
    region_scopes();

    // View with inferred lifetime
    let data = vec![10, 20, 30, 40, 50];
    let view = View::new(&data);
    println!("view sum: {}", view.sum());

    // Multiple inferred regions
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let sum_a: i32 = a.iter().sum();
    let sum_b: i32 = b.iter().sum();
    // Both borrows ended — regions inferred correctly by compiler
    println!("sum_a: {}, sum_b: {}", sum_a, sum_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_infer_regions() {
        assert!(infer_regions("hello world", "hello"));
        assert!(!infer_regions("world", "hello"));
    }
    #[test]
    fn test_use_inferred() {
        assert_eq!(use_inferred(&[1, 2, 3]), 6);
    }
    #[test]
    fn test_view() {
        let d = vec![5, 10, 15];
        let v = View::new(&d);
        assert_eq!(v.sum(), 30);
    }
}
