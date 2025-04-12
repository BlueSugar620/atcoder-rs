use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
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
        m: usize,
        mut lr: [(Usize1, usize); n],
    }
    lr.sort_unstable_by_key(|(l, _)| !l);
    let mut i = 0;
    let mut ans = 0;
    let mut r = m + 1;
    for l in (0..m).rev() {
        while i < n && lr[i].0 == l {
            r = r.min(lr[i].1);
            i += 1;
        }
        ans += r - l - 1;
    }

    println!("{}", ans);
}
