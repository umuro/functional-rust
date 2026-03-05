/// Apply a function to every character of a string, producing a new string.
/// Mirrors OCaml's `String.map : (char -> char) -> string -> string`.
pub fn string_map(s: &str, f: impl Fn(char) -> char) -> String {
    s.chars().map(f).collect()
}

/// Build a string of length `n` where position `i` is determined by `f(i)`.
/// Mirrors OCaml's `String.init : int -> (int -> char) -> string`.
pub fn string_init(n: usize, f: impl Fn(usize) -> char) -> String {
    (0..n).map(f).collect()
}

/// ROT-13 cipher — rotate alphabetic characters by 13 positions.
pub fn rot13(c: char) -> char {
    match c {
        'a'..='z' => (b'a' + (c as u8 - b'a' + 13) % 26) as char,
        'A'..='Z' => (b'A' + (c as u8 - b'A' + 13) % 26) as char,
        _ => c,
    }
}

/// ROT-13 a whole string using `string_map`.
pub fn rot13_string(s: &str) -> String {
    string_map(s, rot13)
}

/// Build the lowercase alphabet using `string_init`.
pub fn alphabet_lowercase() -> String {
    string_init(26, |i| (b'a' + i as u8) as char)
}

/// Build the uppercase alphabet using `string_init`.
pub fn alphabet_uppercase() -> String {
    string_init(26, |i| (b'A' + i as u8) as char)
}

fn main() {
    // String.map: transform each character
    let upper = string_map("hello world", |c| c.to_ascii_uppercase());
    println!("string_map to_uppercase: {upper}");

    // ROT-13 encode and decode
    let encoded = rot13_string("Hello World");
    let decoded = rot13_string(&encoded);
    println!("rot13(\"Hello World\") = {encoded}");
    println!("rot13(rot13(\"Hello World\")) = {decoded}");

    // String.init: build a string from indices
    let alpha = alphabet_lowercase();
    println!("alphabet: {alpha}");

    let digits = string_init(10, |i| (b'0' + i as u8) as char);
    println!("digits: {digits}");

    let stars = string_init(5, |_| '*');
    println!("stars: {stars}");
}

/* Output:
   string_map to_uppercase: HELLO WORLD
   rot13("Hello World") = Uryyb Jbeyq
   rot13(rot13("Hello World")) = Hello World
   alphabet: abcdefghijklmnopqrstuvwxyz
   digits: 0123456789
   stars: *****
*/
