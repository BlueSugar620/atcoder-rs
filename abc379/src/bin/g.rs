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
const LEN: usize = 14;
fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        mut s: [Bytes; h],
    }

    if h < w {
        std::mem::swap(&mut h, &mut w);
        let mut swap = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                swap[i][j] = s[j][i];
            }
        }
        s = swap;
    }

    let mut dp = Map::new();
    for c in 0..3 {
        for bit in 0..1 << (w - 1) {
            let mut p = c;
            let mut ok = true;
            let mut key = [0; LEN];
            for (i, s) in s[0].iter().enumerate() {
                if i > 0 {
                    p = (p + 1 + (bit >> (i - 1) & 1)) % 3;
                }
                key[i] = p;
                if *s != b'?' {
                    let k = (*s - b'1') as usize;
                    if k != p {
                        ok = false;
                    }
                }
            }
            if ok {
                dp.insert(key, 1);
            }
        }
    }

    for s in s[1..].iter() {
        for (j, c) in s.iter().enumerate() {
            let mut next = Map::new();
            for (key, w) in dp {
                for k in 0..3 {
                    if k == key[j]
                        || (*c != b'?' && (*c - b'1') as usize != k)
                        || (j > 0 && key[j - 1] == k)
                    {
                        continue;
                    }
                    let mut key = key;
                    key[j] = k;
                    *next.entry(key).or_insert(0) += w;
                    *next.entry(key).or_insert(0) %= MOD;
                }
            }
            dp = next;
        }
    }

    let ans = dp.values().fold(0, |a, b| {
        let mut c = a + b;
        if c >= MOD {
            c -= MOD;
        }
        c
    });
    println!("{}", ans);
}
