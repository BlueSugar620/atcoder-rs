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
        mut k: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable();
    ab.push((!0, 0));

    let mut ans = 0;
    for &(a, b) in &ab {
        if a - ans > k {
            println!("{}", ans + k);
            return;
        }
        k -= a - ans;
        k += b;
        ans = a;
    }
    panic!();
}
