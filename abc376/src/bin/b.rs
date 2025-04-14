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
        q: usize,
        ht: [(char, Usize1); q],
    }

    let mut l = 0usize;
    let mut r = 1usize;
    let mut ans = 0;
    for &(h, t) in &ht {
        if h == 'L' {
            if t.min(l) < r && r < t.max(l) {
                ans += n - t.max(l) + t.min(l);
            } else {
                ans += l.abs_diff(t);
            }
            l = t;
        } else {
            if t.min(r) < l && l < t.max(r) {
                ans += n - t.max(r) + t.min(r);
            } else {
                ans += r.abs_diff(t);
            }
            r = t;
        }
    }

    println!("{}", ans);
}
