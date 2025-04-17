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

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = 0;
    let mut is = false;
    for s in &s {
        if s == "login" {
            is = true;
        } else if s == "logout" {
            is = false;
        } else if s == "private" && !is {
            ans += 1;
        }
    }

    println!("{}", ans);
}
