use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        k: u64,
        p: [Usize1; n],
    }

    let mut used = vec![false; n];
    let mut ans = vec![0; n];
    for i in 0..n {
        if used[i] {
            continue;
        }

        let mut cycle = vec![i];
        used[i] = true;
        let mut pos = i;
        while p[pos] != i {
            pos = p[pos];
            cycle.push(pos);
            used[pos] = true;
        }

        let x = pow_mod(2, k, cycle.len() as u64) as usize;
        for i in 0..cycle.len() {
            ans[cycle[i]] = cycle[(i + x) % cycle.len()] + 1;
        }
    }

    println!("{}", ans.iter().join(" "));
}

pub fn pow_mod(mut a: u64, mut k: u64, m: u64) -> u64 {
    let mut res = 1;
    while k > 0 {
        if k & 1 == 1 {
            res *= a;
            res %= m;
        }
        a *= a;
        a %= m;
        k >>= 1;
    }
    res
}
