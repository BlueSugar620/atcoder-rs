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

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        q: usize,
        ask: [(Usize1, usize); q],
    }

    let mut e = vec![vec![]; n];
    for (i, p) in p.iter().enumerate() {
        e[*p].push(i + 1);
    }

    let mut in_time = vec![!0; n];
    let mut out_time = vec![!0; n];
    let mut depth_in = vec![vec![]; n + 1];
    dfs(
        0,
        &mut 0,
        &mut 0,
        &e,
        &mut in_time,
        &mut out_time,
        &mut depth_in,
    );

    for &(u, d) in &ask {
        println!(
            "{}",
            depth_in[d].lower_bound(&out_time[u]) - depth_in[d].lower_bound(&in_time[u])
        );
    }
}

fn dfs(
    i: usize,
    t: &mut usize,
    d: &mut usize,
    e: &Vec<Vec<usize>>,
    in_time: &mut [usize],
    out_time: &mut [usize],
    depth_in: &mut Vec<Vec<usize>>,
) {
    in_time[i] = *t;
    depth_in[*d].push(*t);
    *t += 1;
    for j in &e[i] {
        *d += 1;
        dfs(*j, t, d, e, in_time, out_time, depth_in);
        *d -= 1;
    }
    out_time[i] = *t;
    *t += 1;
}
