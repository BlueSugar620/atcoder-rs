use itertools::*;
use memoise::*;
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
        qr: [(u64, u64); n],
        q: usize,
        ask: [(Usize1, u64); q],
    }

    for (t, d) in &ask {
        let (q, r) = qr[*t];
        let ans = if *d < r {
            r
        } else {
            (d - r + q - 1) / q * q + r
        };
        println!("{}", ans);
    }
}
