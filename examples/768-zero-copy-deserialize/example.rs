// 768. Zero-Copy Deserialisation with Lifetime Tricks
// Borrows &'de str from input — zero heap allocation

// ── Zero-copy record: fields borrow from input ────────────────────────────────

/// 'de = "deserialize" lifetime — the input buffer must outlive this struct
#[derive(Debug)]
pub struct PersonView<'de> {
    pub name: &'de str,
    pub age_raw: &'de str,   // raw string, parse lazily
    pub city: Option<&'de str>,
}

impl<'de> PersonView<'de> {
    pub fn age(&self) -> Option<u32> {
        self.age_raw.parse().ok()
    }
}

// ── Simple zero-copy parser ────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ParseError(String);

/// Parse "name=Alice|age=30|city=Berlin" without any allocation
pub fn parse_view(input: &str) -> Result<PersonView<'_>, ParseError> {
    // Returns &str slices that borrow from `input`
    fn find_field<'a>(input: &'a str, key: &str) -> Option<&'a str> {
        let prefix = format!("{key}=");
        for part in input.split('|') {
            if let Some(v) = part.strip_prefix(prefix.as_str()) {
                return Some(v);
            }
        }
        None
    }

    let name = find_field(input, "name")
        .ok_or_else(|| ParseError("missing 'name'".into()))?;
    let age_raw = find_field(input, "age")
        .ok_or_else(|| ParseError("missing 'age'".into()))?;
    let city = find_field(input, "city");

    Ok(PersonView { name, age_raw, city })
}

// ── Owned version (comparison) ─────────────────────────────────────────────────

#[derive(Debug)]
pub struct PersonOwned {
    pub name: String,
    pub age: u32,
    pub city: Option<String>,
}

impl<'de> From<PersonView<'de>> for PersonOwned {
    fn from(v: PersonView<'de>) -> Self {
        PersonOwned {
            name: v.name.to_string(),
            age: v.age().unwrap_or(0),
            city: v.city.map(|s| s.to_string()),
        }
    }
}

// ── Lifetime demo: showing 'de in action ──────────────────────────────────────

/// This function signature shows how 'de ties input lifetime to output lifetime
pub fn deserialize_person<'de>(input: &'de str) -> Result<PersonView<'de>, ParseError> {
    parse_view(input)
}

// ── Batch zero-copy parsing of many records ───────────────────────────────────

pub fn parse_many(input: &str) -> Vec<PersonView<'_>> {
    input.lines()
         .filter(|l| !l.is_empty())
         .filter_map(|line| parse_view(line).ok())
         .collect()
}

fn main() {
    let input = "name=Alice|age=30|city=Amsterdam";
    let view = parse_view(input).expect("parse failed");
    println!("Name: {}", view.name);          // &str pointing into `input`
    println!("Age : {:?}", view.age());
    println!("City: {:?}", view.city);

    // Zero-copy batch
    let records = "name=Bob|age=25\nname=Carol|age=35|city=Berlin\nname=Dave|age=40";
    let views = parse_many(records);
    println!("\nBatch ({} records):", views.len());
    for v in &views {
        println!("  {}: age={}", v.name, v.age_raw);
    }

    // Convert to owned when needed
    let owned: Vec<PersonOwned> = views.into_iter().map(PersonOwned::from).collect();
    println!("\nOwned: {:?}", owned.iter().map(|o| &o.name).collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_copy_name_borrows_input() {
        let input = String::from("name=Umur|age=33");
        let view = parse_view(&input).unwrap();
        // view.name is a slice of `input` — no allocation
        assert_eq!(view.name, "Umur");
        assert_eq!(view.age(), Some(33));
    }

    #[test]
    fn optional_city() {
        let input = "name=Eve|age=28|city=Paris";
        let view = parse_view(input).unwrap();
        assert_eq!(view.city, Some("Paris"));
    }

    #[test]
    fn missing_city_is_none() {
        let view = parse_view("name=X|age=1").unwrap();
        assert!(view.city.is_none());
    }

    #[test]
    fn missing_field_errors() {
        assert!(parse_view("age=30").is_err());
    }
}
