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
        m: u64,
        a: [u64; n],
    }

    let mut s = vec![0];
    for &a in &a {
        s.push((s.last().unwrap() + a) % m);
    }
    let mut ans = 0;
    let mut r = 0;
    for l in (0..=n).rev() {
        r += s[l];
        ans += r;
    }
    let mut l = 0;
    for r in 0..=n {
        l += s[r];
        ans -= l;
    }

    ans += m * inversions(&s) as u64;
    println!("{}", ans);
}

pub fn inversions<T: Copy + PartialOrd>(a: &[T]) -> usize {
    if a.len() == 0 {
        return 0;
    }
    let compressed = compress(&a);
    let m = compressed.iter().max().unwrap() + 1;
    let mut res = 0;
    let mut fenwick = vec![0; m + 1];
    for (i, &x) in compressed.iter().enumerate() {
        res += i;
        {
            let mut x = x + 1;
            while x > 0 {
                res -= fenwick[x];
                x -= x & x.wrapping_neg();
            }
        }
        {
            let mut x = x + 1;
            while let Some(f) = fenwick.get_mut(x) {
                *f = *f + 1;
                x += x & x.wrapping_neg();
            }
        }
    }
    res
}

fn compress<T: Copy + PartialOrd>(a: &[T]) -> Vec<usize> {
    let mut val = a.to_vec();
    val.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    val.dedup();
    let mut res = Vec::with_capacity(a.len());
    for ai in a {
        res.push(val.partition_point(|x| x < ai));
    }
    res
}
