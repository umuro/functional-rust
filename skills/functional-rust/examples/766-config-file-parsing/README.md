# 766: Config File Parsing (INI/TOML-Like)

**Difficulty:** 3  **Level:** Intermediate

Parse a human-editable config file with sections, key=value pairs, and inline comments — returning a two-level `HashMap` with typed accessors.

## The Problem This Solves

Almost every application needs configuration: database host, port, feature flags, timeout values. Hard-coding these means recompiling for every environment. Environment variables work but don't support structure. Full JSON or TOML parsers add dependencies and require exact syntax from non-technical operators.

INI-style files hit a practical sweet spot: operators understand them without documentation, they support comments, they're human-editable in any text editor, and the parser is short enough to audit. When you don't want to add `toml` or `config` to your dependency tree — embedded systems, CLI tools, proprietary software with restricted dependencies — you write this yourself.

The parsing challenge is modest but realistic: strip comments (text after `#`), detect section headers (`[name]`), split key=value pairs, and handle missing keys with sensible defaults. This is also a good example of Rust's string-handling patterns: `split_once`, `trim`, `starts_with`, `ends_with`.

## The Intuition

Python's `configparser` is the direct equivalent — `config['server']['host']` returns the value, `config.get('server', 'host', fallback='localhost')` returns a default. Rust's version is a plain `HashMap<String, HashMap<String, String>>`, which is explicit about the two-level structure.

The typed accessors (`get_str`, `get_int`, `get_bool`) avoid pushing the parsing burden onto callers. Instead of `cfg["database"]["port"].parse::<u16>().unwrap_or(5432)` at every call site, callers write `get_int(&cfg, "database", "port", 5432)`. The default is explicit; the error is silent and documented.

## How It Works in Rust

```rust
pub type Config = HashMap<String, HashMap<String, String>>;

pub fn parse_config(text: &str) -> Config {
    let mut cfg: Config = HashMap::new();
    let mut current_section = "global".to_string();
    cfg.entry("global".to_string()).or_default();

    for raw_line in text.lines() {
        // Strip inline comment: "host = db.example.com  # production" → "host = db.example.com"
        let line = raw_line.split_once('#').map(|(l, _)| l).unwrap_or(raw_line);
        let line = line.trim();

        if line.is_empty() { continue; }

        if line.starts_with('[') && line.ends_with(']') {
            // [server] → section "server"
            current_section = line[1..line.len() - 1].trim().to_string();
            cfg.entry(current_section.clone()).or_default();
        } else if let Some((key, value)) = line.split_once('=') {
            cfg.entry(current_section.clone())
               .or_default()
               .insert(key.trim().to_string(), value.trim().to_string());
        }
        // malformed lines are silently skipped
    }
    cfg
}

// Typed accessors with defaults
pub fn get_str<'a>(cfg: &'a Config, section: &str, key: &str, default: &'a str) -> &'a str {
    cfg.get(section).and_then(|s| s.get(key)).map(|s| s.as_str()).unwrap_or(default)
}

pub fn get_int(cfg: &Config, section: &str, key: &str, default: i64) -> i64 {
    get_str(cfg, section, key, "").parse().unwrap_or(default)
}

pub fn get_bool(cfg: &Config, section: &str, key: &str, default: bool) -> bool {
    match get_str(cfg, section, key, "") {
        "true" | "yes" | "1" | "on"  => true,
        "false"| "no"  | "0" | "off" => false,
        _ => default,
    }
}

// Usage
let cfg = parse_config(include_str!("config.ini"));
let host = get_str(&cfg, "server", "host", "localhost");
let port = get_int(&cfg, "server", "port", 8080) as u16;
let debug = get_bool(&cfg, "server", "debug", false);
```

Input:
```ini
# Main config
[server]
host = localhost
port = 8080
debug = true

[database]
host = db.example.com  # production DB
port = 5432
```

Key points:
- `split_once('#')` strips inline comments in one line — clean and allocation-free
- `.or_default()` on `HashMap::entry` creates the section if it doesn't exist
- `get_bool` accepts `true/yes/1/on` and `false/no/0/off` — operators use different conventions
- `include_str!("config.ini")` embeds the config file at compile time — useful for defaults
- Malformed lines are silently skipped — robust for real-world files with blank lines and comments

## What This Unlocks

- **Zero-dependency config loading**: add configuration to a library or CLI tool without `serde`, `toml`, or `config` — just this 50-line parser
- **Operator-friendly format**: non-developers can edit `[database]\nhost = prod-db` without JSON syntax errors
- **Layered configuration**: parse multiple config files and merge the `HashMap`s — later files override earlier ones

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Data structure | `Hashtbl` or association list | `HashMap<String, HashMap<String, String>>` |
| Comment stripping | `String.split_on_char '#'` | `split_once('#')` returns `Option<(&str, &str)>` |
| Section detection | `String.get` + char comparison | `starts_with('[') && ends_with(']')` |
| Default values | `Hashtbl.find_opt` + `Option.value` | `cfg.get(s).and_then(...)` .unwrap_or(default)` |
| Typed access | Manual `int_of_string_opt` | `get_int`, `get_bool` helper functions |
| Production library | N/A | `toml` crate, `config` crate, `figment` |
