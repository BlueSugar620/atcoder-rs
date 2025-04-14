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
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [u64; n],
            b: [u64; n],
        }

        let idx = (0..n).sorted_by_key(|i| a[*i]).collect_vec();
        let mut s = 0;
        let mut heap = Heap::new();

        for &i in &idx[..k] {
            s += b[i];
            heap.push(b[i]);
        }
        let mut ans = a[idx[k - 1]] * s;

        for &i in &idx[k..] {
            let x = heap.pop().unwrap();
            s -= x;
            heap.push(b[i]);
            s += b[i];
            ans = ans.min(a[i] * s);
        }

        println!("{}", ans);
    }
}
