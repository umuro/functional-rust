// Suffix Array — O(n log² n) prefix doubling + O(n) LCP (Kasai)

fn build_sa(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i64> = s.iter().map(|&c| c as i64).collect();
    let mut tmp  = vec![0i64; n];
    let mut gap  = 1usize;

    while gap < n {
        let g = gap;
        let rank_ref = &rank;
        sa.sort_unstable_by(|&i, &j| {
            let ri = rank_ref[i];
            let rj = rank_ref[j];
            if ri != rj { return ri.cmp(&rj); }
            let ri2 = if i + g < n { rank_ref[i + g] } else { -1 };
            let rj2 = if j + g < n { rank_ref[j + g] } else { -1 };
            ri2.cmp(&rj2)
        });
        // Update ranks
        tmp[sa[0]] = 0;
        for i in 1..n {
            let (pi, ci) = (sa[i - 1], sa[i]);
            let same = rank[pi] == rank[ci]
                && (pi + g < n) == (ci + g < n)
                && (pi + g >= n || rank[pi + g] == rank[ci + g]);
            tmp[ci] = tmp[pi] + if same { 0 } else { 1 };
        }
        rank.copy_from_slice(&tmp);
        gap *= 2;
    }
    sa
}

fn build_lcp(s: &[u8], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0usize; n];
    for (i, &v) in sa.iter().enumerate() { rank[v] = i; }
    let mut lcp = vec![0usize; n];
    let mut k   = 0usize;
    for i in 0..n {
        if rank[i] > 0 {
            let j = sa[rank[i] - 1];
            while i + k < n && j + k < n && s[i + k] == s[j + k] { k += 1; }
            lcp[rank[i]] = k;
            if k > 0 { k -= 1; }
        }
    }
    lcp
}

fn sa_search(s: &[u8], sa: &[usize], pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let left  = sa.partition_point(|&i| &s[i..s.len().min(i + m)] < pattern);
    let right = sa.partition_point(|&i| &s[i..s.len().min(i + m)] <= pattern);
    let mut positions: Vec<usize> = sa[left..right].to_vec();
    positions.sort_unstable();
    positions
}

fn main() {
    let s   = b"banana";
    let sa  = build_sa(s);
    let lcp = build_lcp(s, &sa);

    println!("String: \"banana\"");
    println!("SA:  {:?}", sa);
    println!("LCP: {:?}", lcp);
    println!("Suffixes in order:");
    for &i in &sa {
        println!("  {i}: {:?}", std::str::from_utf8(&s[i..]).unwrap());
    }
    let positions = sa_search(s, &sa, b"an");
    println!("Search 'an': {:?}", positions);

    // Larger example
    let s2  = b"mississippi";
    let sa2 = build_sa(s2);
    println!("\n\"mississippi\" SA: {:?}", sa2);
    println!("Search 'issi': {:?}", sa_search(s2, &sa2, b"issi"));
}
