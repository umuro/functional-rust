//! # Enriched Categories
//!
//! An enriched category has hom-sets that are objects in another category V.
//! - V = Set: ordinary category
//! - V = Vect: linear maps
//! - V = [0,∞]: metric spaces (Lawvere)
//! - V = Bool: preorders

use std::marker::PhantomData;
use std::collections::HashMap;
use std::hash::Hash;

/// Monoidal category for enrichment
pub trait Monoidal {
    type Object;
    fn unit() -> Self::Object;
    fn tensor(a: Self::Object, b: Self::Object) -> Self::Object;
}

// Approach 1: Bool-enriched (preorders)
/// Preorder as Bool-enriched category
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preorder<T> {
    elements: Vec<T>,
    /// leq(a, b) = true means a ≤ b
    relation: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T: Clone + Eq> Preorder<T> {
    pub fn new<F: Fn(&T, &T) -> bool + 'static>(elements: Vec<T>, relation: F) -> Self {
        Preorder {
            elements,
            relation: Box::new(relation),
        }
    }
    
    pub fn leq(&self, a: &T, b: &T) -> bool {
        (self.relation)(a, b)
    }
    
    pub fn is_reflexive(&self) -> bool {
        self.elements.iter().all(|x| self.leq(x, x))
    }
    
    pub fn is_transitive(&self) -> bool {
        for a in &self.elements {
            for b in &self.elements {
                for c in &self.elements {
                    if self.leq(a, b) && self.leq(b, c) && !self.leq(a, c) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

// Approach 2: Extended real enrichment (Lawvere metric spaces)
/// Lawvere metric space: [0,∞]-enriched category
#[derive(Debug, Clone)]
pub struct LawvereMetric<T> {
    elements: Vec<T>,
    /// Distance function d(a,b)
    distance: Box<dyn Fn(&T, &T) -> f64>,
}

impl<T: Clone + Eq> LawvereMetric<T> {
    pub fn new<F: Fn(&T, &T) -> f64 + 'static>(elements: Vec<T>, distance: F) -> Self {
        LawvereMetric {
            elements,
            distance: Box::new(distance),
        }
    }
    
    pub fn dist(&self, a: &T, b: &T) -> f64 {
        (self.distance)(a, b)
    }
    
    /// Check d(a,a) = 0
    pub fn reflexivity_holds(&self) -> bool {
        self.elements.iter().all(|x| self.dist(x, x) == 0.0)
    }
    
    /// Check d(a,c) ≤ d(a,b) + d(b,c)
    pub fn triangle_holds(&self) -> bool {
        for a in &self.elements {
            for b in &self.elements {
                for c in &self.elements {
                    if self.dist(a, c) > self.dist(a, b) + self.dist(b, c) + 0.0001 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

// Approach 3: Cost-enriched category (for optimization)
/// Cost-enriched category for pathfinding
#[derive(Debug, Clone)]
pub struct CostEnriched<V: Hash + Eq + Clone> {
    vertices: Vec<V>,
    costs: HashMap<(V, V), f64>,
}

impl<V: Hash + Eq + Clone> CostEnriched<V> {
    pub fn new() -> Self {
        CostEnriched {
            vertices: Vec::new(),
            costs: HashMap::new(),
        }
    }
    
    pub fn add_vertex(&mut self, v: V) {
        if !self.vertices.contains(&v) {
            self.vertices.push(v.clone());
            self.costs.insert((v.clone(), v), 0.0);
        }
    }
    
    pub fn set_cost(&mut self, from: V, to: V, cost: f64) {
        self.costs.insert((from, to), cost);
    }
    
    pub fn get_cost(&self, from: &V, to: &V) -> f64 {
        *self.costs.get(&(from.clone(), to.clone())).unwrap_or(&f64::INFINITY)
    }
    
    /// Composition cost = sum (tensor in [0,∞])
    pub fn compose_cost(&self, a: &V, b: &V, c: &V) -> f64 {
        self.get_cost(a, b) + self.get_cost(b, c)
    }
}

impl<V: Hash + Eq + Clone> Default for CostEnriched<V> {
    fn default() -> Self {
        Self::new()
    }
}

// Practical: weighted graph as enriched category
pub fn floyd_warshall(graph: &mut CostEnriched<usize>) {
    let vertices: Vec<usize> = graph.vertices.clone();
    let n = vertices.len();
    
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let via_k = graph.get_cost(&vertices[i], &vertices[k])
                         + graph.get_cost(&vertices[k], &vertices[j]);
                let direct = graph.get_cost(&vertices[i], &vertices[j]);
                if via_k < direct {
                    graph.set_cost(vertices[i].clone(), vertices[j].clone(), via_k);
                }
            }
        }
    }
}

/// Hom functor value type for V-enriched categories
pub struct HomValue<V>(pub V);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preorder_reflexive() {
        let pre = Preorder::new(
            vec![1, 2, 3],
            |a: &i32, b: &i32| a <= b
        );
        assert!(pre.is_reflexive());
    }

    #[test]
    fn test_preorder_transitive() {
        let pre = Preorder::new(
            vec![1, 2, 3],
            |a: &i32, b: &i32| a <= b
        );
        assert!(pre.is_transitive());
    }

    #[test]
    fn test_preorder_leq() {
        let pre = Preorder::new(
            vec![1, 2, 3],
            |a: &i32, b: &i32| a <= b
        );
        assert!(pre.leq(&1, &2));
        assert!(!pre.leq(&2, &1));
    }

    #[test]
    fn test_lawvere_metric() {
        let metric = LawvereMetric::new(
            vec![0.0, 1.0, 2.0],
            |a: &f64, b: &f64| (a - b).abs()
        );
        assert!(metric.reflexivity_holds());
        assert!(metric.triangle_holds());
    }

    #[test]
    fn test_cost_enriched() {
        let mut graph = CostEnriched::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        
        graph.set_cost(1, 2, 4.0);
        graph.set_cost(2, 3, 3.0);
        graph.set_cost(1, 3, 10.0);
        
        assert_eq!(graph.get_cost(&1, &2), 4.0);
        assert_eq!(graph.compose_cost(&1, &2, &3), 7.0);
    }

    #[test]
    fn test_floyd_warshall() {
        let mut graph = CostEnriched::new();
        for i in 0..3 { graph.add_vertex(i); }
        
        graph.set_cost(0, 1, 4.0);
        graph.set_cost(1, 2, 3.0);
        graph.set_cost(0, 2, 10.0);
        
        floyd_warshall(&mut graph);
        
        // Should find path 0->1->2 = 7 < direct 10
        assert_eq!(graph.get_cost(&0, &2), 7.0);
    }

    #[test]
    fn test_identity_cost() {
        let mut graph = CostEnriched::new();
        graph.add_vertex(1);
        assert_eq!(graph.get_cost(&1, &1), 0.0);
    }
}
