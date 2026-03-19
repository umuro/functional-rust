use std::collections::HashMap;

/// Nucleotide Count — counting character frequencies
///
/// Ownership: input DNA string is borrowed (&str).
/// Result HashMap is owned by the caller.

pub fn nucleotide_count(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)].into();

    for c in dna.chars() {
        match counts.get_mut(&c) {
            Some(n) => *n += 1,
            None => return Err(c),
        }
    }
    Ok(counts)
}

/// Version 2: Using fold
pub fn nucleotide_count_fold(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        |mut acc, c| {
            *acc.get_mut(&c).ok_or(c)? += 1;
            Ok(acc)
        },
    )
}

/// Version 3: Using array instead of HashMap for performance
pub fn nucleotide_count_array(dna: &str) -> Result<[usize; 4], char> {
    let mut counts = [0usize; 4];
    for c in dna.chars() {
        match c {
            'A' => counts[0] += 1,
            'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
            _ => return Err(c),
        }
    }
    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gattaca() {
        let counts = nucleotide_count("GATTACA").unwrap();
        assert_eq!(counts[&'A'], 3);
        assert_eq!(counts[&'T'], 2);
        assert_eq!(counts[&'G'], 1);
        assert_eq!(counts[&'C'], 1);
    }

    #[test]
    fn test_empty() {
        let counts = nucleotide_count("").unwrap();
        assert!(counts.values().all(|&v| v == 0));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(nucleotide_count("GATTXCA"), Err('X'));
    }

    #[test]
    fn test_fold_version() {
        let counts = nucleotide_count_fold("GATTACA").unwrap();
        assert_eq!(counts[&'A'], 3);
    }

    #[test]
    fn test_array_version() {
        let counts = nucleotide_count_array("GATTACA").unwrap();
        assert_eq!(counts, [3, 1, 1, 2]); // A, C, G, T
    }
}
