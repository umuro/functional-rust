//! # 531. Lifetime Annotations: 'a Basics
//! Explicit lifetime parameters expressing reference validity constraints.

/// Return the longer of two string slices.
/// 'a means: output valid as long as BOTH inputs are valid.
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}

/// First word of a string: output tied to input's lifetime
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// Two different lifetimes: x and y may have different scopes
fn pick_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x // output tied to 'a only
}

/// Struct holding a reference — must annotate lifetime
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 { 3 }
    fn text(&self) -> &str { self.text } // lifetime elided
}

fn main() {
    // Basic lifetime usage
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longer(s1.as_str(), s2.as_str());
        println!("Longer: {}", result);
        // result valid here — both s1 and s2 still alive
    }
    // After s2 dropped, result can't be used if it referred to s2

    // first_word: lifetime tied to input
    let sentence = String::from("hello world foo");
    let word = first_word(&sentence);
    println!("First word: {}", word);
    // drop(sentence); // Would error — word borrows sentence

    // pick_first: different input lifetimes
    let x = String::from("hello");
    let result2;
    {
        let y = String::from("world");
        result2 = pick_first(&x, &y);
        println!("Picked: {}", result2); // OK — result2 tied to x's lifetime
    } // y dropped — but result2 only borrows x!
    println!("Still valid: {}", result2); // OK!

    // Struct with lifetime
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt {
        text: novel.split('.').next().unwrap(),
    };
    println!("Excerpt: {:?}, level: {}", excerpt.text(), excerpt.level());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer() {
        let s1 = "hello";
        let s2 = "hi";
        assert_eq!(longer(s1, s2), "hello");
        assert_eq!(longer("ab", "abc"), "abc");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("oneword"), "oneword");
    }

    #[test]
    fn test_pick_first() {
        let a = String::from("alpha");
        let b = String::from("beta");
        assert_eq!(pick_first(&a, &b), "alpha");
    }

    #[test]
    fn test_excerpt() {
        let text = String::from("First sentence. Second sentence.");
        let e = Excerpt { text: text.split('.').next().unwrap() };
        assert_eq!(e.text(), "First sentence");
    }
}
