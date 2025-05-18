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
        h: usize,
        w: usize,
        n: usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
        ask: [(Usize1, Usize1); q],
    }

    let mut v = [vec![vec![]; h], vec![vec![]; w]];
    for (i, &(x, y)) in xy.iter().enumerate() {
        v[0][x].push(i);
        v[1][y].push(i);
    }
    let mut queries = [vec![!0; h], vec![!0; w]];
    let mut used = vec![true; n];

    for (t, a) in &ask {
        if queries[*t][*a] == !0 {
            let s = &v[*t][*a];
            let ans = s.iter().filter(|a| used[**a]).count();
            println!("{}", ans);
            for &a in s {
                used[a] = false;
            }
            queries[*t][*a] = ans;
        } else {
            println!("{}", 0);
        }
    }
}
