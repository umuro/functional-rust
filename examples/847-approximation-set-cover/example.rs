/// Greedy Set Cover Approximation.
///
/// At each step: pick the set covering the most uncovered elements.
/// Achieves ln(|U|)+1 approximation ratio, which is essentially optimal.

use std::collections::HashSet;

/// Greedy unweighted set cover.
/// Returns indices of chosen sets.
pub fn greedy_set_cover(universe: &HashSet<usize>, sets: &[HashSet<usize>]) -> Vec<usize> {
    let mut uncovered: HashSet<usize> = universe.clone();
    let mut used = vec![false; sets.len()];
    let mut chosen = Vec::new();

    while !uncovered.is_empty() {
        // Find set with maximum coverage of uncovered elements
        let best = sets.iter()
            .enumerate()
            .filter(|(i, _)| !used[*i])
            .max_by_key(|(_, s)| s.intersection(&uncovered).count());

        match best {
            None => break, // No more sets (shouldn't happen if universe is covered)
            Some((idx, s)) => {
                chosen.push(idx);
                used[idx] = true;
                for &e in s {
                    uncovered.remove(&e);
                }
            }
        }
    }
    chosen
}

/// Greedy weighted set cover: minimise total cost.
/// Greedy: pick set with minimum cost per newly covered element.
pub fn greedy_weighted_set_cover(
    universe: &HashSet<usize>,
    sets: &[(HashSet<usize>, f64)],
) -> Vec<usize> {
    let mut uncovered: HashSet<usize> = universe.clone();
    let mut used = vec![false; sets.len()];
    let mut chosen = Vec::new();

    while !uncovered.is_empty() {
        let best = sets.iter()
            .enumerate()
            .filter(|(i, _)| !used[*i])
            .filter_map(|(i, (s, cost))| {
                let new_covered = s.intersection(&uncovered).count();
                if new_covered == 0 { return None; }
                Some((i, cost / new_covered as f64))
            })
            .min_by(|(_, r1), (_, r2)| r1.partial_cmp(r2).unwrap());

        match best {
            None => break,
            Some((idx, _)) => {
                chosen.push(idx);
                used[idx] = true;
                for &e in &sets[idx].0 {
                    uncovered.remove(&e);
                }
            }
        }
    }
    chosen
}

/// Verify that the chosen sets cover the entire universe.
fn verify_cover(universe: &HashSet<usize>, sets: &[HashSet<usize>], chosen: &[usize]) -> bool {
    let mut covered = HashSet::new();
    for &i in chosen {
        covered.extend(&sets[i]);
    }
    universe.is_subset(&covered)
}

fn main() {
    let universe: HashSet<usize> = (1..=10).collect();
    let sets: Vec<HashSet<usize>> = vec![
        [1, 2, 3, 4, 5].iter().copied().collect(),
        [4, 5, 6, 7, 8].iter().copied().collect(),
        [1, 3, 5, 7, 9].iter().copied().collect(),
        [2, 4, 6, 8, 10].iter().copied().collect(),
        [6, 7, 8, 9, 10].iter().copied().collect(),
    ];

    let chosen = greedy_set_cover(&universe, &sets);
    println!("Greedy set cover (unweighted):");
    println!("  Chosen set indices: {:?}", chosen);
    println!("  Covers all: {}", verify_cover(&universe, &sets, &chosen));
    println!("  # sets: {} (ln(10)+1 ≈ {:.1})", chosen.len(), 10f64.ln() + 1.0);

    // Weighted version
    let weighted_sets: Vec<(HashSet<usize>, f64)> = vec![
        ([1usize, 2, 3, 4, 5].iter().copied().collect(), 10.0),
        ([4usize, 5, 6, 7, 8].iter().copied().collect(), 6.0),
        ([1usize, 3, 5, 7, 9].iter().copied().collect(), 7.0),
        ([2usize, 4, 6, 8, 10].iter().copied().collect(), 8.0),
        ([6usize, 7, 8, 9, 10].iter().copied().collect(), 5.0),
    ];
    let w_chosen = greedy_weighted_set_cover(&universe, &weighted_sets);
    println!("\nWeighted set cover:");
    println!("  Chosen: {:?}", w_chosen);
    let total_cost: f64 = w_chosen.iter().map(|&i| weighted_sets[i].1).sum();
    println!("  Total cost: {total_cost}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_universe() -> (HashSet<usize>, Vec<HashSet<usize>>) {
        let u = (1..=10).collect();
        let s = vec![
            [1, 2, 3, 4, 5].iter().copied().collect(),
            [4, 5, 6, 7, 8].iter().copied().collect(),
            [1, 3, 5, 7, 9].iter().copied().collect(),
            [2, 4, 6, 8, 10].iter().copied().collect(),
            [6, 7, 8, 9, 10].iter().copied().collect(),
        ];
        (u, s)
    }

    #[test]
    fn test_covers_all() {
        let (u, sets) = make_universe();
        let chosen = greedy_set_cover(&u, &sets);
        assert!(verify_cover(&u, &sets, &chosen));
    }

    #[test]
    fn test_greedy_efficiency() {
        let (u, sets) = make_universe();
        let chosen = greedy_set_cover(&u, &sets);
        // Greedy should need at most ln(10)+1 ≈ 3.3 → at most 4 sets for this case
        assert!(chosen.len() <= 4, "too many sets: {}", chosen.len());
    }

    #[test]
    fn test_single_set_cover() {
        let u: HashSet<usize> = [1, 2, 3].iter().copied().collect();
        let sets = vec![u.clone()];
        let chosen = greedy_set_cover(&u, &sets);
        assert_eq!(chosen, vec![0]);
    }

    #[test]
    fn test_weighted_covers_all() {
        let u: HashSet<usize> = (1..=10).collect();
        let weighted: Vec<(HashSet<usize>, f64)> = vec![
            ([1, 2, 3, 4, 5].iter().copied().collect(), 10.0),
            ([6, 7, 8, 9, 10].iter().copied().collect(), 5.0),
        ];
        let chosen = greedy_weighted_set_cover(&u, &weighted);
        let covered: HashSet<usize> = chosen.iter()
            .flat_map(|&i| weighted[i].0.iter().copied())
            .collect();
        assert!(u.is_subset(&covered));
    }

    #[test]
    fn test_approximation_ratio() {
        // Verify bound: chosen.len() <= ln(|U|) * OPT + 1
        // Here OPT ≥ 1, so greedy should be within ln(10) * 1 + 1 ≈ 3.3
        // of optimal
        let (u, sets) = make_universe();
        let chosen = greedy_set_cover(&u, &sets);
        let ln_n = (u.len() as f64).ln().ceil() as usize + 1;
        assert!(chosen.len() <= ln_n + 2, "exceeded approximation bound");
    }
}
