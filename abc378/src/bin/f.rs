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
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }
    let mut ans = 0;
    dfs(0, 0, &e, &mut ans);
    println!("{}", ans);
}

fn dfs(p: usize, i: usize, e: &Vec<Vec<usize>>, ans: &mut usize) -> usize {
    if e[i] == vec![p] {
        return 0;
    }
    let mut t = 0;
    for j in &e[i] {
        if *j == p {
            continue;
        }
        let x = dfs(i, *j, e, ans);
        *ans += t * x;
        if e[*j].len() == 2 {
            t += x;
        }
    }
    t
}
