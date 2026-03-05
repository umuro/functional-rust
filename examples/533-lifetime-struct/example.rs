//! # 533. Lifetimes in Structs
//! Struct fields that are references require lifetime annotations.

/// A highlight into a larger text — borrows from the source string
#[derive(Debug)]
struct Highlight<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Highlight<'a> {
    fn new(source: &'a str, start: usize, end: usize) -> Option<Self> {
        if end <= source.len() && start <= end {
            Some(Highlight { text: &source[start..end], start, end })
        } else {
            None
        }
    }

    fn len(&self) -> usize { self.end - self.start }
    fn content(&self) -> &str { self.text }
}

/// Parser result borrowing from input string
#[derive(Debug)]
struct ParseResult<'a> {
    value: &'a str,
    remaining: &'a str,
}

fn parse_word(input: &str) -> Option<ParseResult<'_>> {
    let trimmed = input.trim_start();
    let end = trimmed.find(|c: char| c.is_whitespace()).unwrap_or(trimmed.len());
    if end == 0 { return None; }
    Some(ParseResult {
        value: &trimmed[..end],
        remaining: &trimmed[end..],
    })
}

/// Config struct borrowing from a config file string
struct Config<'a> {
    host: &'a str,
    port: u16,
    name: &'a str,
}

impl<'a> Config<'a> {
    fn from_str(s: &'a str) -> Option<Self> {
        // Simplified: "host:port:name"
        let parts: Vec<&str> = s.splitn(3, ':').collect();
        if parts.len() != 3 { return None; }
        Some(Config {
            host: parts[0],
            port: parts[1].parse().ok()?,
            name: parts[2],
        })
    }
}

fn main() {
    // Highlight into a string
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let highlight = Highlight::new(&text, 4, 9).unwrap();
    println!("Highlight: {:?}", highlight);
    println!("Content: {:?}", highlight.content());
    println!("Length: {}", highlight.len());

    // Struct can't outlive the source
    let h2;
    {
        let src = String::from("Hello, World!");
        h2 = Highlight::new(&src, 0, 5).unwrap();
        println!("\nInner scope highlight: {:?}", h2.content());
        // h2 valid here
    }
    // h2 invalid after src dropped — compiler prevents use here

    // Parser borrowing from input
    println!("\nParsing:");
    let input = "  hello world foo bar";
    let mut remaining = input;
    while let Some(result) = parse_word(remaining) {
        println!("  word: {:?}, rest: {:?}", result.value, result.remaining);
        remaining = result.remaining;
    }

    // Config borrowing from config string
    let config_str = String::from("localhost:8080:my-service");
    if let Some(cfg) = Config::from_str(&config_str) {
        println!("\nConfig: {}:{} ({})", cfg.host, cfg.port, cfg.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight() {
        let text = String::from("hello world");
        let h = Highlight::new(&text, 0, 5).unwrap();
        assert_eq!(h.content(), "hello");
        assert_eq!(h.len(), 5);
    }

    #[test]
    fn test_highlight_oob() {
        let text = String::from("hi");
        assert!(Highlight::new(&text, 0, 10).is_none());
    }

    #[test]
    fn test_parse_word() {
        let r = parse_word("  hello world").unwrap();
        assert_eq!(r.value, "hello");
        assert_eq!(r.remaining, " world");
    }

    #[test]
    fn test_config() {
        let s = String::from("127.0.0.1:9090:api");
        let c = Config::from_str(&s).unwrap();
        assert_eq!(c.host, "127.0.0.1");
        assert_eq!(c.port, 9090);
        assert_eq!(c.name, "api");
    }
}
