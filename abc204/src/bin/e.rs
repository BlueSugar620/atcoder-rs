use std::collections::BinaryHeap;

use num::integer::sqrt;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1, Usize1, u64, u64); m],
    }
    let mut e = vec![vec![]; n];
    for &(a, b, c, d) in &abcd {
        e[a].push((b, c, d));
        e[b].push((a, c, d));
    }
    let mut time = vec![!0; n];
    let mut heap = BinaryHeap::new();
    time[0] = 0;
    heap.push((!0, 0));
    while let Some((t, u)) = heap.pop() {
        let t = !t;
        if time[u] < t {
            continue;
        }
        for &(v, c, d) in &e[u] {
            let nt = f(t, c, d);
            if nt < time[v] {
                time[v] = nt;
                heap.push((!nt, v));
            }
        }
    }
    println!("{}", time[n - 1] as i64);
}

fn f(t: u64, c: u64, d: u64) -> u64 {
    let sqrt_d = sqrt(d);
    let mut ans = t + c + d / (t + 1);
    for p in 0..10 {
        if sqrt_d + p < 5 {
            continue;
        } else {
            let x = sqrt_d + p - 5;
            if t <= x {
                ans = ans.min(x + c + d / (x + 1));
            }
        }
    }
    ans
}
