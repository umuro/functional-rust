// 765. CSV Parsing Without External Crates
// RFC 4180-compliant: quoted fields, embedded commas, escaped quotes

// ── CSV tokenizer (state machine) ─────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum State { Normal, Quoted, QuoteInQuoted }

pub fn parse_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut buf = String::new();
    let mut state = State::Normal;

    for ch in line.chars() {
        match (&state, ch) {
            (State::Normal, '"') => {
                state = State::Quoted;
            }
            (State::Normal, ',') => {
                fields.push(buf.clone());
                buf.clear();
            }
            (State::Normal, c) => {
                buf.push(c);
            }
            (State::Quoted, '"') => {
                state = State::QuoteInQuoted;
            }
            (State::Quoted, c) => {
                buf.push(c);
            }
            (State::QuoteInQuoted, '"') => {
                // Escaped quote ("") inside quoted field
                buf.push('"');
                state = State::Quoted;
            }
            (State::QuoteInQuoted, ',') => {
                fields.push(buf.clone());
                buf.clear();
                state = State::Normal;
            }
            (State::QuoteInQuoted, c) => {
                buf.push(c);
                state = State::Normal;
            }
        }
    }
    fields.push(buf);
    fields
}

/// Parse entire CSV text into rows of fields (skips empty lines)
pub fn parse_csv(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .map(parse_fields)
        .collect()
}

// ── Typed record ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}

impl Person {
    pub fn from_row(row: &[String]) -> Option<Self> {
        if row.len() < 3 { return None; }
        let age = row[1].trim().parse().ok()?;
        Some(Person {
            name: row[0].clone(),
            age,
            city: row[2].clone(),
        })
    }
}

fn main() {
    let csv = "Name,Age,City\nAlice,30,Amsterdam\n\"Bob, Jr.\",25,\"New York\"\nCarol,35,Berlin\n";
    let rows = parse_csv(csv);
    println!("Rows: {} (including header)", rows.len());

    // Skip header
    for row in rows.iter().skip(1) {
        match Person::from_row(row) {
            Some(p) => println!("  {:?}", p),
            None    => println!("  bad row: {:?}", row),
        }
    }

    // Demonstrate CSV serialization
    println!("\nRe-serialized:");
    for row in &rows {
        let line: Vec<String> = row.iter()
            .map(|f| {
                if f.contains(',') || f.contains('"') || f.contains('\n') {
                    format!("\"{}\"", f.replace('"', "\"\""))
                } else {
                    f.clone()
                }
            })
            .collect();
        println!("{}", line.join(","));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fields() {
        assert_eq!(parse_fields("a,b,c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn quoted_with_comma() {
        assert_eq!(parse_fields("\"a,b\",c"), vec!["a,b", "c"]);
    }

    #[test]
    fn escaped_quote() {
        assert_eq!(parse_fields("\"a\"\"b\",c"), vec!["a\"b", "c"]);
    }

    #[test]
    fn empty_fields() {
        assert_eq!(parse_fields(",b,"), vec!["", "b", ""]);
    }

    #[test]
    fn person_from_row() {
        let row = vec!["Alice".to_string(), "30".to_string(), "Berlin".to_string()];
        let p = Person::from_row(&row).unwrap();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 30);
    }
}
