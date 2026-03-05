// String.split_on_char — Tokenize a String
// Standalone copy for HTML display. Run with: rustc example.rs && ./example

/// Split a string by a delimiter character.
pub fn split_on_char(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
}

/// Split and filter out empty tokens.
pub fn split_nonempty(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).filter(|tok| !tok.is_empty()).collect()
}

/// Parse a CSV-style line into fields.
pub fn parse_csv_fields(line: &str) -> Vec<&str> {
    line.split(',').collect()
}

/// Tokenise whitespace-separated words, collapsing consecutive spaces.
pub fn tokenize_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

/// Functional/recursive version — mirrors OCaml pattern matching on lists.
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

fn main() {
    // CSV tokenisation
    let csv = "Alice,30,Engineer,Amsterdam";
    let fields = parse_csv_fields(csv);
    for (i, f) in fields.iter().enumerate() {
        println!("Field {i}: {f}");
    }

    // Whitespace with empty tokens filtered
    let words = split_nonempty("  hello   world  ", ' ');
    println!("words = {:?}", words);

    // Recursive split
    let parts = split_recursive("one:two:three", ':');
    println!("parts = {:?}", parts);

    // split_whitespace convenience
    let tokens = tokenize_words("  foo   bar  baz  ");
    println!("tokens = {:?}", tokens);
}

/* Output:
   Field 0: Alice
   Field 1: 30
   Field 2: Engineer
   Field 3: Amsterdam
   words = ["hello", "world"]
   parts = ["one", "two", "three"]
   tokens = ["foo", "bar", "baz"]
*/
