// Clone vs Copy semantics in Rust

// Copy: bitwise, implicit, no destructor
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D { x: f32, y: f32 }

impl Vector2D {
    fn magnitude(&self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D { x: self.x + other.x, y: self.y + other.y }
    }
}

// Clone only: has heap allocation
#[derive(Debug, Clone)]
struct DNA {
    sequence: String,
    species: String,
}

impl DNA {
    fn new(seq: &str, species: &str) -> Self {
        DNA { sequence: seq.to_string(), species: species.to_string() }
    }

    fn mutate(&mut self, pos: usize, base: char) {
        if pos < self.sequence.len() {
            let mut chars: Vec<char> = self.sequence.chars().collect();
            chars[pos] = base;
            self.sequence = chars.into_iter().collect();
        }
    }
}

// How Drop prevents Copy
struct Resource { name: String }
impl Drop for Resource {
    fn drop(&mut self) { println!("Dropping: {}", self.name); }
}
// Cannot: impl Copy for Resource (has Drop)

fn demonstrate_copy() {
    let v1 = Vector2D { x: 3.0, y: 4.0 };
    let v2 = v1;       // Copy: v1 is still valid!
    let v3 = v1;       // Can copy again
    println!("v1: {:?}, v2: {:?}, v3: {:?}", v1, v2, v3);
    println!("Magnitude: {:.2}", v1.magnitude());
    let sum = v1.add(v2); // v1 and v2 still valid after
    println!("Sum: {:?}", sum);
}

fn demonstrate_clone() {
    let dna1 = DNA::new("ATCGATCG", "human");
    let mut dna2 = dna1.clone(); // explicit clone
    // dna1 is still valid
    dna2.mutate(2, 'G');
    println!("dna1: {}", dna1.sequence); // unchanged
    println!("dna2: {}", dna2.sequence); // mutated copy
}

fn demonstrate_move() {
    let s1 = String::from("hello"); // not Copy
    let s2 = s1;                    // MOVED: s1 is no longer valid
    // println!("{}", s1);          // Error: use of moved value
    println!("s2: {}", s2);

    let s3 = s2.clone();            // Clone: both valid
    println!("s2: {}, s3: {}", s2, s3);
}

fn main() {
    println!("=== Copy ===");
    demonstrate_copy();
    println!("\n=== Clone ===");
    demonstrate_clone();
    println!("\n=== Move ===");
    demonstrate_move();

    {
        let r = Resource { name: "file".to_string() };
        println!("Resource created: {}", r.name);
    } // drop called here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let v = Vector2D { x: 1.0, y: 0.0 };
        let v2 = v;
        assert_eq!(v, v2); // v still valid
    }

    #[test]
    fn test_clone_independence() {
        let d1 = DNA::new("ATCG", "mouse");
        let mut d2 = d1.clone();
        d2.mutate(0, 'G');
        assert_eq!(d1.sequence, "ATCG"); // unchanged
        assert_eq!(d2.sequence, "GTCG");
    }
}
