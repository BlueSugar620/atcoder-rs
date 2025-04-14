use __std_iter::FromIterator;
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
        mut a: [u64; n],
        mut b: [u64; n - 1],
    }

    a.sort_unstable();
    a.reverse();
    b.sort_unstable();
    b.reverse();
    let mut ans = !0;
    let mut i = 0;
    for &a in &a {
        if i < n - 1 && a <= b[i] {
            i += 1;
        } else {
            if ans == !0 {
                ans = a;
            } else {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", ans);
}
