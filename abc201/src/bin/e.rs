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
type Heap<T> = BinaryHeap<T>;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, u64); n - 1],
    }

    let mut e = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        e[u].push((v, w));
        e[v].push((u, w));
    }

    let mut dist = vec![!0; n];
    let mut stack = vec![0];
    dist[0] = 0;
    while let Some(i) = stack.pop() {
        for &(j, d) in &e[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] ^ d;
                stack.push(j);
            }
        }
    }

    let mut ans = xor_2d_sum(&dist, &dist);
    if ans & 1 == 1 {
        ans += MOD;
    }
    println!("{}", ans / 2);
}

fn xor_2d_sum(f: &[u64], g: &[u64]) -> u64 {
    let n = f.len();
    let m = g.len();

    let mut cnt_f = vec![0u64; 64];
    let mut cnt_g = vec![0u64; 64];
    for (k, cnt_f) in cnt_f.iter_mut().enumerate() {
        for f in f {
            if (f >> k) & 1 == 1 {
                *cnt_f += 1;
            }
        }
    }
    for (k, cnt_g) in cnt_g.iter_mut().enumerate() {
        for g in g {
            if (g >> k) & 1 == 1 {
                *cnt_g += 1;
            }
        }
    }

    (0..64)
        .map(|k| {
            (cnt_f[k] * (m as u64 - cnt_g[k]) % MOD + (n as u64 - cnt_f[k]) * cnt_g[k] % MOD)
                * ((1 << k) % MOD)
                % MOD
        })
        .fold(0, |acc, a| (acc + a) % MOD)
}
