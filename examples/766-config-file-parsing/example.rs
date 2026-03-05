// 766. Config File Parsing (INI/TOML-Like)
// Sections, key=value, # comments — std-only

use std::collections::HashMap;

pub type Config = HashMap<String, HashMap<String, String>>;

pub fn parse_config(text: &str) -> Config {
    let mut cfg: Config = HashMap::new();
    let mut current_section = "global".to_string();
    cfg.entry("global".to_string()).or_default();

    for raw_line in text.lines() {
        // Strip inline comment
        let line = raw_line.split_once('#').map(|(l, _)| l).unwrap_or(raw_line);
        let line = line.trim();

        if line.is_empty() { continue; }

        if line.starts_with('[') && line.ends_with(']') {
            // Section header
            current_section = line[1..line.len() - 1].trim().to_string();
            cfg.entry(current_section.clone()).or_default();
        } else if let Some((key, value)) = line.split_once('=') {
            let key   = key.trim().to_string();
            let value = value.trim().to_string();
            cfg.entry(current_section.clone())
               .or_default()
               .insert(key, value);
        }
        // else: malformed line, skip
    }
    cfg
}

pub fn get_str<'a>(cfg: &'a Config, section: &str, key: &str, default: &'a str) -> &'a str {
    cfg.get(section)
       .and_then(|s| s.get(key))
       .map(|s| s.as_str())
       .unwrap_or(default)
}

pub fn get_int(cfg: &Config, section: &str, key: &str, default: i64) -> i64 {
    get_str(cfg, section, key, "")
        .parse()
        .unwrap_or(default)
}

pub fn get_bool(cfg: &Config, section: &str, key: &str, default: bool) -> bool {
    match get_str(cfg, section, key, "") {
        "true" | "yes" | "1" | "on"  => true,
        "false"| "no"  | "0" | "off" => false,
        _ => default,
    }
}

fn main() {
    let text = r#"
# Main config
[server]
host = localhost
port = 8080
debug = true

[database]
host = db.example.com
port = 5432
name = mydb  # production DB
max_connections = 10
"#;

    let cfg = parse_config(text);

    println!("server.host  = {}", get_str(&cfg, "server",   "host", "(none)"));
    println!("server.port  = {}", get_int(&cfg, "server",   "port", 80));
    println!("server.debug = {}", get_bool(&cfg, "server",  "debug", false));
    println!("db.host      = {}", get_str(&cfg, "database", "host", "(none)"));
    println!("db.maxconn   = {}", get_int(&cfg, "database", "max_connections", 5));

    // Dump all sections
    println!("\nAll sections:");
    let mut sections: Vec<&String> = cfg.keys().collect();
    sections.sort();
    for section in sections {
        println!("  [{section}]");
        let mut keys: Vec<&String> = cfg[section].keys().collect();
        keys.sort();
        for key in keys {
            println!("    {key} = {}", cfg[section][key]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[app]
name = myapp
version = 2
enabled = true
"#;

    #[test]
    fn parse_string_value() {
        let cfg = parse_config(SAMPLE);
        assert_eq!(get_str(&cfg, "app", "name", ""), "myapp");
    }

    #[test]
    fn parse_int_value() {
        let cfg = parse_config(SAMPLE);
        assert_eq!(get_int(&cfg, "app", "version", 0), 2);
    }

    #[test]
    fn parse_bool_value() {
        let cfg = parse_config(SAMPLE);
        assert!(get_bool(&cfg, "app", "enabled", false));
    }

    #[test]
    fn missing_key_returns_default() {
        let cfg = parse_config(SAMPLE);
        assert_eq!(get_str(&cfg, "app", "missing", "default"), "default");
    }

    #[test]
    fn comment_stripped() {
        let cfg = parse_config("[s]\nkey = value # comment\n");
        assert_eq!(get_str(&cfg, "s", "key", ""), "value");
    }
}
