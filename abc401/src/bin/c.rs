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

const MOD: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = vec![1; k];
    let mut sum = k;
    if n < k {
        println!("1");
        return;
    }
    for i in k..=n {
        a.push(sum);
        sum += sum + MOD - a[i - k];
        sum %= MOD;
    }

    println!("{}", a[n]);
}
