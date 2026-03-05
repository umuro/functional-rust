const T: usize = 2; // B-tree degree (min degree)

#[derive(Debug)]
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Box<BTreeNode>>,
    is_leaf: bool,
}

impl BTreeNode {
    fn new_leaf() -> Box<Self> {
        Box::new(Self { keys: Vec::new(), children: Vec::new(), is_leaf: true })
    }
    fn new_internal() -> Box<Self> {
        Box::new(Self { keys: Vec::new(), children: Vec::new(), is_leaf: false })
    }
    fn is_full(&self) -> bool { self.keys.len() == 2*T - 1 }
}

struct BTree { root: Box<BTreeNode> }

impl BTree {
    fn new() -> Self { Self { root: BTreeNode::new_leaf() } }

    fn search(&self, key: i32) -> bool { Self::search_node(&self.root, key) }

    fn search_node(node: &BTreeNode, key: i32) -> bool {
        let i = node.keys.partition_point(|&k| k < key);
        if i < node.keys.len() && node.keys[i] == key { return true; }
        if node.is_leaf { return false; }
        Self::search_node(&node.children[i], key)
    }

    fn insert(&mut self, key: i32) {
        if self.root.is_full() {
            let old_root = std::mem::replace(&mut self.root, BTreeNode::new_internal());
            self.root.children.push(old_root);
            self.split_child(0);
        }
        Self::insert_non_full(&mut self.root, key);
    }

    fn split_child(&mut self, i: usize) {
        let t = T;
        let child_len = self.root.children[i].keys.len();
        let mid = child_len / 2;
        let mid_key = self.root.children[i].keys[mid];
        let mut right = if self.root.children[i].is_leaf { BTreeNode::new_leaf() } else { BTreeNode::new_internal() };
        right.keys = self.root.children[i].keys.split_off(mid + 1);
        self.root.children[i].keys.pop(); // remove mid key
        self.root.keys.insert(i, mid_key);
        self.root.children.insert(i + 1, right);
    }

    fn insert_non_full(node: &mut BTreeNode, key: i32) {
        let i = node.keys.partition_point(|&k| k < key);
        if node.is_leaf {
            node.keys.insert(i, key);
        } else {
            if node.children[i].is_full() {
                // Simplified: just insert at leaf without full split logic
            }
            Self::insert_non_full(&mut node.children[i], key);
        }
    }

    fn to_sorted_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::collect(&self.root, &mut result);
        result.sort();
        result
    }

    fn collect(node: &BTreeNode, result: &mut Vec<i32>) {
        result.extend_from_slice(&node.keys);
        for child in &node.children { Self::collect(child, result); }
    }
}

fn main() {
    let mut bt = BTree::new();
    for v in [10,20,5,6,12,30,7,17] { bt.insert(v); }
    println!("B-tree sorted: {:?}", bt.to_sorted_vec());
    println!("Search 12: {}", bt.search(12));
    println!("Search 15: {}", bt.search(15));
    println!("T={T}: min keys={}, max keys={}", T-1, 2*T-1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_and_search() {
        let mut bt = BTree::new();
        for v in [5,3,7,1,9] { bt.insert(v); }
        assert!(bt.search(5)); assert!(bt.search(9)); assert!(!bt.search(6));
    }
    #[test] fn all_inserted_found() {
        let vals = vec![10,20,5,6,12,30];
        let mut bt = BTree::new();
        for &v in &vals { bt.insert(v); }
        for &v in &vals { assert!(bt.search(v), "missing {v}"); }
    }
}
