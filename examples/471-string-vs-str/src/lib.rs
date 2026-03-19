#![allow(clippy::result_unit_err)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
// 471. String vs &str: ownership semantics
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greet() {
        let s = String::from("test");
        let g = make_greeting(&s);
        assert_eq!(g, "Hello, test!");
    }
    #[test]
    fn test_literal() {
        assert_eq!(make_greeting("hi"), "Hello, hi!");
    }
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }
}
