//! Lifetimes in dyn Trait
//!
//! Lifetime bounds on trait objects.

use std::fmt;

pub trait Renderer: fmt::Debug {
    fn render(&self) -> String;
}

/// Box<dyn Renderer> = Box<dyn Renderer + 'static>
#[derive(Debug)]
pub struct HtmlRenderer {
    template: String,
}

impl Renderer for HtmlRenderer {
    fn render(&self) -> String {
        format!("<html>{}</html>", self.template)
    }
}

/// Store 'static renderer.
pub fn store_renderer(r: Box<dyn Renderer>) -> Box<dyn Renderer> {
    r
}

/// Renderer that borrows (needs lifetime).
#[derive(Debug)]
pub struct BorrowingRenderer<'a> {
    content: &'a str,
}

impl<'a> Renderer for BorrowingRenderer<'a> {
    fn render(&self) -> String {
        self.content.to_string()
    }
}

/// Accept borrowed renderer with explicit lifetime.
pub fn use_borrowed_renderer<'a>(r: &'a dyn Renderer) -> String {
    r.render()
}

/// Vec of trait objects (must be 'static).
pub fn collect_renderers() -> Vec<Box<dyn Renderer>> {
    vec![
        Box::new(HtmlRenderer { template: "hello".into() }),
        Box::new(HtmlRenderer { template: "world".into() }),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_renderer() {
        let r = HtmlRenderer { template: "content".into() };
        assert_eq!(r.render(), "<html>content</html>");
    }

    #[test]
    fn test_store_renderer() {
        let r: Box<dyn Renderer> = Box::new(HtmlRenderer { template: "x".into() });
        let r2 = store_renderer(r);
        assert!(r2.render().contains("x"));
    }

    #[test]
    fn test_borrowing_renderer() {
        let content = String::from("borrowed");
        let r = BorrowingRenderer { content: &content };
        assert_eq!(r.render(), "borrowed");
    }

    #[test]
    fn test_use_borrowed() {
        let r = HtmlRenderer { template: "test".into() };
        let result = use_borrowed_renderer(&r);
        assert!(result.contains("test"));
    }

    #[test]
    fn test_collect_renderers() {
        let renderers = collect_renderers();
        assert_eq!(renderers.len(), 2);
    }
}
