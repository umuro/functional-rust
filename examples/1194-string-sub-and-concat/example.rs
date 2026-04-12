// Solution 1: Idiomatic Rust — slice syntax for substring extraction
// &str[start..end] borrows a substring with no allocation; panics on bad range
pub fn substring(s: &str, start: usize, len: usize) -> &str {
    &s[start..start + len]
}

// Solution 2: Safe variant — returns None instead of panicking on bad bounds
// OCaml raises Invalid_argument; Rust idiom is Option
pub fn substring_safe(s: &str, start: usize, len: usize) -> Option<&str> {
    s.get(start..start + len)
}

// Solution 3: Idiomatic join — str::join is the direct equivalent of String.concat
pub fn join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}

// Solution 4: Functional fold-based join — mirrors OCaml's List.fold_left pattern
pub fn join_fold(parts: &[&str], sep: &str) -> String {
    parts
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (i, part)| {
            if i > 0 {
                acc.push_str(sep);
            }
            acc.push_str(part);
            acc
        })
}

fn main() {
    let s = "Hello, World!";

    // Substring extraction — equivalent to String.sub s 0 5
    let hello = substring(s, 0, 5);
    let world = substring(s, 7, 5);
    println!("'{}' and '{}'", hello, world);

    // Safe substring — returns Option instead of panicking
    println!("safe(3, 10) = {:?}", substring_safe("Hello", 3, 10));
    println!("safe(1, 3)  = {:?}", substring_safe("Hello", 1, 3));

    // Join — equivalent to String.concat " | " ["one";"two";"three"]
    let parts = ["one", "two", "three"];
    let joined = join(&parts, " | ");
    println!("Joined: {}", joined);

    // Fold-based join produces same result
    let joined_fold = join_fold(&parts, " | ");
    println!("Fold join: {}", joined_fold);

    // Empty sep is concatenation
    println!("concat: {}", join(&["hello", "world"], ""));
}

/* Output:
   'Hello' and 'World'
   safe(3, 10) = None
   safe(1, 3)  = Some("ell")
   Joined: one | two | three
   Fold join: one | two | three
   concat: helloworld
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_from_start() {
        assert_eq!(substring("Hello, World!", 0, 5), "Hello");
    }

    #[test]
    fn test_substring_mid() {
        assert_eq!(substring("Hello, World!", 7, 5), "World");
    }

    #[test]
    fn test_substring_single_char() {
        assert_eq!(substring("Hello", 1, 1), "e");
    }

    #[test]
    fn test_substring_full() {
        let s = "Rust";
        assert_eq!(substring(s, 0, s.len()), "Rust");
    }

    #[test]
    fn test_substring_safe_valid() {
        assert_eq!(substring_safe("Hello, World!", 0, 5), Some("Hello"));
    }

    #[test]
    fn test_substring_safe_out_of_bounds() {
        assert_eq!(substring_safe("Hello", 3, 10), None);
    }

    #[test]
    fn test_substring_safe_empty() {
        assert_eq!(substring_safe("Hello", 0, 0), Some(""));
    }

    #[test]
    fn test_join_typical() {
        assert_eq!(join(&["one", "two", "three"], " | "), "one | two | three");
    }

    #[test]
    fn test_join_empty_list() {
        assert_eq!(join(&[], ", "), "");
    }

    #[test]
    fn test_join_single() {
        assert_eq!(join(&["only"], " | "), "only");
    }

    #[test]
    fn test_join_empty_sep() {
        assert_eq!(join(&["hello", "world"], ""), "helloworld");
    }

    #[test]
    fn test_join_fold_matches_join() {
        let parts = &["a", "b", "c"];
        let sep = "-";
        assert_eq!(join(parts, sep), join_fold(parts, sep));
    }

    #[test]
    fn test_join_fold_empty() {
        assert_eq!(join_fold(&[], ", "), "");
    }
}
