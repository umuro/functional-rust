// Graph basics
use std::collections::HashMap;

fn main() {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    graph.entry(1).or_default().push(2);
    graph.entry(1).or_default().push(3);
    graph.entry(2).or_default().push(4);
    
    println!("Graph: {:?}", graph);
    println!("Neighbors of 1: {:?}", graph.get(&1));
}
