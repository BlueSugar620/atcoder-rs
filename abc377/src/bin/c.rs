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
        ab: [(Usize1, Usize1); m],
    }

    let mut set = Set::new();
    for &(a, b) in &ab {
        set.insert((a, b));
        for (da, db) in [
            (2, 1),
            (1, 2),
            (!0, 2),
            (!1, 1),
            (!1, !0),
            (!0, !1),
            (1, !1),
            (2, !0),
        ] {
            let a = a.wrapping_add(da);
            let b = b.wrapping_add(db);
            if a < n && b < n {
                set.insert((a, b));
            }
        }
    }

    println!("{}", n * n - set.len());
}
