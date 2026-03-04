/// Rose Tree — Multi-Way Tree with Fold

#[derive(Debug, Clone, PartialEq)]
pub struct Rose<T> {
    pub value: T,
    pub children: Vec<Rose<T>>,
}

impl<T> Rose<T> {
    pub fn leaf(value: T) -> Self {
        Rose {
            value,
            children: vec![],
        }
    }

    pub fn node(value: T, children: Vec<Rose<T>>) -> Self {
        Rose { value, children }
    }

    /// Bottom-up fold: applies f to each node's value and its children's fold results.
    pub fn fold<R>(&self, f: &dyn Fn(&T, Vec<R>) -> R) -> R {
        let child_results: Vec<R> = self.children.iter().map(|c| c.fold(f)).collect();
        f(&self.value, child_results)
    }
}

pub fn size<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, sizes: Vec<usize>| 1 + sizes.iter().sum::<usize>())
}

pub fn depth<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, depths: Vec<usize>| {
        1 + depths.iter().copied().max().unwrap_or(0)
    })
}

pub fn to_string_repr(tree: &Rose<&str>) -> String {
    tree.fold(&|&x, strs: Vec<String>| {
        if strs.is_empty() {
            x.to_string()
        } else {
            format!("{}({})", x, strs.join(","))
        }
    })
}

fn main() {
    let tree = Rose::node(
        "a",
        vec![
            Rose::node("b", vec![Rose::leaf("d"), Rose::leaf("e")]),
            Rose::node("c", vec![Rose::leaf("f")]),
        ],
    );

    println!("size={} depth={} repr={}", size(&tree), depth(&tree), to_string_repr(&tree));
}

/* Output:
   size=6 depth=3 repr=a(b(d,e),c(f))
*/
