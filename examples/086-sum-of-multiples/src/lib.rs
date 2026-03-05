use std::collections::HashSet;

/// Solution 1: Idiomatic Rust — iterator-based with a HashSet to deduplicate multiples.
/// Mirrors the OCaml Set.Make(Int) approach: collect all multiples into a set, then sum.
pub fn sum_of_multiples(factors: &[u64], limit: u64) -> u64 {
    factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&f| (f..limit).step_by(f as usize))
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum()
}

/// Solution 2: Functional/fold-based — mirrors OCaml's List.fold_left structure.
/// Explicitly accumulates into a HashSet, then sums, just like the OCaml Set fold.
pub fn sum_of_multiples_fold(factors: &[u64], limit: u64) -> u64 {
    let set: HashSet<u64> =
        factors
            .iter()
            .filter(|&&f| f != 0)
            .fold(HashSet::new(), |mut acc, &f| {
                (f..limit).step_by(f as usize).for_each(|m| {
                    acc.insert(m);
                });
                acc
            });
    set.into_iter().sum()
}

/// Solution 3: Mathematical — uses inclusion-exclusion principle (no set needed).
/// Computes sum of multiples of k below limit via arithmetic series: k * n*(n+1)/2.
pub fn sum_of_multiples_math(factors: &[u64], limit: u64) -> u64 {
    // sum_divisible(k, limit) = sum of multiples of k in [k, limit)
    fn sum_divisible(k: u64, limit: u64) -> u64 {
        if k == 0 {
            return 0;
        }
        let n = (limit - 1) / k;
        k * n * (n + 1) / 2
    }

    // Collect unique non-zero factors
    let unique: Vec<u64> = {
        let mut seen = HashSet::new();
        factors
            .iter()
            .filter(|&&f| f != 0 && seen.insert(f))
            .copied()
            .collect()
    };

    // Inclusion-exclusion over all non-empty subsets
    let n = unique.len();
    (1u64..=(1 << n) - 1)
        .map(|mask| {
            let mut lcm = 1u64;
            let mut bits = 0u32;
            for (i, &val) in unique.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    bits += 1;
                    lcm = lcm_pair(lcm, val);
                    if lcm >= limit {
                        return 0i64; // no multiples below limit
                    }
                }
            }
            let s = sum_divisible(lcm, limit) as i64;
            if bits % 2 == 1 {
                s
            } else {
                -s
            }
        })
        .sum::<i64>() as u64
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm_pair(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- sum_of_multiples (idiomatic) ---

    #[test]
    fn test_empty_factors() {
        assert_eq!(sum_of_multiples(&[], 1000), 0);
    }

    #[test]
    fn test_single_factor() {
        // multiples of 3 below 10: 3, 6, 9 → 18
        assert_eq!(sum_of_multiples(&[3], 10), 18);
    }

    #[test]
    fn test_classic_3_5_below_1000() {
        assert_eq!(sum_of_multiples(&[3, 5], 1000), 233168);
    }

    #[test]
    fn test_multiple_factors_no_double_count() {
        // multiples of 2 or 3 below 10: {2,3,4,6,8,9} → 32
        assert_eq!(sum_of_multiples(&[2, 3], 10), 32);
    }

    #[test]
    fn test_zero_factor_is_ignored() {
        assert_eq!(sum_of_multiples(&[0, 3], 10), 18);
    }

    #[test]
    fn test_limit_of_one() {
        // no multiples of anything below 1
        assert_eq!(sum_of_multiples(&[3, 5], 1), 0);
    }

    #[test]
    fn test_large_input() {
        assert_eq!(sum_of_multiples(&[2, 3, 5, 7, 11], 10000), 39_614_537);
    }

    // --- sum_of_multiples_fold ---

    #[test]
    fn test_fold_matches_idiomatic() {
        assert_eq!(
            sum_of_multiples_fold(&[3, 5], 1000),
            sum_of_multiples(&[3, 5], 1000)
        );
    }

    #[test]
    fn test_fold_empty() {
        assert_eq!(sum_of_multiples_fold(&[], 100), 0);
    }

    // --- sum_of_multiples_math ---

    #[test]
    fn test_math_matches_idiomatic_small() {
        assert_eq!(
            sum_of_multiples_math(&[3, 5], 1000),
            sum_of_multiples(&[3, 5], 1000)
        );
    }

    #[test]
    fn test_math_matches_idiomatic_large() {
        assert_eq!(
            sum_of_multiples_math(&[2, 3, 5, 7, 11], 10000),
            sum_of_multiples(&[2, 3, 5, 7, 11], 10000)
        );
    }

    #[test]
    fn test_math_empty_factors() {
        assert_eq!(sum_of_multiples_math(&[], 1000), 0);
    }

    #[test]
    fn test_math_zero_factor_ignored() {
        assert_eq!(sum_of_multiples_math(&[0, 3], 10), 18);
    }
}
