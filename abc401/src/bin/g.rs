use ac_library::*;
use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
use proconio::*;
use rand::{thread_rng, Rng};
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        s: [(f64, f64); n],
        g: [(f64, f64); n],
    }

    let mut e = vec![];
    for (i, (sx, sy)) in s.iter().enumerate() {
        for (j, (gx, gy)) in g.iter().enumerate() {
            e.push(((sx - gx).hypot(sy - gy), i, j));
        }
    }
    e.sort_unstable_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    let mut l = 0;
    let mut r = e.len();
    while r - l > 1 {
        let o = (r + l) / 2;
        let mut graph = MfGraph::new(2 * n + 2);
        for i in 0..n {
            graph.add_edge(2 * n, i, 1);
            graph.add_edge(n + i, 2 * n + 1, 1);
        }
        for &(_, i, j) in &e[..o] {
            graph.add_edge(i, n + j, 1);
        }
        let flow = graph.flow(2 * n, 2 * n + 1);
        if flow == n {
            r = o;
        } else {
            l = o;
        }
    }

    println!("{}", e[l].0);
}
