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
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    }

    let mut freq_a = vec![0; n];
    let mut freq_b = vec![0; n];
    let mut freq_c = vec![0; n];
    for a in &a {
        freq_a[*a] += 1;
    }
    for c in &c {
        freq_c[*c] += 1;
    }
    for (i, b) in b.iter().enumerate() {
        freq_b[*b] += freq_c[i];
    }

    let ans = (0..n).map(|i| freq_a[i] * freq_b[i]).sum::<usize>();
    println!("{}", ans);
}
