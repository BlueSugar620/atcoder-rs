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
        c: u32,
        t: [u32; n],
    }

    let mut i = t[0];
    let mut ans = 1;
    for &t in &t[1..] {
        if t - i >= c {
            ans += 1;
            i = t;
        }
    }
    println!("{}", ans);
}
