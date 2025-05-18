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
    println!(
        "{}",
        s.iter()
            .rev()
            .map(|&c| if c == b'6' {
                b'9'
            } else if c == b'9' {
                b'6'
            } else {
                c
            })
            .map(|c| c as char)
            .join("")
    );
}
