// Example 112: Cow<T> — Clone on Write
//
// Cow<'a, T> = either Borrowed(&'a T) or Owned(T).
// Allocation is deferred until — and only if — mutation is needed.

use std::borrow::Cow;

// Solution 1: Idiomatic Rust — conditional string modification.
// Returns a borrowed slice when no change is needed; allocates only on mutation.
pub fn normalize_whitespace(s: &str) -> Cow<'_, str> {
    if s.contains('\t') {
        Cow::Owned(s.replace('\t', " "))
    } else {
        Cow::Borrowed(s)
    }
}

// Solution 2: Functional — strip a prefix, borrowing when it isn't present.
pub fn strip_prefix_cow<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    match s.strip_prefix(prefix) {
        Some(stripped) => Cow::Owned(stripped.to_owned()),
        None => Cow::Borrowed(s),
    }
}

// Solution 3: to_mut() demonstrates lazy cloning on a Cow<[T]>.
// The slice is shared until the data actually needs reordering.
pub fn ensure_sorted(v: &[i32]) -> Cow<'_, [i32]> {
    if v.windows(2).all(|w| w[0] <= w[1]) {
        Cow::Borrowed(v)
    } else {
        let mut owned = v.to_vec();
        owned.sort();
        Cow::Owned(owned)
    }
}

// Solution 4: Accumulate only when the input needs HTML escaping.
pub fn escape_html(s: &str) -> Cow<'_, str> {
    if !s.contains(['<', '>', '&', '"']) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    // --- normalize_whitespace ---

    #[test]
    fn test_normalize_no_tabs_borrows() {
        let s = "hello world";
        let result = normalize_whitespace(s);
        assert_eq!(&*result, "hello world");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_normalize_with_tabs_owns() {
        let s = "hello\tworld\tthere";
        let result = normalize_whitespace(s);
        assert_eq!(&*result, "hello world there");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_normalize_empty_string() {
        let result = normalize_whitespace("");
        assert_eq!(&*result, "");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    // --- strip_prefix_cow ---

    #[test]
    fn test_strip_prefix_present_owns() {
        let result = strip_prefix_cow("Mr. Smith", "Mr. ");
        assert_eq!(&*result, "Smith");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_strip_prefix_absent_borrows() {
        let result = strip_prefix_cow("Dr. Jones", "Mr. ");
        assert_eq!(&*result, "Dr. Jones");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    // --- ensure_sorted ---

    #[test]
    fn test_sorted_input_borrows() {
        let v = [1, 2, 3, 4, 5];
        let result = ensure_sorted(&v);
        assert_eq!(&*result, &[1, 2, 3, 4, 5]);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_unsorted_input_owns() {
        let v = [3, 1, 4, 1, 5, 9, 2, 6];
        let result = ensure_sorted(&v);
        assert_eq!(&*result, &[1, 1, 2, 3, 4, 5, 6, 9]);
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_empty_slice_borrows() {
        let v: &[i32] = &[];
        let result = ensure_sorted(v);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    // --- escape_html ---

    #[test]
    fn test_escape_html_no_special_borrows() {
        let s = "hello world";
        let result = escape_html(s);
        assert_eq!(&*result, "hello world");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_escape_html_with_special_owns() {
        let result = escape_html("<b>bold</b> & \"quoted\"");
        assert_eq!(&*result, "&lt;b&gt;bold&lt;/b&gt; &amp; &quot;quoted&quot;");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_escape_html_only_ampersand() {
        let result = escape_html("a & b");
        assert_eq!(&*result, "a &amp; b");
        assert!(matches!(result, Cow::Owned(_)));
    }
}
