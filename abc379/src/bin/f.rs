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
        q: usize,
        h: [u64; n],
        ask: [(Usize1, Usize1); q],
    }

    let mut idxs = (0..q).collect_vec();
    idxs.sort_unstable_by_key(|i| !ask[*i].0);
    let mut ans = vec![0; q];

    let mut v = vec![];
    let mut idx = 0;
    for (i, hi) in h.iter().copied().enumerate().rev() {
        while idx < q && i == ask[idxs[idx]].0 {
            if ask[idxs[idx]].1 >= v[0] {
                ans[idxs[idx]] = 0;
            } else {
                let mut l = 0;
                let mut r = v.len();
                while r - l > 1 {
                    let o = (r + l) / 2;
                    if v[o] <= ask[idxs[idx]].1 {
                        r = o;
                    } else {
                        l = o;
                    }
                }
                ans[idxs[idx]] = r;
            }
            idx += 1;
        }
        while v.len() > 0 && h[*v.last().unwrap()] < hi {
            v.pop();
        }
        v.push(i);
    }

    println!("{}", ans.iter().join("\n"));
}
