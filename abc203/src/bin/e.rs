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
        n: usize,
        m: usize,
        mut xy: [(usize, usize); m],
    }

    let mut ans = Set::new();
    ans.insert(n);
    xy.sort_unstable();

    let mut now = 0;
    let mut set = Set::new();
    let mut fix_set = Set::new();
    for &(x, y) in &xy {
        if now != x {
            for a in set.iter() {
                ans.insert(*a);
            }
            set.clear();
            for a in fix_set.iter() {
                ans.remove(a);
            }
            fix_set.clear();
            now = x;
        }
        if ans.contains(&y) {
            fix_set.insert(y);
        }
        if (y > 0 && ans.contains(&(y - 1))) || ans.contains(&(y + 1)) {
            set.insert(y);
            fix_set.remove(&y);
        }
    }
    for a in set.iter() {
        ans.insert(*a);
    }
    set.clear();
    for a in fix_set.iter() {
        ans.remove(a);
    }
    fix_set.clear();

    println!("{}", ans.len());
}
