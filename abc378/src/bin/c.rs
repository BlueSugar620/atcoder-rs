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
        a: [usize; n],
    }

    let mut map = Map::new();
    let mut b = vec![];
    for (i, a) in a.iter().enumerate() {
        let x = map.get(a).unwrap_or(&!0);
        b.push(*x as isize);
        map.insert(*a, i + 1);
    }

    println!("{}", b.iter().join(" "));
}
