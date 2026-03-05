// Static vs dynamic dispatch in Rust
use std::time::Instant;

trait Processor {
    fn process(&self, x: i64) -> i64;
    fn name(&self) -> &str;
}

struct Double;
struct Square;
struct AddN { n: i64 }

impl Processor for Double {
    fn process(&self, x: i64) -> i64 { x * 2 }
    fn name(&self) -> &str { "Double" }
}

impl Processor for Square {
    fn process(&self, x: i64) -> i64 { x * x }
    fn name(&self) -> &str { "Square" }
}

impl Processor for AddN {
    fn process(&self, x: i64) -> i64 { x + self.n }
    fn name(&self) -> &str { "AddN" }
}

// STATIC dispatch — monomorphized, can be inlined
fn apply_static<P: Processor>(p: &P, x: i64) -> i64 {
    p.process(x)
}

// DYNAMIC dispatch — vtable lookup
fn apply_dynamic(p: &dyn Processor, x: i64) -> i64 {
    p.process(x)
}

// Static: heterogeneous requires generic — can't mix types in one Vec
fn process_all_static(x: i64) -> i64 {
    let d = Double;
    let s = Square;
    let a = AddN { n: 10 };
    apply_static(&d, x) + apply_static(&s, x) + apply_static(&a, x)
}

// Dynamic: heterogeneous collection works
fn process_all_dynamic(processors: &[Box<dyn Processor>], x: i64) -> i64 {
    processors.iter().map(|p| apply_dynamic(p.as_ref(), x)).sum()
}

fn benchmark(n: u64) {
    let iterations = 10_000_000u64;
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(Double), Box::new(Square), Box::new(AddN { n: 5 }),
    ];

    let t0 = Instant::now();
    let mut sum = 0i64;
    for i in 0..iterations { sum = sum.wrapping_add(process_all_static(i as i64)); }
    let static_time = t0.elapsed();

    let t1 = Instant::now();
    let mut sum2 = 0i64;
    for i in 0..iterations { sum2 = sum2.wrapping_add(process_all_dynamic(&processors, i as i64)); }
    let dynamic_time = t1.elapsed();

    println!("Static:  {:?} (sum={})", static_time, sum);
    println!("Dynamic: {:?} (sum={})", dynamic_time, sum2);
}

fn main() {
    println!("=== Static Dispatch ===");
    let d = Double;
    println!("{}: {}", d.name(), apply_static(&d, 5));

    println!("\n=== Dynamic Dispatch ===");
    let procs: Vec<Box<dyn Processor>> = vec![
        Box::new(Double), Box::new(Square), Box::new(AddN { n: 7 }),
    ];
    for p in &procs {
        println!("{}: {}", p.name(), apply_dynamic(p.as_ref(), 5));
    }

    println!("\n=== Benchmark ===");
    benchmark(10_000_000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_dynamic_agree() {
        let d = Double;
        assert_eq!(apply_static(&d, 7), apply_dynamic(&d, 7));
        assert_eq!(apply_static(&d, 7), 14);
    }

    #[test]
    fn test_dynamic_collection() {
        let procs: Vec<Box<dyn Processor>> = vec![Box::new(Double), Box::new(AddN { n: 3 })];
        assert_eq!(process_all_dynamic(&procs, 5), 10 + 8); // 5*2 + 5+3
    }
}
