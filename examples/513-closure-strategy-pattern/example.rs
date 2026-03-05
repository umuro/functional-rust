//! # 513. Strategy Pattern via Closures
//! Interchangeable algorithms as closure parameters and struct fields.

/// Sorter with configurable comparison strategy
struct Sorter<T> {
    compare: Box<dyn Fn(&T, &T) -> std::cmp::Ordering>,
}

impl<T: Clone> Sorter<T> {
    fn new(compare: impl Fn(&T, &T) -> std::cmp::Ordering + 'static) -> Self {
        Sorter { compare: Box::new(compare) }
    }

    fn sort(&self, mut data: Vec<T>) -> Vec<T> {
        data.sort_by(|a, b| (self.compare)(a, b));
        data
    }
}

/// Pricing with configurable discount strategy
struct PriceCalculator {
    discount: Box<dyn Fn(f64) -> f64>,
}

impl PriceCalculator {
    fn new(discount: impl Fn(f64) -> f64 + 'static) -> Self {
        PriceCalculator { discount: Box::new(discount) }
    }

    fn calculate(&self, base_price: f64) -> f64 {
        (self.discount)(base_price)
    }
}

/// Validator with composable strategies
struct Validator<T> {
    rules: Vec<Box<dyn Fn(&T) -> bool>>,
}

impl<T> Validator<T> {
    fn new() -> Self { Validator { rules: Vec::new() } }

    fn add_rule(mut self, rule: impl Fn(&T) -> bool + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    fn validate(&self, value: &T) -> bool {
        self.rules.iter().all(|rule| rule(value))
    }
}

fn main() {
    // Sorting strategies
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];

    let asc_sorter = Sorter::new(|a: &i32, b: &i32| a.cmp(b));
    let desc_sorter = Sorter::new(|a: &i32, b: &i32| b.cmp(a));
    let abs_sorter = Sorter::new(|a: &i32, b: &i32| a.abs().cmp(&b.abs()));

    println!("asc:  {:?}", asc_sorter.sort(nums.clone()));
    println!("desc: {:?}", desc_sorter.sort(nums.clone()));
    println!("abs:  {:?}", abs_sorter.sort(nums));

    // Discount strategies at runtime
    let strategies: Vec<(&str, Box<dyn Fn(f64) -> f64>)> = vec![
        ("no discount",    Box::new(|p| p)),
        ("10% off",        Box::new(|p| p * 0.9)),
        ("bulk (15% off)", Box::new(|p| p * 0.85)),
        ("flat -20",       Box::new(|p| (p - 20.0).max(0.0))),
    ];

    println!("\nPrice for $100.00:");
    for (name, strategy) in &strategies {
        println!("  {}: ${:.2}", name, strategy(100.0));
    }

    // Composable validation strategies
    let validator = Validator::new()
        .add_rule(|&x: &i32| x > 0)
        .add_rule(|&x| x < 1000)
        .add_rule(|&x| x % 2 == 0);

    println!("\nValidation:");
    for n in [42, -1, 1001, 100, 7] {
        println!("  validate({}) = {}", n, validator.validate(&n));
    }

    // Strategy swap at runtime
    let use_premium = true;
    let discount: Box<dyn Fn(f64) -> f64> = if use_premium {
        Box::new(|p| p * 0.7) // 30% off
    } else {
        Box::new(|p| p * 0.95) // 5% off
    };
    println!("\nRuntime strategy: ${:.2} -> ${:.2}", 200.0, discount(200.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorter_asc() {
        let s = Sorter::new(|a: &i32, b: &i32| a.cmp(b));
        assert_eq!(s.sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }

    #[test]
    fn test_sorter_desc() {
        let s = Sorter::new(|a: &i32, b: &i32| b.cmp(a));
        assert_eq!(s.sort(vec![3, 1, 2]), vec![3, 2, 1]);
    }

    #[test]
    fn test_validator() {
        let v = Validator::new()
            .add_rule(|&x: &i32| x > 0)
            .add_rule(|&x| x < 100);
        assert!(v.validate(&50));
        assert!(!v.validate(&0));
        assert!(!v.validate(&100));
    }
}
