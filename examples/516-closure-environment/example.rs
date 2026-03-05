//! # 516. Complex Closure Environments
//! Closures capturing structs, collections, and other closures.

struct Config {
    prefix: String,
    max_len: usize,
    transform: Box<dyn Fn(String) -> String>,
}

/// Closure capturing a Config struct
fn make_formatter(cfg: Config) -> impl FnMut(&str) -> String {
    move |s: &str| {
        let truncated = if s.len() > cfg.max_len {
            format!("{}...", &s[..cfg.max_len])
        } else {
            s.to_string()
        };
        (cfg.transform)(format!("{}{}", cfg.prefix, truncated))
    }
}

/// Closure capturing a Vec and an index — cyclic iterator
fn make_cycler<T: Clone>(items: Vec<T>) -> impl FnMut() -> T {
    let mut index = 0;
    move || {
        let val = items[index].clone();
        index = (index + 1) % items.len();
        val
    }
}

/// Nested closures: factory that creates specialized closures
fn make_multiplier_factory(base: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(move |factor| {
        let combined = base * factor;
        Box::new(move |x| x * combined)
    })
}

/// Closure capturing another closure as part of its environment
fn make_pipeline_step(
    name: String,
    transform: impl Fn(i32) -> i32 + 'static,
    next: Option<Box<dyn Fn(i32) -> i32>>,
) -> impl Fn(i32) -> i32 {
    move |x| {
        let after_transform = transform(x);
        println!("  [{}]: {} -> {}", name, x, after_transform);
        match &next {
            Some(f) => f(after_transform),
            None => after_transform,
        }
    }
}

fn main() {
    // Complex struct capture
    let cfg = Config {
        prefix: "[INFO] ".to_string(),
        max_len: 10,
        transform: Box::new(|s| s.to_uppercase()),
    };
    let mut fmt = make_formatter(cfg);
    println!("{}", fmt("hello world this is a long message"));
    println!("{}", fmt("hi"));

    // Vec + index capture
    println!("\nCycler:");
    let mut cycle = make_cycler(vec!["red", "green", "blue"]);
    for _ in 0..7 {
        print!("{} ", cycle());
    }
    println!();

    // Nested closure factory
    let make_times = make_multiplier_factory(5);
    let times10 = make_times(2); // base=5, factor=2 => combined=10
    let times15 = make_times(3); // base=5, factor=3 => combined=15
    println!("\ntimes10(3) = {}", times10(3));
    println!("times15(4) = {}", times15(4));

    // Pipeline of closures, each capturing the previous
    println!("\nPipeline trace:");
    let step3 = make_pipeline_step("negate".to_string(), |x| -x, None);
    let step2 = make_pipeline_step("double".to_string(), |x| x * 2, Some(Box::new(step3)));
    let step1 = make_pipeline_step("add5".to_string(),  |x| x + 5, Some(Box::new(step2)));
    let result = step1(3);
    println!("Final: {}", result); // (3+5)*2 = 16, negated = -16

    // Closure over HashMap
    let mut scores: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    scores.insert("alice", 90);
    scores.insert("bob", 85);
    let lookup = move |name: &str| scores.get(name).copied().unwrap_or(0);
    println!("\nalice: {}, carol: {}", lookup("alice"), lookup("carol"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycler() {
        let mut c = make_cycler(vec![1, 2, 3]);
        assert_eq!(c(), 1);
        assert_eq!(c(), 2);
        assert_eq!(c(), 3);
        assert_eq!(c(), 1); // wraps
    }

    #[test]
    fn test_multiplier_factory() {
        let factory = make_multiplier_factory(3);
        let times6 = factory(2);
        assert_eq!(times6(4), 24); // 4 * 6
    }

    #[test]
    fn test_formatter() {
        let cfg = Config {
            prefix: ">>".to_string(),
            max_len: 5,
            transform: Box::new(|s| s),
        };
        let mut fmt = make_formatter(cfg);
        assert_eq!(fmt("hi"), ">>hi");
        assert_eq!(fmt("hello world"), ">>hello...");
    }
}
