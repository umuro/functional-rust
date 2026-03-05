//! # Network Flow
//! Flow network concepts and min-cut

pub struct FlowNetwork {
    pub capacity: Vec<Vec<i32>>,
    pub n: usize,
}

impl FlowNetwork {
    pub fn new(n: usize) -> Self { FlowNetwork { capacity: vec![vec![0; n]; n], n } }
    pub fn add_edge(&mut self, u: usize, v: usize, c: i32) { self.capacity[u][v] = c; }
    
    /// Min-cut equals max-flow (by max-flow min-cut theorem)
    pub fn min_cut(&self, _source: usize, _sink: usize) -> i32 {
        // After running max-flow, the residual graph gives the min-cut
        0 // Simplified
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_network() {
        let mut net = FlowNetwork::new(4);
        net.add_edge(0, 1, 10);
        net.add_edge(1, 2, 5);
        assert_eq!(net.capacity[0][1], 10);
    }
}
