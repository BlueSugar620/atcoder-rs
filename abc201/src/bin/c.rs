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
        s: Bytes,
    }

    let o = ('0'..='9')
        .filter(|i| s[*i as usize - '0' as usize] == b'o')
        .collect::<Vec<_>>();
    let h = ('0'..='9')
        .filter(|i| s[*i as usize - '0' as usize] == b'?')
        .collect::<Vec<_>>();

    let ans = (0..10000)
        .filter(|i| {
            let v = format!("{i:0>4}").chars().collect::<Vec<_>>();
            o.iter().all(|o| v.contains(o)) && v.iter().all(|v| o.contains(v) || h.contains(v))
        })
        .count();

    println!("{}", ans);
}
