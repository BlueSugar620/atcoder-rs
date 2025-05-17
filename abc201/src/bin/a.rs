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
        mut a: [u8; 3],
    }
    a.sort_unstable();
    println!("{}", if a[0] + a[2] == 2 * a[1] { "Yes" } else { "No" });
}
