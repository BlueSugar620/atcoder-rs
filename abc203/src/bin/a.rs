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
        x: [u8; 3],
    }

    if x[0] != x[1] && x[1] != x[2] && x[2] != x[0] {
        println!("0");
    } else {
        println!("{}", x.iter().fold(0, |acc, a| acc ^ a));
    }
}
