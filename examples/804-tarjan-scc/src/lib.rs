//! # Tarjan's SCC Algorithm

pub fn tarjan_scc(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges { adj[u].push(v); }
    
    let mut index = 0;
    let mut indices = vec![None; n];
    let mut low = vec![0; n];
    let mut on_stack = vec![false; n];
    let mut stack = vec![];
    let mut sccs = vec![];
    
    fn dfs(v: usize, adj: &[Vec<usize>], index: &mut usize, indices: &mut [Option<usize>], 
           low: &mut [usize], on_stack: &mut [bool], stack: &mut Vec<usize>, sccs: &mut Vec<Vec<usize>>) {
        indices[v] = Some(*index);
        low[v] = *index;
        *index += 1;
        stack.push(v);
        on_stack[v] = true;
        
        for &w in &adj[v] {
            if indices[w].is_none() {
                dfs(w, adj, index, indices, low, on_stack, stack, sccs);
                low[v] = low[v].min(low[w]);
            } else if on_stack[w] {
                low[v] = low[v].min(indices[w].unwrap());
            }
        }
        
        if indices[v] == Some(low[v]) {
            let mut scc = vec![];
            while let Some(w) = stack.pop() {
                on_stack[w] = false;
                scc.push(w);
                if w == v { break; }
            }
            sccs.push(scc);
        }
    }
    
    for v in 0..n {
        if indices[v].is_none() {
            dfs(v, &adj, &mut index, &mut indices, &mut low, &mut on_stack, &mut stack, &mut sccs);
        }
    }
    sccs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_tarjan() {
        let edges = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4), (4, 5), (5, 3)];
        let sccs = tarjan_scc(6, &edges);
        assert_eq!(sccs.len(), 2);
    }
}
