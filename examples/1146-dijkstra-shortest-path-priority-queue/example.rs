use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Graph = HashMap<String, Vec<(String, usize)>>;

fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        let Some(neighbors) = graph.get(&u) else {
            continue;
        };
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

fn dijkstra_functional(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        let relaxed: Vec<(String, usize)> = graph
            .get(&u)
            .into_iter()
            .flatten()
            .filter_map(|(v, w)| {
                let alt = d + w;
                (alt < *dist.get(v).unwrap_or(&usize::MAX)).then_some((v.clone(), alt))
            })
            .collect();

        dist.extend(relaxed.iter().cloned());
        heap.extend(relaxed.into_iter().map(|(v, alt)| Reverse((alt, v))));
    }

    dist
}

fn main() {
    let graph: Graph = [
        (
            "a".to_string(),
            vec![("b".to_string(), 1), ("c".to_string(), 4)],
        ),
        (
            "b".to_string(),
            vec![("c".to_string(), 2), ("d".to_string(), 6)],
        ),
        ("c".to_string(), vec![("d".to_string(), 3)]),
        ("d".to_string(), vec![]),
    ]
    .into_iter()
    .collect();

    let dist = dijkstra(&graph, "a");
    let mut nodes: Vec<_> = dist.iter().collect();
    nodes.sort_by_key(|(k, _)| k.as_str());
    for (node, cost) in &nodes {
        println!("{}: {}", node, cost);
    }

    println!();
    let dist2 = dijkstra_functional(&graph, "a");
    assert_eq!(dist, dist2);
    println!("Both implementations agree.");
}

/* Output:
   a: 0
   b: 1
   c: 3
   d: 6

   Both implementations agree.
*/
