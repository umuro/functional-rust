use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap};

// --- Solution 1: Idiomatic Rust ---
// BinaryHeap (min-heap via Reverse) + HashMap.
// OCaml's Set.Make PQ → BinaryHeap<Reverse<...>> with stale-entry filtering.

pub fn dijkstra(
    graph: &HashMap<String, Vec<(String, i32)>>,
    start: &str,
) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(i32, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue; // stale entry — OCaml's Set prevents these; we skip instead
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < dist.get(v).copied().unwrap_or(i32::MAX) {
                dist.insert(v.clone(), alt);
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }
    dist
}

// --- Solution 2: Functional / set-based ---
// BTreeSet<(i32, String)> mirrors OCaml's Set.Make ordered priority queue.
// BTreeMap mirrors OCaml's Map.Make(String).
// Inner fn go mirrors OCaml's `let rec go pq dist = ...`.

pub fn dijkstra_functional(
    graph: &BTreeMap<String, Vec<(String, i32)>>,
    start: &str,
) -> BTreeMap<String, i32> {
    fn go(
        graph: &BTreeMap<String, Vec<(String, i32)>>,
        mut pq: BTreeSet<(i32, String)>,
        dist: BTreeMap<String, i32>,
    ) -> BTreeMap<String, i32> {
        let Some((d, u)) = pq.pop_first() else {
            return dist; // OCaml: `if PQ.is_empty pq then dist`
        };

        let (dist, pq) = graph
            .get(&u)
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .iter()
            // OCaml: `List.fold_left (fun (dist, pq) (v, w) -> ...) (dist, pq) neighbors`
            .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                let alt = d + w;
                if alt < dist.get(v).copied().unwrap_or(i32::MAX) {
                    dist.insert(v.clone(), alt);
                    pq.insert((alt, v.clone()));
                }
                (dist, pq)
            });

        go(graph, pq, dist)
    }

    let dist = BTreeMap::from([(start.to_string(), 0)]);
    let pq = BTreeSet::from([(0i32, start.to_string())]);
    go(graph, pq, dist)
}

fn main() {
    // Graph from the OCaml example
    let pairs = [
        ("a", vec![("b", 1), ("c", 4)]),
        ("b", vec![("c", 2), ("d", 6)]),
        ("c", vec![("d", 3)]),
        ("d", vec![]),
    ];
    let graph: HashMap<String, Vec<(String, i32)>> = pairs
        .into_iter()
        .map(|(k, vs)| {
            (
                k.to_string(),
                vs.into_iter().map(|(v, w)| (v.to_string(), w)).collect(),
            )
        })
        .collect();

    println!("=== Idiomatic (BinaryHeap + HashMap) ===");
    let dist = dijkstra(&graph, "a");
    let mut keys: Vec<_> = dist.keys().collect();
    keys.sort();
    for k in &keys {
        println!("{k}: {}", dist[*k]);
    }

    let btree_graph: BTreeMap<String, Vec<(String, i32)>> = graph
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    println!("\n=== Functional (BTreeSet + BTreeMap) ===");
    let dist2 = dijkstra_functional(&btree_graph, "a");
    for (k, v) in &dist2 {
        println!("{k}: {v}");
    }
}

/* Output:
   === Idiomatic (BinaryHeap + HashMap) ===
   a: 0
   b: 1
   c: 3
   d: 6

   === Functional (BTreeSet + BTreeMap) ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
