//! # Minimum Spanning Tree
//! Overview of MST algorithms

pub struct Edge { pub u: usize, pub v: usize, pub w: i32 }

/// Simple MST using sorted edges (Kruskal-like)
pub fn mst(edges: &mut [Edge], n: usize) -> Vec<&Edge> {
    edges.sort_by_key(|e| e.w);
    let mut parent: Vec<usize> = (0..n).collect();
    let mut result = Vec::new();
    
    fn find(parent: &mut [usize], i: usize) -> usize {
        if parent[i] != i { parent[i] = find(parent, parent[i]); }
        parent[i]
    }
    
    for edge in edges.iter() {
        let pu = find(&mut parent, edge.u);
        let pv = find(&mut parent, edge.v);
        if pu != pv {
            parent[pu] = pv;
            result.push(edge);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mst() {
        let mut edges = vec![
            Edge { u: 0, v: 1, w: 4 },
            Edge { u: 0, v: 2, w: 3 },
            Edge { u: 1, v: 2, w: 1 },
            Edge { u: 1, v: 3, w: 2 },
            Edge { u: 2, v: 3, w: 4 },
        ];
        let mst_edges = mst(&mut edges, 4);
        let total: i32 = mst_edges.iter().map(|e| e.w).sum();
        assert_eq!(total, 6);
    }
}
