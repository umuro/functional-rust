//! # 560. Lifetime Annotation Cheatsheet
//! Quick reference for all common lifetime annotation patterns.

// =============================================================================
// 1. FUNCTION SIGNATURES
// =============================================================================

// Elided (most common — let compiler infer)
fn first_word(s: &str) -> &str { s.split_whitespace().next().unwrap_or("") }

// Explicit (when multiple inputs, output tied to one)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// Different input lifetimes, output from first
fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str { x }

// Static lifetime
fn get_greeting() -> &'static str { "Hello, World!" }

// =============================================================================
// 2. STRUCT DEFINITIONS
// =============================================================================

// Struct borrowing a string
struct StrWrapper<'a> {
    value: &'a str,
}

// Struct with multiple lifetime params
struct PairRef<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

// Generic struct with lifetime
struct Container<'a, T> {
    items: &'a [T],
    label: &'a str,
}

// =============================================================================
// 3. IMPL BLOCKS
// =============================================================================

impl<'a> StrWrapper<'a> {
    fn new(s: &'a str) -> Self { StrWrapper { value: s } }
    fn get(&self) -> &str { self.value } // rule 3: tied to &self
    fn get_explicit(&self) -> &'a str { self.value } // explicit — tied to 'a
}

impl<'a, 'b> PairRef<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self { PairRef { first, second } }
    fn first(&self) -> &'a str { self.first }
    fn second(&self) -> &'b str { self.second }
}

// =============================================================================
// 4. TRAIT DEFINITIONS AND IMPLEMENTATIONS
// =============================================================================

trait Borrow<'a> {
    fn borrow(&'a self) -> &'a str;
}

struct Named { name: String }
impl<'a> Borrow<'a> for Named {
    fn borrow(&'a self) -> &'a str { &self.name }
}

// =============================================================================
// 5. HIGHER-RANKED TRAIT BOUNDS (for<'a>)
// =============================================================================

fn apply_to_str<F>(s: &str, f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize, // works for any lifetime
{
    f(s)
}

// =============================================================================
// 6. CLOSURES WITH LIFETIMES
// =============================================================================

fn make_prefix_fn<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a {
    move |s| format!("{}: {}", prefix, s)
}

// =============================================================================
// 7. LIFETIME BOUNDS ON TYPES
// =============================================================================

// T must not contain references shorter than 'static
fn store<T: 'static>(value: T) -> Box<dyn std::any::Any> {
    Box::new(value)
}

// T must outlive 'a
fn use_ref<'a, T: 'a>(r: &'a T) -> &'a T { r }

// =============================================================================
// 8. ANONYMOUS LIFETIMES
// =============================================================================

// '_ means "infer the lifetime"
fn get_first(v: &[i32]) -> Option<&'_ i32> { v.first() } // '_ is explicit elision

fn main() {
    // 1. Functions
    println!("first_word: {}", first_word("hello world"));
    println!("longest: {}", longest("hi", "hello"));
    println!("greeting: {}", get_greeting());

    // 2. Structs
    let s = String::from("wrapper content");
    let w = StrWrapper::new(&s);
    println!("wrapper.get(): {}", w.get());

    let a = String::from("alpha");
    let b = String::from("beta");
    let pair = PairRef::new(&a, &b);
    println!("pair: {} / {}", pair.first(), pair.second());

    // 3. Container
    let items = vec![1, 2, 3, 4, 5];
    let c = Container { items: &items, label: "numbers" };
    println!("container '{}': {:?}", c.label, c.items);

    // 4. Trait
    let named = Named { name: "Alice".to_string() };
    println!("named: {}", named.borrow());

    // 5. HRTB
    let len = apply_to_str("hello world", |s| s.len());
    println!("len via HRTB: {}", len);

    // 6. Closure with lifetime
    let prefix = String::from("INFO");
    let log = make_prefix_fn(&prefix);
    println!("{}", log("server started"));

    // 7. Lifetime bounds
    let boxed = store(42i32);
    println!("stored: {:?}", boxed.downcast_ref::<i32>());

    // 8. Anonymous
    let v = vec![10, 20, 30];
    println!("first: {:?}", get_first(&v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() { assert_eq!(first_word("hello world"), "hello"); }

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "ab"), "abc");
        assert_eq!(longest("x", "xyz"), "xyz");
    }

    #[test]
    fn test_str_wrapper() {
        let s = String::from("test");
        let w = StrWrapper::new(&s);
        assert_eq!(w.get(), "test");
        assert_eq!(w.get_explicit(), "test");
    }

    #[test]
    fn test_pair_ref() {
        let a = String::from("a");
        let b = String::from("b");
        let p = PairRef::new(&a, &b);
        assert_eq!(p.first(), "a");
        assert_eq!(p.second(), "b");
    }

    #[test]
    fn test_hrtb() {
        let count = apply_to_str("a b c d", |s| s.split_whitespace().count());
        assert_eq!(count, 4);
    }

    #[test]
    fn test_closure_lifetime() {
        let p = String::from("LOG");
        let f = make_prefix_fn(&p);
        assert_eq!(f("msg"), "LOG: msg");
    }
}
