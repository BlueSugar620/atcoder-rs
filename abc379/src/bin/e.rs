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
        n: usize,
        s: Bytes,
    }

    let mut t = 0;
    let mut v = vec![];
    for (i, s) in s.iter().enumerate() {
        let s = (s - b'0') as usize;
        t += s * (i + 1);
        v.push(t);
    }

    let mut u = 0;
    for v in v.iter_mut().rev() {
        *v += u;
        u = *v / 10;
        *v %= 10;
    }
    if u != 0 {
        v.insert(0, u);
    }

    println!("{}", v.iter().join(""));
}
