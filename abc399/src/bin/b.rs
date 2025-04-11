use itertools::*;
use memoise::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut q = p.clone();
    q.sort_unstable();
    q.reverse();

    for &p in &p {
        println!("{}", q.iter().position(|q| *q == p).unwrap() + 1);
    }
}
