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
        k: usize,
        mut a: [u64; n],
    }

    if k == n {
        println!("0 {}", n);
        return;
    }

    a.sort_unstable();

    let mut aoki = 0;
    let mut takahashi = !0usize;

    for i in 0..=k {
        let mut t = 0;
        let mut max_height = a[n - 1 - i] / 2;
        while max_height > 0 {
            t += 1;
            let x = a.upper_bound(&max_height);
            if x == 0 {
                max_height = 0;
            } else {
                max_height = a[x - 1] / 2;
            }
        }
        if (t, i) <= (takahashi, aoki) {
            (takahashi, aoki) = (t, i);
        }
    }

    println!("{} {}", takahashi, aoki);
}
