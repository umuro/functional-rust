// String.split_on_char — Tokenize a String
// OCaml: String.split_on_char delimiter str -> string list
// Rust:  str::split(delimiter) -> Split<char>

/// Split a string by a delimiter character — idiomatic Rust.
/// Returns owned `Vec<String>` to mirror OCaml's `string list`.
pub fn split_on_char(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
}

/// Split and filter out empty tokens — mirrors OCaml's
/// `List.filter (fun s -> s <> "") (String.split_on_char ' ' s)`.
pub fn split_nonempty(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).filter(|tok| !tok.is_empty()).collect()
}

/// Parse a CSV-style line into fields (no quote handling).
/// Demonstrates real-world use of `split_on_char`.
pub fn parse_csv_fields(line: &str) -> Vec<&str> {
    line.split(',').collect()
}

/// Tokenise whitespace-separated words, collapsing consecutive spaces.
/// Equivalent to `String.split_on_char ' '` + filter, but also trims.
pub fn tokenize_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

/// Functional/recursive version — closer to OCaml's recursive list style.
/// Splits on the first occurrence of `delimiter` and recurses.
pub fn split_recursive(s: &str, delimiter: char) -> Vec<&str> {
    match s.find(delimiter) {
        None => vec![s],
        Some(pos) => {
            let mut result = vec![&s[..pos]];
            result.extend(split_recursive(&s[pos + delimiter.len_utf8()..], delimiter));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- split_on_char ---

    #[test]
    fn test_split_csv_line() {
        let fields = split_on_char("Alice,30,Engineer,Amsterdam", ',');
        assert_eq!(fields, vec!["Alice", "30", "Engineer", "Amsterdam"]);
    }

    #[test]
    fn test_split_single_token() {
        assert_eq!(split_on_char("hello", ','), vec!["hello"]);
    }

    #[test]
    fn test_split_empty_string() {
        assert_eq!(split_on_char("", ','), vec![""]);
    }

    #[test]
    fn test_split_produces_empty_tokens() {
        // OCaml: String.split_on_char ',' "a,,b" = ["a"; ""; "b"]
        assert_eq!(split_on_char("a,,b", ','), vec!["a", "", "b"]);
    }

    #[test]
    fn test_split_delimiter_at_start_end() {
        assert_eq!(split_on_char(",a,b,", ','), vec!["", "a", "b", ""]);
    }

    // --- split_nonempty ---

    #[test]
    fn test_split_nonempty_filters_blanks() {
        let words = split_nonempty("  hello   world  ", ' ');
        assert_eq!(words, vec!["hello", "world"]);
    }

    #[test]
    fn test_split_nonempty_no_blanks() {
        assert_eq!(split_nonempty("a,b,c", ','), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_nonempty_all_delimiters() {
        let result = split_nonempty(",,,,", ',');
        assert!(result.is_empty());
    }

    // --- parse_csv_fields ---

    #[test]
    fn test_parse_csv_fields() {
        let fields = parse_csv_fields("Alice,30,Engineer,Amsterdam");
        assert_eq!(fields, vec!["Alice", "30", "Engineer", "Amsterdam"]);
    }

    #[test]
    fn test_parse_csv_single_field() {
        assert_eq!(parse_csv_fields("OnlyField"), vec!["OnlyField"]);
    }

    // --- tokenize_words ---

    #[test]
    fn test_tokenize_collapses_spaces() {
        assert_eq!(tokenize_words("  hello   world  "), vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_empty() {
        let result = tokenize_words("   ");
        assert!(result.is_empty());
    }

    // --- split_recursive ---

    #[test]
    fn test_recursive_matches_stdlib() {
        let input = "one:two:three";
        assert_eq!(split_recursive(input, ':'), split_on_char(input, ':'));
    }

    #[test]
    fn test_recursive_no_delimiter() {
        assert_eq!(split_recursive("hello", ':'), vec!["hello"]);
    }

    #[test]
    fn test_recursive_unicode_delimiter() {
        // '·' is U+00B7, two UTF-8 bytes — tests len_utf8 path
        let result = split_recursive("a·b·c", '·');
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
