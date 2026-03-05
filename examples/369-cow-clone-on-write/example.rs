use std::borrow::Cow;

fn ensure_no_spaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_")) // allocation only when needed
    } else {
        Cow::Borrowed(s) // zero-cost borrow
    }
}

fn truncate_to_limit(s: &str, limit: usize) -> Cow<str> {
    if s.len() <= limit {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s[..limit].to_string())
    }
}

fn normalize_whitespace(input: &str) -> Cow<str> {
    if !input.contains("  ") && !input.starts_with(' ') && !input.ends_with(' ') {
        Cow::Borrowed(input)
    } else {
        let mut result = String::with_capacity(input.len());
        let mut prev_space = false;
        for c in input.chars() {
            if c == ' ' {
                if !prev_space { result.push(c); }
                prev_space = true;
            } else {
                result.push(c); prev_space = false;
            }
        }
        Cow::Owned(result.trim().to_string())
    }
}

fn main() {
    let inputs = ["hello", "hello world", "no_spaces", "with   extra  spaces"];
    for s in &inputs {
        let result = ensure_no_spaces(s);
        let borrowed = matches!(result, Cow::Borrowed(_));
        println!("{s:?} -> {result:?} ({})", if borrowed { "borrowed" } else { "owned" });
    }

    for s in &["short", "this is a longer string"] {
        let t = truncate_to_limit(s, 10);
        println!("{t:?}");
    }

    for s in &["clean", "  extra   spaces  "] {
        let n = normalize_whitespace(s);
        println!("{n:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn no_spaces_borrowed() {
        let r = ensure_no_spaces("hello");
        assert!(matches!(r, Cow::Borrowed(_)));
        assert_eq!(r, "hello");
    }
    #[test] fn has_spaces_owned() {
        let r = ensure_no_spaces("hello world");
        assert!(matches!(r, Cow::Owned(_)));
        assert_eq!(r, "hello_world");
    }
    #[test] fn truncate() {
        assert_eq!(truncate_to_limit("hello", 10), "hello");
        assert_eq!(truncate_to_limit("hello world", 5), "hello");
    }
}
