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
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[a].push(b);
    }

    let mut que = Deque::new();
    let mut dist = vec![!0; n];
    que.push_back(0);
    dist[0] = 0;

    while let Some(u) = que.pop_front() {
        let d = dist[u];
        for &v in &e[u] {
            if v == 0 {
                println!("{}", d + 1);
                return;
            } else if dist[v] == !0 {
                que.push_back(v);
                dist[v] = d + 1;
            }
        }
    }

    println!("-1");
}
