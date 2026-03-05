// 760. Derive-Based Serialization: How derive(Serialize) Works
// Manually writing what #[derive(Serialize)] would generate

use std::collections::HashMap;

// ── Traits (hand-written derive targets) ─────────────────────────────────────

pub trait Serialize {
    /// Emit key=value pairs into the provided map
    fn serialize_fields(&self, out: &mut HashMap<String, String>);

    fn serialize(&self) -> String {
        let mut map = HashMap::new();
        self.serialize_fields(&mut map);
        let mut parts: Vec<String> = map.into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        parts.sort(); // deterministic output
        parts.join("|")
    }
}

pub trait Deserialize: Sized {
    fn deserialize_fields(map: &HashMap<String, String>) -> Option<Self>;

    fn deserialize(s: &str) -> Option<Self> {
        let map = s.split('|').filter_map(|f| {
            let mut it = f.splitn(2, '=');
            Some((it.next()?.to_string(), it.next()?.to_string()))
        }).collect();
        Self::deserialize_fields(&map)
    }
}

// ── Domain struct ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// This is what #[derive(Serialize)] generates (conceptually):
impl Serialize for Color {
    fn serialize_fields(&self, out: &mut HashMap<String, String>) {
        out.insert("r".to_string(), self.r.to_string());
        out.insert("g".to_string(), self.g.to_string());
        out.insert("b".to_string(), self.b.to_string());
    }
}

// This is what #[derive(Deserialize)] generates (conceptually):
impl Deserialize for Color {
    fn deserialize_fields(map: &HashMap<String, String>) -> Option<Self> {
        Some(Color {
            r: map.get("r")?.parse().ok()?,
            g: map.get("g")?.parse().ok()?,
            b: map.get("b")?.parse().ok()?,
        })
    }
}

// ── Nested example ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
    // Note: nested structs need a flattening strategy — shown here as prefix
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
}

impl Serialize for Pixel {
    fn serialize_fields(&self, out: &mut HashMap<String, String>) {
        out.insert("x".to_string(), self.x.to_string());
        out.insert("y".to_string(), self.y.to_string());
        out.insert("color_r".to_string(), self.color_r.to_string());
        out.insert("color_g".to_string(), self.color_g.to_string());
        out.insert("color_b".to_string(), self.color_b.to_string());
    }
}

fn main() {
    let red = Color { r: 255, g: 0, b: 0 };
    let s = red.serialize();
    println!("Serialized Color : {s}");

    let decoded = Color::deserialize(&s).expect("decode failed");
    println!("Deserialized     : {decoded:?}");

    let pixel = Pixel { x: 10, y: 20, color_r: 128, color_g: 64, color_b: 32 };
    println!("Pixel serialized : {}", pixel.serialize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_round_trip() {
        let c = Color { r: 10, g: 20, b: 30 };
        let s = c.serialize();
        assert_eq!(Color::deserialize(&s), Some(Color { r: 10, g: 20, b: 30 }));
    }

    #[test]
    fn missing_field_returns_none() {
        assert_eq!(Color::deserialize("r=255|g=0"), None);
    }

    #[test]
    fn serialize_deterministic() {
        let c1 = Color { r: 1, g: 2, b: 3 };
        let c2 = Color { r: 1, g: 2, b: 3 };
        assert_eq!(c1.serialize(), c2.serialize());
    }
}
