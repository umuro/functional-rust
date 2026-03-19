//! Clone and Copy Traits
//!
//! Copy: implicit bitwise copy for small, stack-only types.
//! Clone: explicit, potentially expensive duplication.

/// A 2D vector — small, stack-only, implements Copy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    pub fn zero() -> Self {
        Vector2D { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(self, factor: f32) -> Vector2D {
        Vector2D {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

/// A point with optional label — Clone but not Copy (contains String).
#[derive(Debug, Clone, PartialEq)]
pub struct LabeledPoint {
    pub x: f64,
    pub y: f64,
    pub label: String,
}

impl LabeledPoint {
    pub fn new(x: f64, y: f64, label: &str) -> Self {
        LabeledPoint {
            x,
            y,
            label: label.to_string(),
        }
    }

    pub fn with_label(&self, new_label: &str) -> Self {
        LabeledPoint {
            label: new_label.to_string(),
            ..self.clone()
        }
    }
}

/// DNA sequence — Clone only (heap allocation).
#[derive(Debug, Clone, PartialEq)]
pub struct DNA {
    pub sequence: String,
    pub species: String,
}

impl DNA {
    pub fn new(sequence: &str, species: &str) -> Self {
        DNA {
            sequence: sequence.to_string(),
            species: species.to_string(),
        }
    }

    pub fn mutate(&mut self, position: usize, base: char) {
        if position < self.sequence.len() {
            let mut chars: Vec<char> = self.sequence.chars().collect();
            chars[position] = base;
            self.sequence = chars.into_iter().collect();
        }
    }

    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sequence.is_empty()
    }
}

/// Color type — small enough for Copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn with_alpha(self, a: u8) -> Self {
        Color { a, ..self }
    }

    pub fn blend(self, other: Color, factor: f32) -> Color {
        let f = factor.clamp(0.0, 1.0);
        let inv = 1.0 - f;
        Color {
            r: (self.r as f32 * inv + other.r as f32 * f) as u8,
            g: (self.g as f32 * inv + other.g as f32 * f) as u8,
            b: (self.b as f32 * inv + other.b as f32 * f) as u8,
            a: (self.a as f32 * inv + other.a as f32 * f) as u8,
        }
    }
}

/// Demonstrates that Copy types can be used after assignment.
pub fn copy_demonstration() -> (Vector2D, Vector2D) {
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = v1; // Copy: v1 still valid
    let v3 = v1; // Can use v1 again
    (v2, v3)
}

/// Demonstrates that Clone requires explicit .clone().
pub fn clone_demonstration() -> (DNA, DNA) {
    let dna1 = DNA::new("ATCG", "human");
    let mut dna2 = dna1.clone(); // Explicit clone
    dna2.mutate(0, 'G');
    (dna1, dna2) // Both independent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_copy() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = v1; // Copy
        let v3 = v1; // Still valid
        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }

    #[test]
    fn test_vector_operations() {
        let v = Vector2D::new(3.0, 4.0);
        assert_eq!(v.magnitude(), 5.0);

        let sum = v.add(Vector2D::new(1.0, 1.0));
        assert_eq!(sum, Vector2D::new(4.0, 5.0));
        // v still valid after add (Copy)
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_labeled_point_clone() {
        let p1 = LabeledPoint::new(1.0, 2.0, "A");
        let p2 = p1.clone();
        assert_eq!(p1, p2);
        // Both are independent
        assert_eq!(p1.label, "A");
        assert_eq!(p2.label, "A");
    }

    #[test]
    fn test_labeled_point_with_label() {
        let p1 = LabeledPoint::new(1.0, 2.0, "original");
        let p2 = p1.with_label("modified");
        assert_eq!(p1.label, "original");
        assert_eq!(p2.label, "modified");
        // Coordinates are the same
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
    }

    #[test]
    fn test_dna_clone_independence() {
        let dna1 = DNA::new("ATCGATCG", "mouse");
        let mut dna2 = dna1.clone();
        dna2.mutate(0, 'G');
        assert_eq!(dna1.sequence, "ATCGATCG"); // Unchanged
        assert_eq!(dna2.sequence, "GTCGATCG"); // Mutated
    }

    #[test]
    fn test_dna_len() {
        let dna = DNA::new("ATCG", "test");
        assert_eq!(dna.len(), 4);
        assert!(!dna.is_empty());
    }

    #[test]
    fn test_color_copy() {
        let red = Color::rgb(255, 0, 0);
        let red2 = red; // Copy
        let red3 = red; // Still valid
        assert_eq!(red, red2);
        assert_eq!(red2, red3);
    }

    #[test]
    fn test_color_with_alpha() {
        let color = Color::rgb(100, 150, 200);
        let transparent = color.with_alpha(128);
        assert_eq!(color.a, 255); // Original unchanged (Copy)
        assert_eq!(transparent.a, 128);
    }

    #[test]
    fn test_color_blend() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        let gray = black.blend(white, 0.5);
        assert_eq!(gray.r, 127);
        assert_eq!(gray.g, 127);
        assert_eq!(gray.b, 127);
    }

    #[test]
    fn test_copy_demonstration() {
        let (v1, v2) = copy_demonstration();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_clone_demonstration() {
        let (dna1, dna2) = clone_demonstration();
        assert_ne!(dna1.sequence, dna2.sequence);
        assert_eq!(dna1.sequence, "ATCG");
        assert_eq!(dna2.sequence, "GTCG");
    }
}
