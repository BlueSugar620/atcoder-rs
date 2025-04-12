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
        s: [Bytes; n],
    }

    let mut rng = thread_rng();
    let base = rng.gen::<u64>() % (1 << 32);

    let mut map = std::collections::HashMap::new();
    let empty = RollingHash::<HASH>::new(&vec![], base);
    map.insert(empty.hash(..), 0);

    for s in s.iter() {
        let s = s.iter().map(|c| (c - b'a') as u64).collect_vec();
        let mut ans = !0;
        let hs = RollingHash::<HASH>::new(&s, base);
        for i in (0..=s.len()).rev() {
            let x = hs.hash(..i);
            if map.contains_key(&x) {
                let v = *map.get(&x).unwrap();
                ans = ans.min(s.len() + v - 2 * i);
            }
        }
        println!("{}", ans);
        for i in 0..=s.len() {
            let x = hs.hash(..i);
            *map.entry(x).or_insert(!0) = *map.get(&x).unwrap_or(&!0).min(&s.len());
        }
    }
}

use std::ops::{Bound, RangeBounds};

const HASH: u64 = (1 << 61) - 1;

pub struct RollingHash<const HASH: u64> {
    n: usize,
    hash_acc: Vec<u64>,
    base_pow: Vec<u64>,
}

impl<const HASH: u64> RollingHash<HASH> {
    pub fn new(a: &[u64], base: u64) -> Self {
        let mut hash_acc = vec![0];
        let mut base_pow = vec![1];
        for a in a {
            let h = mul_hash(hash_acc.last().unwrap(), &base) + a + 1;
            hash_acc.push(if h >= HASH { h - HASH } else { h });
            base_pow.push(mul_hash(base_pow.last().unwrap(), &base));
        }
        Self {
            n: a.len(),
            hash_acc,
            base_pow,
        }
    }

    pub fn hash<R: RangeBounds<usize>>(&self, range: R) -> u64 {
        let (l, r) = unzip(range, self.n);
        let l = mul_hash(&self.hash_acc[l], &self.base_pow[r - l]);
        let r = self.hash_acc[r];
        if r < l {
            panic!();
            HASH - l + r
        } else {
            r - l
        }
    }
}

fn mul_hash(a: &u64, b: &u64) -> u64 {
    let t = *a as u128 * *b as u128;
    let u = (t >> 61) as u64 + (t & HASH as u128) as u64;
    if u >= HASH {
        u - HASH
    } else {
        u
    }
}

fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
}
