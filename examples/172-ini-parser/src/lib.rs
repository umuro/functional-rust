#![allow(clippy::all)]
// Example 172: INI File Parser
// INI file parser: sections [name], key = value pairs

use std::collections::HashMap;

type ParseResult<'a, T> = Result<(T, &'a str), String>;

#[derive(Debug, Clone)]
struct IniSection {
    name: String,
    entries: Vec<(String, String)>,
}

type IniFile = Vec<IniSection>;

fn skip_line(input: &str) -> &str {
    match input.find('\n') {
        Some(i) => &input[i + 1..],
        None => "",
    }
}

fn skip_blank_and_comments(mut input: &str) -> &str {
    loop {
        input = input.trim_start_matches(|c: char| c == ' ' || c == '\t');
        if input.starts_with('\n') {
            input = &input[1..];
        } else if input.starts_with('#') || input.starts_with(';') {
            input = skip_line(input);
        } else {
            return input;
        }
    }
}

// ============================================================
// Approach 1: Parse section header [name]
// ============================================================

fn parse_section_header(input: &str) -> ParseResult<String> {
    let s = input.trim_start();
    if !s.starts_with('[') {
        return Err("Expected '['".to_string());
    }
    match s.find(']') {
        Some(i) => {
            let name = s[1..i].trim().to_string();
            let rest = skip_line(&s[i + 1..]);
            Ok((name, rest))
        }
        None => Err("Expected ']'".to_string()),
    }
}

// ============================================================
// Approach 2: Parse key = value
// ============================================================

fn parse_entry(input: &str) -> ParseResult<(String, String)> {
    let s = input.trim_start_matches(|c: char| c == ' ' || c == '\t');
    if s.is_empty()
        || s.starts_with('[')
        || s.starts_with('#')
        || s.starts_with(';')
        || s.starts_with('\n')
    {
        return Err("Not a key=value entry".to_string());
    }
    let line_end = s.find('\n').unwrap_or(s.len());
    let line = &s[..line_end];
    match line.find('=') {
        Some(eq_pos) => {
            let key = line[..eq_pos].trim().to_string();
            let mut value = line[eq_pos + 1..].trim().to_string();
            // Strip inline comments
            if let Some(hash) = value.find('#') {
                value = value[..hash].trim().to_string();
            }
            if let Some(semi) = value.find(';') {
                value = value[..semi].trim().to_string();
            }
            let rest = if line_end < s.len() {
                &s[line_end + 1..]
            } else {
                ""
            };
            Ok(((key, value), rest))
        }
        None => Err("Expected '='".to_string()),
    }
}

// ============================================================
// Approach 3: Full INI parser
// ============================================================

fn parse_ini(input: &str) -> ParseResult<IniFile> {
    let mut sections = Vec::new();
    let mut remaining = skip_blank_and_comments(input);

    while !remaining.is_empty() {
        let (name, rest) = parse_section_header(remaining)?;
        let mut entries = Vec::new();
        remaining = skip_blank_and_comments(rest);

        while let Ok(((key, value), rest)) = parse_entry(remaining) {
            entries.push((key, value));
            remaining = skip_blank_and_comments(rest);
        }

        sections.push(IniSection { name, entries });
    }

    Ok((sections, ""))
}

/// Convert to HashMap for easy lookup
fn ini_to_map(sections: &[IniSection]) -> HashMap<String, HashMap<String, String>> {
    sections
        .iter()
        .map(|s| {
            let entries: HashMap<String, String> = s.entries.iter().cloned().collect();
            (s.name.clone(), entries)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_header() {
        let (name, _) = parse_section_header("[database]").unwrap();
        assert_eq!(name, "database");
    }

    #[test]
    fn test_section_header_spaces() {
        let (name, _) = parse_section_header("[ my section ]").unwrap();
        assert_eq!(name, "my section");
    }

    #[test]
    fn test_entry() {
        let ((k, v), _) = parse_entry("host = localhost\n").unwrap();
        assert_eq!(k, "host");
        assert_eq!(v, "localhost");
    }

    #[test]
    fn test_entry_inline_comment() {
        let ((k, v), _) = parse_entry("name = myapp # comment\n").unwrap();
        assert_eq!(k, "name");
        assert_eq!(v, "myapp");
    }

    #[test]
    fn test_full_ini() {
        let input = "[db]\nhost = localhost\nport = 5432\n\n[app]\nname = test\n";
        let (sections, _) = parse_ini(input).unwrap();
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].name, "db");
        assert_eq!(sections[0].entries.len(), 2);
        assert_eq!(sections[1].name, "app");
    }

    #[test]
    fn test_comments_skipped() {
        let input = "# header comment\n[s]\n; another comment\nk = v\n";
        let (sections, _) = parse_ini(input).unwrap();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].entries.len(), 1);
    }

    #[test]
    fn test_ini_to_map() {
        let input = "[db]\nhost = localhost\n";
        let (sections, _) = parse_ini(input).unwrap();
        let map = ini_to_map(&sections);
        assert_eq!(map["db"]["host"], "localhost");
    }
}
