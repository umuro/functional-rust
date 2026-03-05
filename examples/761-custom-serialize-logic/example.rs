// 761. Custom Serialization for Complex Types
// Enums with payloads, Option<T>, Vec<T> — all hand-rolled

use std::fmt::Write as FmtWrite;

// ── Error ──────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum SerError {
    Eof,
    BadTag(String),
    ParseError(String),
}

// ── Traits ─────────────────────────────────────────────────────────────────────

pub trait Serialize {
    fn serialize(&self, out: &mut String);
}

pub trait Deserialize: Sized {
    fn deserialize(s: &str) -> Result<(Self, &str), SerError>;
}

// ── Primitive impls ────────────────────────────────────────────────────────────

impl Serialize for f64 {
    fn serialize(&self, out: &mut String) { write!(out, "{}", self).unwrap(); }
}
impl Serialize for u32 {
    fn serialize(&self, out: &mut String) { write!(out, "{}", self).unwrap(); }
}
impl Serialize for String {
    fn serialize(&self, out: &mut String) {
        write!(out, "{}:{}", self.len(), self).unwrap();
    }
}

impl Deserialize for f64 {
    fn deserialize(s: &str) -> Result<(f64, &str), SerError> {
        let end = s.find('|').or_else(|| s.find('\n')).unwrap_or(s.len());
        let v = s[..end].parse::<f64>().map_err(|e| SerError::ParseError(e.to_string()))?;
        Ok((v, &s[end..]))
    }
}
impl Deserialize for String {
    fn deserialize(s: &str) -> Result<(String, &str), SerError> {
        let colon = s.find(':').ok_or(SerError::Eof)?;
        let len: usize = s[..colon].parse().map_err(|e| SerError::ParseError(format!("{e}")))?;
        let rest = &s[colon + 1..];
        if rest.len() < len { return Err(SerError::Eof); }
        Ok((rest[..len].to_string(), &rest[len..]))
    }
}

// ── Domain enum ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Point,
}

impl Serialize for Shape {
    fn serialize(&self, out: &mut String) {
        match self {
            Shape::Circle(r) => {
                out.push_str("C|");
                r.serialize(out);
            }
            Shape::Rectangle { width, height } => {
                out.push_str("R|");
                width.serialize(out);
                out.push('|');
                height.serialize(out);
            }
            Shape::Point => out.push('P'),
        }
    }
}

impl Deserialize for Shape {
    fn deserialize(s: &str) -> Result<(Shape, &str), SerError> {
        match s.chars().next().ok_or(SerError::Eof)? {
            'C' => {
                let rest = &s[2..]; // skip "C|"
                let (r, rest) = f64::deserialize(rest)?;
                Ok((Shape::Circle(r), rest))
            }
            'R' => {
                let rest = &s[2..];
                let (w, rest) = f64::deserialize(rest)?;
                let rest = rest.trim_start_matches('|');
                let (h, rest) = f64::deserialize(rest)?;
                Ok((Shape::Rectangle { width: w, height: h }, rest))
            }
            'P' => Ok((Shape::Point, &s[1..])),
            c => Err(SerError::BadTag(c.to_string())),
        }
    }
}

// ── Option<T> ─────────────────────────────────────────────────────────────────

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, out: &mut String) {
        match self {
            None => out.push('N'),
            Some(v) => { out.push('S'); v.serialize(out); }
        }
    }
}

// ── Vec<Shape> round-trip ──────────────────────────────────────────────────────

fn serialize_shapes(shapes: &[Shape]) -> String {
    let mut out = String::new();
    write!(out, "{}\n", shapes.len()).unwrap();
    for s in shapes {
        s.serialize(&mut out);
        out.push('\n');
    }
    out
}

fn deserialize_shapes(s: &str) -> Result<Vec<Shape>, SerError> {
    let mut lines = s.lines();
    let count: usize = lines.next().ok_or(SerError::Eof)?
        .parse().map_err(|e| SerError::ParseError(format!("{e}")))?;
    let mut shapes = Vec::with_capacity(count);
    for line in lines.take(count) {
        let (shape, _) = Shape::deserialize(line)?;
        shapes.push(shape);
    }
    Ok(shapes)
}

fn main() {
    let shapes = vec![
        Shape::Circle(3.14),
        Shape::Rectangle { width: 2.0, height: 5.0 },
        Shape::Point,
    ];
    let encoded = serialize_shapes(&shapes);
    println!("Encoded:\n{encoded}");
    let decoded = deserialize_shapes(&encoded).expect("decode failed");
    println!("Decoded: {decoded:?}");

    // Option demo
    let maybe: Option<u32> = Some(42);
    let mut buf = String::new();
    maybe.serialize(&mut buf);
    println!("Option<42> = {buf}");
    let none: Option<u32> = None;
    let mut buf2 = String::new();
    none.serialize(&mut buf2);
    println!("Option<None> = {buf2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_round_trip() {
        let s = Shape::Circle(2.718);
        let mut buf = String::new();
        s.serialize(&mut buf);
        let (decoded, _) = Shape::deserialize(&buf).unwrap();
        assert_eq!(decoded, Shape::Circle(2.718));
    }

    #[test]
    fn rect_round_trip() {
        let s = Shape::Rectangle { width: 3.0, height: 4.0 };
        let mut buf = String::new();
        s.serialize(&mut buf);
        let (decoded, _) = Shape::deserialize(&buf).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn point_round_trip() {
        let s = Shape::Point;
        let mut buf = String::new();
        s.serialize(&mut buf);
        let (decoded, _) = Shape::deserialize(&buf).unwrap();
        assert_eq!(decoded, Shape::Point);
    }

    #[test]
    fn vec_shapes_round_trip() {
        let shapes = vec![Shape::Circle(1.0), Shape::Point];
        let enc = serialize_shapes(&shapes);
        let dec = deserialize_shapes(&enc).unwrap();
        assert_eq!(shapes, dec);
    }
}
