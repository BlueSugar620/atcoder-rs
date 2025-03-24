use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Bytes,
        ask: [(Usize1, usize); q],
    }

    let a = (0..n).filter(|i| s[*i] == b'1').collect::<Vec<_>>();
    let b = (0..n).filter(|i| s[*i] == b'2').collect::<Vec<_>>();
    let c = (0..n).filter(|i| s[*i] == b'/').collect::<Vec<_>>();

    for &(l, r) in &ask {
        let y = c.lower_bound(&l);
        if c.len() <= y || r <= c[y] {
            println!("0");
            continue;
        }
        let mut ok = 0;
        let mut ng = n;
        while ng - ok > 1 {
            let o = (ng + ok) / 2;
            let x = a.lower_bound(&l) + o - 1;
            if a.len() <= x {
                ng = o;
                continue;
            }
            let y = c.lower_bound(&a[x]);
            if c.len() <= y {
                ng = o;
                continue;
            }
            let z = b.lower_bound(&c[y]) + o - 1;
            if b.len() <= z {
                ng = o;
                continue;
            }
            if b[z] < r {
                ok = o;
            } else {
                ng = o;
            }
        }
        println!("{}", 2 * ok + 1);
    }
}
