/// Allergies — Bitflag Decoding
///
/// Ownership: Allergen is Copy. Score is a simple u32.
/// No heap allocation needed.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Allergen {
    Eggs, Peanuts, Shellfish, Strawberries,
    Tomatoes, Chocolate, Pollen, Cats,
}

impl Allergen {
    pub const ALL: [Allergen; 8] = [
        Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish,
        Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate,
        Allergen::Pollen, Allergen::Cats,
    ];

    pub fn score(self) -> u32 {
        match self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Allergen::Eggs => "eggs",
            Allergen::Peanuts => "peanuts",
            Allergen::Shellfish => "shellfish",
            Allergen::Strawberries => "strawberries",
            Allergen::Tomatoes => "tomatoes",
            Allergen::Chocolate => "chocolate",
            Allergen::Pollen => "pollen",
            Allergen::Cats => "cats",
        }
    }
}

pub fn is_allergic_to(allergen: Allergen, score: u32) -> bool {
    score & allergen.score() != 0
}

pub fn allergies(score: u32) -> Vec<Allergen> {
    Allergen::ALL
        .iter()
        .copied()
        .filter(|&a| is_allergic_to(a, score))
        .collect()
}

/// Version 2: Using bit position instead of match
pub fn allergies_bitpos(score: u32) -> Vec<Allergen> {
    Allergen::ALL
        .iter()
        .enumerate()
        .filter(|&(i, _)| score & (1 << i) != 0)
        .map(|(_, &a)| a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eggs_only() {
        assert_eq!(allergies(1), vec![Allergen::Eggs]);
    }

    #[test]
    fn test_peanuts_and_chocolate() {
        assert_eq!(allergies(34), vec![Allergen::Peanuts, Allergen::Chocolate]);
    }

    #[test]
    fn test_everything() {
        assert_eq!(allergies(255).len(), 8);
    }

    #[test]
    fn test_none() {
        assert_eq!(allergies(0), vec![]);
    }

    #[test]
    fn test_is_allergic() {
        assert!(is_allergic_to(Allergen::Peanuts, 34));
        assert!(!is_allergic_to(Allergen::Eggs, 34));
    }

    #[test]
    fn test_ignore_high_bits() {
        assert_eq!(allergies(257), vec![Allergen::Eggs]);
    }
}
