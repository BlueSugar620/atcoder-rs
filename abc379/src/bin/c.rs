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
        m: usize,
        mut x: [Usize1; m],
        mut a: [usize; m],
    }
    x.push(n);
    a.push(0);

    if a.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }

    let idx = (0..m + 1).sorted_unstable_by_key(|i| x[*i]).collect_vec();
    let mut s = 0;

    for i in idx {
        let x = x[i];
        let a = a[i];
        if s < x {
            println!("-1");
            return;
        }
        s += a;
    }

    println!(
        "{}",
        n * (n - 1) / 2 - x.iter().zip(a.iter()).map(|(x, a)| x * a).sum::<usize>()
    );
}
