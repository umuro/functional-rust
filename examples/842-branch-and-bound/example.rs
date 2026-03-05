/// Branch and Bound: TSP Optimisation.
///
/// Explores the space of tours, pruning branches where current cost
/// already exceeds the best known solution. Simple bound: current cost + min edges.

const INF: u64 = u64::MAX / 2;

struct Tsp {
    dist: Vec<Vec<u64>>,
    n: usize,
}

impl Tsp {
    fn new(dist: Vec<Vec<u64>>) -> Self {
        let n = dist.len();
        Self { dist, n }
    }

    /// Solve TSP with branch and bound.
    /// Returns (optimal_cost, optimal_tour).
    fn solve(&self) -> (u64, Vec<usize>) {
        let mut best_cost = INF;
        let mut best_tour = Vec::new();
        let mut path = vec![0usize];
        let mut visited = vec![false; self.n];
        visited[0] = true;

        self.bnb(&mut path, &mut visited, 0, &mut best_cost, &mut best_tour);
        (best_cost, best_tour)
    }

    fn bnb(
        &self,
        path: &mut Vec<usize>,
        visited: &mut Vec<bool>,
        cost: u64,
        best_cost: &mut u64,
        best_tour: &mut Vec<usize>,
    ) {
        if path.len() == self.n {
            // Complete tour: add return to start
            let return_cost = self.dist[*path.last().unwrap()][path[0]];
            if return_cost < INF {
                let total = cost + return_cost;
                if total < *best_cost {
                    *best_cost = total;
                    *best_tour = path.clone();
                }
            }
            return;
        }

        // Lower bound: current cost + cheapest unvisited edge from current node
        let cur = *path.last().unwrap();
        let min_forward = (0..self.n)
            .filter(|&v| !visited[v])
            .map(|v| self.dist[cur][v])
            .min()
            .unwrap_or(0);

        if cost + min_forward >= *best_cost {
            return; // Prune: cannot improve
        }

        // Sort candidates by distance for better pruning (greedy order)
        let mut candidates: Vec<usize> = (0..self.n).filter(|&v| !visited[v]).collect();
        candidates.sort_by_key(|&v| self.dist[cur][v]);

        for v in candidates {
            let edge = self.dist[cur][v];
            if edge >= INF { continue; }
            let new_cost = cost + edge;
            if new_cost < *best_cost {
                visited[v] = true;
                path.push(v);
                self.bnb(path, visited, new_cost, best_cost, best_tour);
                path.pop();
                visited[v] = false;
            }
        }
    }
}

fn main() {
    // 4-city example
    let dist4 = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let tsp4 = Tsp::new(dist4);
    let (cost, tour) = tsp4.solve();
    println!("4-city TSP:");
    println!("  Optimal cost: {cost}");
    let tour_str: Vec<String> = tour.iter().map(|v| v.to_string()).collect();
    println!("  Tour: {} -> 0", tour_str.join(" -> "));

    // 5-city example
    let dist5 = vec![
        vec![0, 2, 9, 10, 4],
        vec![1, 0, 6, 4, 3],
        vec![15, 7, 0, 8, 12],
        vec![6, 3, 12, 0, 5],
        vec![10, 4, 8, 5, 0],
    ];
    let tsp5 = Tsp::new(dist5);
    let (cost5, tour5) = tsp5.solve();
    println!("\n5-city TSP:");
    println!("  Optimal cost: {cost5}");
    let t5: Vec<String> = tour5.iter().map(|v| v.to_string()).collect();
    println!("  Tour: {} -> 0", t5.join(" -> "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4city() {
        let dist = vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0],
        ];
        let (cost, tour) = Tsp::new(dist.clone()).solve();
        // Verify tour visits all cities and returns to 0
        let mut visited = vec![false; 4];
        for &v in &tour { visited[v] = true; }
        assert!(visited.iter().all(|&v| v), "not all cities visited");
        // Verify cost is correct
        let mut computed = 0u64;
        for i in 0..tour.len() {
            let from = tour[i];
            let to = tour[(i + 1) % tour.len()];
            computed += dist[from][to];
        }
        assert_eq!(computed, cost);
    }

    #[test]
    fn test_symmetric_triangle_inequality() {
        // For a 3-city problem, brute force: 2 tours
        let dist = vec![
            vec![0, 1, 2],
            vec![1, 0, 3],
            vec![2, 3, 0],
        ];
        let (cost, _) = Tsp::new(dist).solve();
        // Tours: 0->1->2->0 = 1+3+2=6, 0->2->1->0 = 2+3+1=6
        assert_eq!(cost, 6);
    }

    #[test]
    fn test_2_cities() {
        let dist = vec![vec![0, 5], vec![5, 0]];
        let (cost, _) = Tsp::new(dist).solve();
        assert_eq!(cost, 10); // 0->1->0
    }
}
