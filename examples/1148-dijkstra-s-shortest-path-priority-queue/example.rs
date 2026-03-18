use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};

pub type Graph = HashMap<String, Vec<(String, usize)>>;

pub fn build_graph(edges: &[(&str, &[(&str, usize)])]) -> Graph {
    edges
        .iter()
        .map(|(from, neighbors)| {
            let ns = neighbors
                .iter()
                .map(|(to, w)| ((*to).to_string(), *w))
                .collect();
            ((*from).to_string(), ns)
        })
        .collect()
}

pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let empty: Vec<(String, usize)> = Vec::new();
        let neighbors = graph.get(&u).unwrap_or(&empty);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&usize::MAX) {
                dist.insert(v.clone(), alt);
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }

    dist
}

pub fn dijkstra_functional(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let mut dist: BTreeMap<String, usize> = BTreeMap::from([(start.to_string(), 0)]);
    let mut pq: BTreeSet<(usize, String)> = BTreeSet::from([(0, start.to_string())]);

    while let Some((d, u)) = pq.iter().next().cloned() {
        pq.remove(&(d, u.clone()));

        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let empty: Vec<(String, usize)> = Vec::new();
        let neighbors = graph.get(&u).unwrap_or(&empty);

        (dist, pq) = neighbors
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
    }

    dist
}

fn main() {
    let g = build_graph(&[
        ("a", &[("b", 1), ("c", 4)]),
        ("b", &[("c", 2), ("d", 6)]),
        ("c", &[("d", 3)]),
        ("d", &[]),
    ]);

    println!("=== dijkstra (BinaryHeap min-heap) ===");
    let mut dist: Vec<_> = dijkstra(&g, "a").into_iter().collect();
    dist.sort();
    for (node, d) in &dist {
        println!("{node}: {d}");
    }

    println!("\n=== dijkstra_functional (BTreeSet + fold) ===");
    for (node, d) in dijkstra_functional(&g, "a") {
        println!("{node}: {d}");
    }
}

/* Output:
   === dijkstra (BinaryHeap min-heap) ===
   a: 0
   b: 1
   c: 3
   d: 6

   === dijkstra_functional (BTreeSet + fold) ===
   a: 0
   b: 1
   c: 3
   d: 6
*/
