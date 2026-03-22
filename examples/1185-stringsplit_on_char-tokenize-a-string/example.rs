#![allow(dead_code)]

/// Split a string on a delimiter, preserving empty tokens.
pub fn split_on_char(s: &str, delim: char) -> Vec<&str> {
    s.split(delim).collect()
}

/// Split and filter out empty tokens.
pub fn split_nonempty(s: &str, delim: char) -> Vec<&str> {
    s.split(delim).filter(|t| !t.is_empty()).collect()
}

/// Split on whitespace, dropping empty tokens.
pub fn tokenize(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

/// Parse a CSV record: split on commas and trim each field.
pub fn parse_csv(line: &str) -> Vec<&str> {
    line.split(',').map(str::trim).collect()
}

/// Split on first occurrence of delimiter.
pub fn split_first_occurrence(s: &str, delim: char) -> Option<(&str, &str)> {
    s.split_once(delim)
}

fn main() {
    let csv = "Alice,30,Engineer,Amsterdam";
    let fields = split_on_char(csv, ',');
    println!("CSV fields: {:?}", fields);
    for (i, f) in fields.iter().enumerate() {
        println!("  Field {}: {}", i, f);
    }

    let spaced = "  hello   world  ";
    println!("tokenize({:?}) = {:?}", spaced, tokenize(spaced));

    let padded_csv = " Alice , 30 , Engineer ";
    println!("parse_csv: {:?}", parse_csv(padded_csv));

    println!("split_once('key=value=extra', '=') = {:?}",
        split_first_occurrence("key=value=extra", '='));
}

/* Output:
   CSV fields: ["Alice", "30", "Engineer", "Amsterdam"]
     Field 0: Alice
     Field 1: 30
     Field 2: Engineer
     Field 3: Amsterdam
   tokenize("  hello   world  ") = ["hello", "world"]
   parse_csv: ["Alice", "30", "Engineer"]
   split_once('key=value=extra', '=') = Some(("key", "value=extra"))
*/
