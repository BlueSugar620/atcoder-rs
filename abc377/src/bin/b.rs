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
        s: [Bytes; 8],
    }

    let ans = iproduct!(0..8, 0..8)
        .filter(|&(i, j)| (0..8).all(|x| s[x][j] == b'.') && (0..8).all(|y| s[i][y] == b'.'))
        .count();
    println!("{}", ans);
}
