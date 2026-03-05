//! # 543. Lifetimes in dyn Trait
//! Lifetime bounds on trait objects: 'static vs borrowed.

use std::fmt;

trait Renderer: fmt::Debug {
    fn render(&self) -> String;
}

/// Default: Box<dyn Renderer> = Box<dyn Renderer + 'static>
/// The renderer must own all its data (no borrowed refs)
fn store_renderer(r: Box<dyn Renderer>) -> Box<dyn Renderer> {
    println!("Stored: {:?}", r);
    r
}

/// Explicit 'static bound — same as default but clearer
fn store_static(r: Box<dyn Renderer + 'static>) {
    println!("Static: {}", r.render());
}

/// 'a bound: renderer can borrow from data with lifetime 'a
fn use_borrowed_renderer<'a>(r: &dyn Renderer, _data: &'a str) {
    println!("Borrowed renderer: {}", r.render());
}

/// Struct storing a boxed renderer that borrows
struct Screen<'a> {
    renderer: Box<dyn Renderer + 'a>,
}

impl<'a> Screen<'a> {
    fn new(r: impl Renderer + 'a) -> Self {
        Screen { renderer: Box::new(r) }
    }

    fn draw(&self) {
        println!("Screen: {}", self.renderer.render());
    }
}

/// Renderer that owns its data (satisfies 'static)
#[derive(Debug)]
struct OwnedRenderer {
    content: String,
}

impl Renderer for OwnedRenderer {
    fn render(&self) -> String { format!("Owned: {}", self.content) }
}

/// Renderer that borrows a string slice (not 'static)
#[derive(Debug)]
struct BorrowedRenderer<'a> {
    content: &'a str,
}

impl<'a> Renderer for BorrowedRenderer<'a> {
    fn render(&self) -> String { format!("Borrowed: {}", self.content) }
}

fn main() {
    // Box<dyn Renderer + 'static> — owned renderer works
    let owned = OwnedRenderer { content: "hello".to_string() };
    let stored = store_renderer(Box::new(owned));
    println!("stored render: {}", stored.render());

    // Borrowed renderer — needs 'a lifetime
    let text = String::from("borrowed content");
    let borrowed = BorrowedRenderer { content: &text };
    // store_renderer(Box::new(borrowed)); // ERROR: borrowed doesn't satisfy 'static

    // Use with lifetime parameter
    use_borrowed_renderer(&borrowed, &text);

    // Screen<'a> can hold borrowed renderers
    let screen = Screen::new(BorrowedRenderer { content: &text });
    screen.draw();
    drop(screen);

    // Screen<'static> for owned
    let screen2 = Screen::new(OwnedRenderer { content: "owned".to_string() });
    screen2.draw();

    // Vec of trait objects — must all satisfy same lifetime
    let data = String::from("shared data");
    let renderers: Vec<Box<dyn Renderer + '_>> = vec![
        Box::new(BorrowedRenderer { content: &data }),
        Box::new(BorrowedRenderer { content: "static literal" }),
    ];
    for r in &renderers {
        println!("{}", r.render());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_renderer() {
        let r = OwnedRenderer { content: "test".to_string() };
        assert_eq!(r.render(), "Owned: test");
    }

    #[test]
    fn test_borrowed_renderer() {
        let s = String::from("hello");
        let r = BorrowedRenderer { content: &s };
        assert_eq!(r.render(), "Borrowed: hello");
    }

    #[test]
    fn test_screen() {
        let s = String::from("content");
        let screen = Screen::new(BorrowedRenderer { content: &s });
        // Just verify it renders without panic
        let _ = screen.renderer.render();
    }
}
