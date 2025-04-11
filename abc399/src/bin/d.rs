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
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [Usize1; 2 * n],
        }

        let mut pos0 = vec![n; n];
        let mut pos1 = vec![n; n];
        for (i, &a) in a.iter().enumerate() {
            if pos0[a] == n {
                pos0[a] = i;
            } else {
                pos1[a] = i;
            }
        }
        let mut b = vec![];
        for (i, &a) in a.iter().enumerate() {
            if pos0[a] == i {
                b.push(pos1[a]);
            } else {
                b.push(pos0[a]);
            }
        }

        let mut ans = 0usize;
        for i in 0..2 * n - 1 {
            let x = a[i];
            let y = a[i + 1];
            if x == y {
                continue;
            }
            let k = b[i];
            let l = b[i + 1];
            if i.abs_diff(k) != 1 && (i + 1).abs_diff(l) != 1 && k.abs_diff(l) == 1 {
                ans += 1;
            }
        }

        println!("{}", ans / 2);
    }
}
