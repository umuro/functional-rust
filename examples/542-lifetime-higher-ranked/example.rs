//! # 542. Higher-Ranked Trait Bounds (for<'a>)
//! Universal quantification over lifetimes for maximally flexible callbacks.

/// Without HRTB: lifetime 'a is fixed at the call site
/// This works for one specific 'a
fn apply_fixed<'a, F>(f: F, s: &'a str) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(s)
}

/// With HRTB: F works for ANY lifetime
/// The closure must handle references of any lifetime
fn apply_hrtb<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s).to_string()
}

/// HRTB in a struct: store a callback that works with any lifetime
struct Processor {
    transform: Box<dyn for<'a> Fn(&'a str) -> &'a str>,
}

impl Processor {
    fn new(f: impl for<'a> Fn(&'a str) -> &'a str + 'static) -> Self {
        Processor { transform: Box::new(f) }
    }

    fn process<'a>(&self, input: &'a str) -> &'a str {
        (self.transform)(input)
    }
}

/// HRTB with Fn returning owned (no lifetime in output)
fn map_strings<F>(strings: &[String], f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> String,
{
    strings.iter().map(|s| f(s.as_str())).collect()
}

/// Most common HRTB usage: passing closures to generic iterators
fn apply_to_all<T, F>(items: &[T], f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a T) -> String,
{
    items.iter().map(|x| f(x)).collect()
}

fn main() {
    // HRTB in a free function
    let result = apply_hrtb(|s| s.trim(), "  hello world  ");
    println!("trimmed: {:?}", result);

    // The closure works for any 'a — shown by using it with different lifetimes
    let f: &(dyn for<'a> Fn(&'a str) -> &'a str) = &|s: &str| {
        if s.is_empty() { "empty" } else { s }
    };

    let s1 = String::from("hello");
    let s2 = String::from("world");
    println!("f(s1): {}", f(&s1));
    println!("f(s2): {}", f(&s2));
    println!("f(literal): {}", f("literal"));

    // HRTB in struct
    let processor = Processor::new(|s: &str| {
        // Works for any lifetime 'a
        s.trim()
    });

    let owned = String::from("  hello  ");
    println!("processed: {:?}", processor.process(&owned));
    {
        let temp = String::from("  temp  ");
        println!("temp processed: {:?}", processor.process(&temp));
    }

    // Map strings with HRTB closure
    let strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    let uppers = map_strings(&strings, |s| s.to_uppercase());
    println!("uppers: {:?}", uppers);

    // apply_to_all
    let nums = vec![1i32, 2, 3, 4, 5];
    let formatted = apply_to_all(&nums, |n| format!("num:{}", n));
    println!("formatted: {:?}", formatted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_hrtb() {
        let result = apply_hrtb(|s| s.trim(), "  test  ");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_processor_hrtb() {
        let p = Processor::new(|s: &str| if s.len() > 5 { &s[..5] } else { s });
        let s = String::from("hello world");
        assert_eq!(p.process(&s), "hello");
    }

    #[test]
    fn test_map_strings_hrtb() {
        let v = vec!["a".to_string(), "bb".to_string()];
        let lens = map_strings(&v, |s| s.len().to_string());
        assert_eq!(lens, vec!["1", "2"]);
    }
}
