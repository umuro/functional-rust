#![allow(clippy::all)]
//! # Golden File Tests
//!
//! Testing by comparing output against known-good "golden" files.

use std::fs;
use std::path::Path;

/// Render a markdown document
pub fn render_markdown(input: &str) -> String {
    let mut output = String::new();
    for line in input.lines() {
        if line.starts_with("# ") {
            output.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
        } else if line.starts_with("## ") {
            output.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
        } else if line.starts_with("- ") {
            output.push_str(&format!("<li>{}</li>\n", &line[2..]));
        } else if line.is_empty() {
            output.push_str("<br/>\n");
        } else {
            output.push_str(&format!("<p>{}</p>\n", line));
        }
    }
    output
}

/// Pretty print JSON
pub fn pretty_json(input: &str) -> String {
    // Simple formatter - just add indentation
    let mut output = String::new();
    let mut indent = 0;
    let mut in_string = false;

    for ch in input.chars() {
        if ch == '"' && !in_string {
            in_string = true;
            output.push(ch);
        } else if ch == '"' && in_string {
            in_string = false;
            output.push(ch);
        } else if in_string {
            output.push(ch);
        } else {
            match ch {
                '{' | '[' => {
                    output.push(ch);
                    output.push('\n');
                    indent += 2;
                    output.push_str(&" ".repeat(indent));
                }
                '}' | ']' => {
                    output.push('\n');
                    indent = indent.saturating_sub(2);
                    output.push_str(&" ".repeat(indent));
                    output.push(ch);
                }
                ',' => {
                    output.push(ch);
                    output.push('\n');
                    output.push_str(&" ".repeat(indent));
                }
                ':' => {
                    output.push_str(": ");
                }
                ' ' | '\n' | '\t' | '\r' => {}
                _ => output.push(ch),
            }
        }
    }
    output
}

/// Assert output matches golden file
pub fn assert_golden(name: &str, actual: &str, golden_dir: &Path) {
    let golden_path = golden_dir.join(format!("{}.golden", name));

    if !golden_path.exists() {
        fs::write(&golden_path, actual).expect("failed to write golden file");
        println!("Created golden file: {:?}", golden_path);
        return;
    }

    let expected = fs::read_to_string(&golden_path).expect("failed to read golden file");

    if actual != expected {
        let diff_path = golden_dir.join(format!("{}.actual", name));
        fs::write(&diff_path, actual).expect("failed to write actual file");
        panic!(
            "Golden test '{}' failed!\nExpected:\n{}\nActual:\n{}\nActual saved to: {:?}",
            name, expected, actual, diff_path
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown_headers() {
        let input = "# Title\n## Subtitle";
        let output = render_markdown(input);
        assert!(output.contains("<h1>Title</h1>"));
        assert!(output.contains("<h2>Subtitle</h2>"));
    }

    #[test]
    fn test_render_markdown_list() {
        let input = "- Item 1\n- Item 2";
        let output = render_markdown(input);
        assert!(output.contains("<li>Item 1</li>"));
        assert!(output.contains("<li>Item 2</li>"));
    }

    #[test]
    fn test_render_markdown_paragraph() {
        let input = "Hello world";
        let output = render_markdown(input);
        assert!(output.contains("<p>Hello world</p>"));
    }

    #[test]
    fn test_pretty_json_object() {
        let input = r#"{"a":1,"b":2}"#;
        let output = pretty_json(input);
        assert!(output.contains("\"a\": 1"));
        assert!(output.contains("\"b\": 2"));
    }

    #[test]
    fn test_pretty_json_array() {
        let input = r#"[1,2,3]"#;
        let output = pretty_json(input);
        assert!(output.contains("1"));
        assert!(output.contains("2"));
        assert!(output.contains("3"));
    }

    #[test]
    fn test_pretty_json_nested() {
        let input = r#"{"outer":{"inner":42}}"#;
        let output = pretty_json(input);
        assert!(output.contains("\"inner\": 42"));
    }
}
