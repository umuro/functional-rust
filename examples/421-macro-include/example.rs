// include! and include_str! in Rust
// Note: we demonstrate the API; actual file embedding requires the file to exist

// Embed a text constant (simulating what include_str! would do)
// In a real project: const QUERY: &str = include_str!("queries/users.sql");
// const CONFIG_TEMPLATE: &str = include_str!("templates/config.toml.tpl");

// We'll use a macro to simulate the concept with inline strings
macro_rules! fake_include_str {
    ($content:literal) => { $content };
}

const SQL_QUERY: &str = fake_include_str!(
    "SELECT id, name, email FROM users WHERE active = true ORDER BY name"
);

const HTML_TEMPLATE: &str = fake_include_str!(
    "<!DOCTYPE html>
     <html><body>
     <h1>{{title}}</h1>
     <p>{{content}}</p>
     </body></html>"
);

const GRAPHQL_SCHEMA: &str = fake_include_str!(
    "type User {
      id: ID!
      name: String!
      email: String!
    }"
);

// Demonstrate include_bytes! concept with a byte array
// In real code: static WASM_BYTES: &[u8] = include_bytes!("module.wasm");
static EMBEDDED_DATA: &[u8] = &[0x52, 0x75, 0x73, 0x74]; // "Rust" in ASCII

fn render_template(template: &str, title: &str, content: &str) -> String {
    template
        .replace("{{title}}", title)
        .replace("{{content}}", content)
}

fn main() {
    println!("=== Embedded SQL ===");
    println!("{}", SQL_QUERY);

    println!("
=== HTML Template ===");
    let rendered = render_template(HTML_TEMPLATE, "Welcome", "Hello from Rust!");
    println!("{}", rendered);

    println!("
=== GraphQL Schema ===");
    println!("{}", GRAPHQL_SCHEMA);

    println!("
=== Binary Data ===");
    println!("Embedded bytes: {:?}", EMBEDDED_DATA);
    println!("As string: {}", std::str::from_utf8(EMBEDDED_DATA).unwrap());

    // Real use case: parse embedded JSON/TOML at startup
    let data = r#"{"version": "1.0.0", "name": "my-app"}"#;
    println!("
Embedded JSON length: {} bytes", data.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_sql() {
        assert!(SQL_QUERY.contains("SELECT"));
        assert!(SQL_QUERY.contains("FROM users"));
    }

    #[test]
    fn test_template_render() {
        let result = render_template(HTML_TEMPLATE, "Test", "Body");
        assert!(result.contains("Test"));
        assert!(result.contains("Body"));
        assert!(!result.contains("{{title}}"));
    }

    #[test]
    fn test_embedded_bytes() {
        assert_eq!(EMBEDDED_DATA, b"Rust");
    }
}
