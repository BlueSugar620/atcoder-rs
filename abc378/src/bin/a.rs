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
        mut a: [u8; 4],
    }

    a.sort_unstable();
    if a[0] == a[1] && a[2] == a[3] {
        println!("2");
        return;
    }
    a.dedup();
    if a.len() != 4 {
        println!("1");
        return;
    }
    println!("0");
}
