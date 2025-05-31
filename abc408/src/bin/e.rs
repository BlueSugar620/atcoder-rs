use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, u32); m],
    }
    let mut e = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    let mut ans = 0u32;
    for b in (0..31).rev() {
        let mut stack = vec![0];
        let mut dist = vec![false; n];
        dist[0] = true;
        while let Some(u) = stack.pop() {
            for &(v, w) in &e[u] {
                if w & ans ^ w < 1 << b && !dist[v] {
                    dist[v] = true;
                    stack.push(v);
                }
            }
        }
        if !dist[n - 1] {
            ans |= 1 << b;
        }
    }
    println!("{}", ans);
}
