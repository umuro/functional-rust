📖 **[View on hightechmind.io →](https://hightechmind.io/rust/172-ini-parser)**

---

# 172: INI File Parser

**Difficulty:** 3  **Level:** Advanced

Parse `.ini` config files with sections, key-value pairs, and comments — a complete real-world config format.

## The Problem This Solves

INI files are everywhere: Git's `.gitconfig`, Python's `setup.cfg`, Windows registry exports, game configs, application settings. The format seems simple — `key = value` — but has enough structure to be interesting: sections group related keys, comments can appear on any line, inline comments trail after values, and blank lines should be ignored.

Building an INI parser from scratch is a clean exercise in *line-oriented parsing* — a common real-world pattern where the top-level structure is rows, and each row has internal structure. It's simpler than CSV (no quoting), but introduces the two-level hierarchy: sections contain entries.

This is one of the "payoff" examples: you're using `tag`, `take_while`, and whitespace combinators from earlier to build something you could actually ship.

## The Intuition

Process line by line. Each line is one of: blank, comment, section header `[name]`, or key-value pair `key = value`. Group consecutive key-value pairs under the most recently seen section header.

```
[database]       ← section
host = localhost  ← key-value under [database]
port = 5432       ← key-value under [database]
# Primary DB      ← comment, skip

[cache]          ← new section
ttl = 300        ← key-value under [cache]
```

## How It Works in Rust

```rust
#[derive(Debug)]
struct IniSection {
    name: String,
    entries: Vec<(String, String)>,
}

fn parse_ini(input: &str) -> ParseResult<Vec<IniSection>> {
    let mut sections: Vec<IniSection> = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        let line_end = remaining.find('\n').unwrap_or(remaining.len());
        let line = remaining[..line_end].trim();
        remaining = if line_end < remaining.len() {
            &remaining[line_end + 1..]  // skip the '\n'
        } else {
            ""
        };

        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;  // blank or comment
        }

        if line.starts_with('[') {
            // Section header: [name]
            let name = line.strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .ok_or("invalid section header")?
                .trim()
                .to_string();
            sections.push(IniSection { name, entries: Vec::new() });
        } else if let Some(eq_pos) = line.find('=') {
            // Key-value pair
            let key = line[..eq_pos].trim().to_string();
            let value_raw = line[eq_pos + 1..].trim();

            // Strip inline comment (# or ;)
            let value = value_raw
                .find(|c| c == '#' || c == ';')
                .map(|i| value_raw[..i].trim())
                .unwrap_or(value_raw)
                .to_string();

            // Append to the current section (or a default unnamed section)
            if sections.is_empty() {
                sections.push(IniSection { name: "".to_string(), entries: Vec::new() });
            }
            sections.last_mut().unwrap().entries.push((key, value));
        }
    }

    Ok((sections, ""))
}

// Convenience: convert to HashMap for easy lookup
use std::collections::HashMap;
fn section_map(section: &IniSection) -> HashMap<&str, &str> {
    section.entries.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect()
}
```

## What This Unlocks

- **Real config parsing** — read `.gitconfig`, `setup.cfg`, application settings without a dependency.
- **Line-oriented parsing pattern** — works for log files, `.env` files, and many other formats.
- **Two-level structure** — sections containing entries is a pattern that generalizes (e.g., TOML is similar).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lookup | `List.assoc "key" entries` | `HashMap` or linear `find` on `Vec` |
| Section record | `{ name: string; entries: (string * string) list }` | `struct IniSection { name: String, entries: Vec<(String, String)> }` |
| String trimming | `String.trim` | `str::trim()` |
| Inline comment | `String.index_opt '#' s` | `str::find('#')` or `find(&#124;c&#124; c == '#' \|\| c == ';')` |
