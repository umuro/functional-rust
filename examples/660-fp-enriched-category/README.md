# Enriched Categories

Enriched categories replace hom-sets with hom-objects in a monoidal category V.

## Common Enrichments

- **Set**: Ordinary categories
- **Bool/2**: Preorders (a ≤ b)
- **[0,∞]**: Lawvere metric spaces
- **Vect**: Linear categories

## Applications

- **Preorders**: Order relations as Bool-categories
- **Metrics**: Distance as composition cost
- **Graphs**: Weighted paths, shortest path algorithms

## Usage

```rust
use example_660_fp_enriched_category::*;

// Preorder
let pre = Preorder::new(vec![1, 2, 3], |a, b| a <= b);
assert!(pre.leq(&1, &2));

// Cost graph with Floyd-Warshall
let mut graph = CostEnriched::new();
graph.add_vertex(0);
graph.add_vertex(1);
graph.set_cost(0, 1, 5.0);
```
