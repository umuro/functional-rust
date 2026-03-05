//! # 532. Multiple Lifetime Parameters
//! Independent lifetimes for inputs with different validity scopes.

/// Output tied to x only — y can have shorter lifetime
fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

/// Struct with two independent borrowed fields
struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> Pair<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self {
        Pair { first, second }
    }

    /// Returns from first — tied to 'a
    fn get_first(&self) -> &'a str { self.first }

    /// Returns from second — tied to 'b
    fn get_second(&self) -> &'b str { self.second }
}

/// Lifetime bound: 'long must outlive 'short
fn use_while_valid<'long, 'short>(long_lived: &'long str, _short_lived: &'short str) -> &'long str
where
    'long: 'short, // 'long outlives 'short
{
    long_lived
}

/// Function where output lifetime is independent of input structure
fn split_at_first<'a>(s: &'a str, delimiter: char) -> (&'a str, &'a str) {
    match s.find(delimiter) {
        Some(i) => (&s[..i], &s[i+1..]),
        None    => (s, ""),
    }
}

fn main() {
    // Independent lifetimes
    let long_string = String::from("long string that lives longer");
    let result;
    {
        let short_string = String::from("short");
        // first_of: output tied to long_string's lifetime, not short_string's
        result = first_of(&long_string, &short_string);
        println!("Inside inner scope: {}", result);
    } // short_string dropped here — but result doesn't borrow it!
    println!("Outside inner scope: {}", result); // OK!

        // Pair with independent lifetimes — use within same scope
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let pair = Pair::new(&s1, &s2);
    println!("first: {}, second: {}", pair.get_first(), pair.get_second());
    drop(pair);  // explicitly drop pair before s2 to show independence

    // Demonstrate first_of has output tied only to first argument
    let long_lived = String::from("long lived");
    let result;
    {
        let short_lived = String::from("short");
        result = first_of(&long_lived, &short_lived);
        println!("result inside: {}", result);
    } // short_lived dropped — but result only borrows long_lived!
    println!("first still valid: {}", result); // OK — tied to long_lived

    // split_at_first: both halves tied to input
    let s = String::from("hello:world");
    let (left, right) = split_at_first(&s, ':');
    println!("split: {:?} and {:?}", left, right);

    // Nested references
    let inner = String::from("inner data");
    let outer_ref: &str = &inner;
    let nested: &str = &inner; // &String -> &str via Deref
    println!("nested deref: {}", nested);
    let _ = outer_ref;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_of_independent() {
        let long = String::from("long-lived");
        let result;
        {
            let short = String::from("short");
            result = first_of(&long, &short);
        }
        assert_eq!(result, "long-lived"); // short gone, but result valid
    }

    #[test]
    fn test_pair_lifetimes() {
        let a = String::from("alpha");
        let b = String::from("beta");
        let p = Pair::new(&a, &b);
        assert_eq!(p.get_first(), "alpha");
        assert_eq!(p.get_second(), "beta");
    }

    #[test]
    fn test_split_at_first() {
        let s = String::from("key=value");
        let (k, v) = split_at_first(&s, '=');
        assert_eq!(k, "key");
        assert_eq!(v, "value");
    }

    #[test]
    fn test_split_no_delimiter() {
        let s = String::from("nodash");
        let (a, b) = split_at_first(&s, '-');
        assert_eq!(a, "nodash");
        assert_eq!(b, "");
    }
}
