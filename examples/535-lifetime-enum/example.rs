//! # 535. Lifetimes in Enums
//! Enum variants containing references require lifetime parameters.

/// Token borrowing from the input string
#[derive(Debug, PartialEq)]
enum Token<'a> {
    Word(&'a str),
    Number(i64),
    Punctuation(char),
    End,
}

/// Parse result that borrows from input
#[derive(Debug)]
enum ParseResult<'a, T> {
    Ok(T, &'a str),   // value + remaining input
    Err(&'a str, String), // failing input + error message
}

/// JSON-like value that may borrow from source
#[derive(Debug)]
enum JsonValue<'a> {
    Null,
    Bool(bool),
    Int(i64),
    Str(&'a str), // zero-copy string slice
    Array(Vec<JsonValue<'a>>),
}

fn parse_token(input: &str) -> ParseResult<'_, Token<'_>> {
    let input = input.trim_start();
    if input.is_empty() {
        return ParseResult::Ok(Token::End, "");
    }

    let mut chars = input.char_indices();
    let (_, first) = chars.next().unwrap();

    if first.is_alphabetic() {
        let end = chars
            .find(|(_, c)| !c.is_alphanumeric() && *c != '_')
            .map(|(i, _)| i)
            .unwrap_or(input.len());
        return ParseResult::Ok(Token::Word(&input[..end]), &input[end..]);
    }

    if first.is_ascii_digit() || (first == '-' && chars.next().map(|(_, c)| c.is_ascii_digit()).unwrap_or(false)) {
        let end = input.find(|c: char| !c.is_ascii_digit() && c != '-').unwrap_or(input.len());
        if let Ok(n) = input[..end].parse::<i64>() {
            return ParseResult::Ok(Token::Number(n), &input[end..]);
        }
    }

    if first.is_ascii_punctuation() || first == ' ' {
        return ParseResult::Ok(Token::Punctuation(first), &input[1..]);
    }

    ParseResult::Err(input, format!("unexpected character: {:?}", first))
}

fn main() {
    // Tokenize a string
    let input = "hello world 42 foo!";
    let mut remaining = input;
    println!("Tokenizing: {:?}", input);

    loop {
        match parse_token(remaining) {
            ParseResult::Ok(Token::End, _) => break,
            ParseResult::Ok(token, rest) => {
                println!("  {:?}", token);
                remaining = rest;
            }
            ParseResult::Err(at, msg) => {
                println!("  Error: {} at {:?}", msg, at);
                break;
            }
        }
    }

    // JsonValue with borrowed strings
    let source = String::from(r#"hello world rust"#);
    let words: Vec<&str> = source.split_whitespace().collect();
    let json_arr: Vec<JsonValue<'_>> = words.iter().map(|w| JsonValue::Str(w)).collect();
    let json = JsonValue::Array(json_arr);
    println!("\nJsonValue: {:?}", json);

    // Pattern match on enum with lifetime
    let text = String::from("  identifier 123");
    match parse_token(&text) {
        ParseResult::Ok(Token::Word(w), rest) => println!("Word: {:?}, rest: {:?}", w, rest),
        ParseResult::Ok(Token::Number(n), _)  => println!("Number: {}", n),
        other => println!("other: {:?}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_word() {
        let input = "hello world";
        match parse_token(input) {
            ParseResult::Ok(Token::Word(w), rest) => {
                assert_eq!(w, "hello");
                assert_eq!(rest, " world");
            }
            _ => panic!("expected Word"),
        }
    }

    #[test]
    fn test_parse_number() {
        let input = "42 rest";
        match parse_token(input) {
            ParseResult::Ok(Token::Number(n), _) => assert_eq!(n, 42),
            _ => panic!("expected Number"),
        }
    }

    #[test]
    fn test_parse_end() {
        let input = "";
        match parse_token(input) {
            ParseResult::Ok(Token::End, _) => {}
            _ => panic!("expected End"),
        }
    }
}
