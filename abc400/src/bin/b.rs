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
        n: u128,
        m: u32,
    }

    let mut x = 0u128;
    let mut t = 1u128;
    for _ in 0..=m {
        x = x.saturating_add(t);
        t = t.saturating_mul(n);
    }
    if x > 10u128.pow(9) {
        println!("inf");
    } else {
        println!("{}", x);
    }
}
