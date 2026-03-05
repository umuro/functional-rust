//! # 558. Input Lifetimes Guide
//! When and how to annotate lifetime parameters on function inputs.

/// Case 1: Single input ref — elision handles it
fn first_element(slice: &[i32]) -> Option<&i32> { // &slice -> &result: rule 2
    slice.first()
}

/// Case 2: Multiple input refs, output from specific one — must annotate
fn get_key<'a, 'b>(map_key: &'a str, _context: &'b str) -> &'a str {
    map_key // output from 'a, not 'b
}

/// Case 3: Closure input with lifetime — anonymous lifetime
fn apply_to_str<F: Fn(&str) -> usize>(s: &str, f: F) -> usize {
    f(s) // s's lifetime doesn't matter for the output (usize)
}

/// Case 4: Struct method — rule 3 applies
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn as_bytes(&self) -> &[u8] { &self.data } // tied to self
    fn get_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end.min(self.data.len())]
    }
}

/// Case 5: Multiple outputs, each from different input
fn split_around<'a>(haystack: &'a str, needle: char) -> (&'a str, &'a str) {
    match haystack.find(needle) {
        Some(i) => (&haystack[..i], &haystack[i+1..]),
        None    => (haystack, ""),
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("first: {:?}", first_element(&nums));

    let key = String::from("my-key");
    let ctx = String::from("context");
    let result = get_key(&key, &ctx);
    drop(ctx); // ctx dropped — result only borrows key
    println!("key: {}", result);

    let len = apply_to_str("hello world", |s| s.split_whitespace().count());
    println!("word count: {}", len);

    let buf = Buffer { data: b"hello world".to_vec() };
    println!("bytes: {:?}", buf.as_bytes());
    println!("slice [0..5]: {:?}", buf.get_slice(0, 5));

    let s = "hello:world";
    let (left, right) = split_around(s, ':');
    println!("left: {}, right: {}", left, right);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_element() {
        assert_eq!(first_element(&[3, 1, 4]), Some(&3));
        assert_eq!(first_element(&[]), None);
    }
    #[test]
    fn test_split_around() {
        let (l, r) = split_around("a=b", '=');
        assert_eq!((l, r), ("a", "b"));
    }
    #[test]
    fn test_buffer() {
        let b = Buffer { data: b"abcde".to_vec() };
        assert_eq!(b.get_slice(1, 3), b"bc");
    }
}
