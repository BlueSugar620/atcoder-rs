use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt, Roots};
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
        n: u64,
    }

    println!("{}", sqrt(n / 2) + sqrt(n / 4));
}
