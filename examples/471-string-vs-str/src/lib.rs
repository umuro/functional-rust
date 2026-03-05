// 471. String vs &str: ownership semantics
fn greet(name: &str) { println!("Hello, {}!", name); }
fn make_greeting(name: &str) -> String { format!("Hello, {}!", name) }
fn first_word(s: &str) -> &str { &s[..s.find(' ').unwrap_or(s.len())] }


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_greet()      { let s=String::from("test"); let g=make_greeting(&s); assert_eq!(g,"Hello, test!"); }
    #[test] fn test_literal()    { assert_eq!(make_greeting("hi"),"Hello, hi!"); }
    #[test] fn test_first_word() { assert_eq!(first_word("hello world"),"hello"); assert_eq!(first_word("single"),"single"); }
}
