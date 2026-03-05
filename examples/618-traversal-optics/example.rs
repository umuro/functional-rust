// Traversal: optic focusing on multiple values with effects

// Traverse a Vec with Option effect (sequence + map)
fn traverse_opt<A, B>(xs: Vec<A>, f: impl Fn(A) -> Option<B>) -> Option<Vec<B>> {
    xs.into_iter().map(f).collect()
}

// Traverse with Result effect
fn traverse_result<A, B, E>(xs: Vec<A>, f: impl Fn(A) -> Result<B, E>) -> Result<Vec<B>, E> {
    xs.into_iter().map(f).collect()
}

// Traverse nested structure
fn traverse_matrix<A: Clone, B>(
    m: Vec<Vec<A>>,
    f: impl Fn(A) -> Option<B> + Clone,
) -> Option<Vec<Vec<B>>> {
    traverse_opt(m, |row| traverse_opt(row, f.clone()))
}

// Practical traversal: parse all fields
fn parse_all_ints(strs: &[&str]) -> Option<Vec<i32>> {
    traverse_opt(strs.to_vec(), |s| s.parse::<i32>().ok())
}

fn parse_all_floats(strs: &[&str]) -> Result<Vec<f64>, String> {
    traverse_result(strs.to_vec(), |s| {
        s.parse::<f64>().map_err(|e| format!("{}: {}", s, e))
    })
}

// Traversal as filter_map (some targets may be missing)
fn collect_values<A: Clone, B>(xs: &[A], prism: impl Fn(&A) -> Option<B>) -> Vec<B> {
    xs.iter().filter_map(prism).collect()
}

// Traversal over nested Option
fn traverse_nested<A, B>(opt: Option<Vec<A>>, f: impl Fn(A) -> Option<B>) -> Option<Vec<B>> {
    opt.and_then(|xs| traverse_opt(xs, f))
}

fn main() {
    println!("parse [1,2,3]: {:?}", parse_all_ints(&["1","2","3"]));
    println!("parse [1,x,3]: {:?}", parse_all_ints(&["1","x","3"]));
    println!("parse floats:  {:?}", parse_all_floats(&["1.5","2.7","3.14"]));
    println!("parse bad:     {:?}", parse_all_floats(&["1.5","abc"]));

    let matrix = vec![vec!["1","2"],vec!["3","4"]];
    let parsed = traverse_matrix(matrix, |s| s.parse::<i32>().ok());
    println!("matrix: {:?}", parsed);

    #[derive(Debug,Clone)]
    enum Json { Num(f64), Str(String), Null }
    let jsons = vec![Json::Num(1.0), Json::Str("hi".into()), Json::Num(2.0), Json::Null];
    let nums: Vec<f64> = collect_values(&jsons, |j| match j { Json::Num(n)=>Some(*n), _=>None });
    println!("nums from json: {:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn traverse_ok()  { assert_eq!(parse_all_ints(&["1","2","3"]), Some(vec![1,2,3])); }
    #[test] fn traverse_fail(){ assert_eq!(parse_all_ints(&["1","x","3"]), None); }
    #[test] fn traverse_mat() {
        let m = vec![vec!["1","2"],vec!["3","4"]];
        assert_eq!(traverse_matrix(m, |s|s.parse::<i32>().ok()), Some(vec![vec![1,2],vec![3,4]]));
    }
}
