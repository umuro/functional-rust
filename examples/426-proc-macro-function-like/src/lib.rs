#![allow(clippy::all)]
//! Function-like Proc Macros
//!
//! Macros invoked like function calls.

/// Example: what sql!("SELECT...") might do
pub fn parse_sql(query: &str) -> Vec<&str> {
    query.split_whitespace().collect()
}

/// Example: what html!(<div>Hello</div>) might generate
pub struct HtmlElement {
    pub tag: String,
    pub content: String,
}

impl HtmlElement {
    pub fn new(tag: &str, content: &str) -> Self {
        HtmlElement {
            tag: tag.to_string(),
            content: content.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!("<{}>{}</{}>", self.tag, self.content, self.tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sql() {
        let parts = parse_sql("SELECT * FROM users");
        assert_eq!(parts, vec!["SELECT", "*", "FROM", "users"]);
    }

    #[test]
    fn test_html_render() {
        let el = HtmlElement::new("div", "Hello");
        assert_eq!(el.render(), "<div>Hello</div>");
    }

    #[test]
    fn test_html_span() {
        let el = HtmlElement::new("span", "World");
        assert_eq!(el.render(), "<span>World</span>");
    }

    #[test]
    fn test_sql_count() {
        let parts = parse_sql("SELECT COUNT(*) FROM items WHERE active = true");
        assert_eq!(parts.len(), 8);
    }

    #[test]
    fn test_html_empty() {
        let el = HtmlElement::new("br", "");
        assert_eq!(el.render(), "<br></br>");
    }
}
