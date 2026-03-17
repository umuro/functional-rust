use std::collections::{BTreeMap, BTreeSet};

type Graph = BTreeMap<String, Vec<(String, usize)>>;

/// Dijkstra using BTreeSet as priority queue — mirrors OCaml's Set.Make idiom.
fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let mut dist: BTreeMap<String, usize> = BTreeMap::from([(start.to_string(), 0)]);
    let mut pq: BTreeSet<(usize, String)> = BTreeSet::from([(0, start.to_string())]);

    while let Some((d, u)) = pq.iter().next().cloned() {
        pq.remove(&(d, u.clone()));
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&usize::MAX) {
                dist.insert(v.clone(), alt);
                pq.insert((alt, v.clone()));
            }
        }
    }
    dist
}

/// Dijkstra — recursive, mirrors OCaml's `let rec go pq dist = ...`
fn dijkstra_recursive(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let dist = BTreeMap::from([(start.to_string(), 0)]);
    let pq = BTreeSet::from([(0usize, start.to_string())]);
    go(graph, pq, dist)
}

fn go(
    graph: &Graph,
    mut pq: BTreeSet<(usize, String)>,
    dist: BTreeMap<String, usize>,
) -> BTreeMap<String, usize> {
    let Some((d, u)) = pq.iter().next().cloned() else {
        return dist;
    };
    pq.remove(&(d, u.clone()));
    let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
    let (dist, pq) = neighbors
        .iter()
        .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
            let alt = d + w;
            let current = *dist.get(v).unwrap_or(&usize::MAX);
            if alt < current {
                dist.insert(v.clone(), alt);
                pq.insert((alt, v.clone()));
            }
            (dist, pq)
        });
    go(graph, pq, dist)
}

fn main() {
    let graph: Graph = [
        ("a".to_string(), vec![("b".to_string(), 1), ("c".to_string(), 4)]),
        ("b".to_string(), vec![("c".to_string(), 2), ("d".to_string(), 6)]),
        ("c".to_string(), vec![("d".to_string(), 3)]),
        ("d".to_string(), vec![]),
    ]
    .into_iter()
    .collect();

    println!("--- iterative (BTreeSet PQ) ---");
    let dist = dijkstra(&graph, "a");
    // BTreeMap iterates in sorted order — no manual sort needed
    for (node, cost) in &dist {
        println!("{}: {}", node, cost);
    }

    println!("\n--- recursive (fold over neighbours) ---");
    let dist2 = dijkstra_recursive(&graph, "a");
    for (node, cost) in &dist2 {
        println!("{}: {}", node, cost);
    }

    assert_eq!(dist, dist2);
    println!("\nBoth implementations agree.");
}

/* Output:
   --- iterative (BTreeSet PQ) ---
   a: 0
   b: 1
   c: 3
   d: 6

   --- recursive (fold over neighbours) ---
   a: 0
   b: 1
   c: 3
   d: 6

   Both implementations agree.
*/
