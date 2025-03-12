use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut ans = 0;
    let mut edge = std::collections::HashSet::new();
    for &(u, v) in &uv {
        if u == v || edge.contains(&(u, v)) || edge.contains(&(v, u)) {
            ans += 1;
        } else {
            edge.insert((u, v));
        }
    }

    println!("{}", ans);
}
