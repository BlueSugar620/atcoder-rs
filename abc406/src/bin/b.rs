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
        k: u32,
        a: [u64; n],
    }

    let mut ans = 1u64;
    for &a in &a {
        ans = ans.saturating_mul(a);
        if ans >= 10u64.pow(k) {
            ans = 1;
        }
    }
    println!("{}", ans);
}
