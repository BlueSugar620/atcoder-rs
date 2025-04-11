use itertools::*;
use memoise::*;
use proconio::marker::*;
use proconio::*;
use rand::Rng;
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
        t: Bytes,
    }

    let mut dsu_s = DisjointSetUnion::new(n + 26);
    let mut dsu_t = DisjointSetUnion::new(n + 26);

    for (i, s) in s.iter().enumerate() {
        let s = (s - b'a') as usize;
        dsu_s.unite(i, n + s);
    }
    for (i, t) in t.iter().enumerate() {
        let t = (t - b'a') as usize;
        dsu_t.unite(i, n + t);
    }

    for i in 0..n {
        if !dsu_t.is_same(dsu_s.root(i), i) {
            println!("-1");
            return;
        }
    }

    let mut ts = vec![Set::new(); 26];
    for i in 0..n {
        let s = (s[i] - b'a') as usize;
        let t = (t[i] - b'a') as usize;
        ts[t].insert(s);
    }

    let mut ans = 0;
    let mut aki = (0..26).filter(|i| dsu_s.size(n + i) == 1).collect_vec();
    for i in 0..26 {
        if ts[i].contains(&i) {
            ans += ts[i].len() - 1;
            for &x in &ts[i] {
                if x != i {
                    aki.push(x);
                }
            }
        }
    }
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let i: i32 = rng.gen(); // 整数値の乱数を生成する
    let mut flag = if i % 2 == 1 { true } else { false };
    //println!(
    //    "{:?}",
    //    (0..26)
    //        .filter(|i| !ts[*i].contains(i) && ts[*i].len() > 0)
    //        .collect_vec()
    //);
    //println!(
    //    "{:?}",
    //    (0..26)
    //        .filter(|i| (0..26).any(|j| ts[j].contains(i) && !ts[j].contains(&&j)))
    //        .collect_vec()
    //);

    for i in 0..26 {
        if !ts[i].contains(&i) && ts[i].len() > 0 {
            ans += ts[i].len();
            for &x in &ts[i] {
                if x != i {
                    aki.push(x);
                }
            }
        }
    }

    println!("{}", ans + if flag { 1 } else { 0 });
}

pub struct DisjointSetUnion {
    parents: Vec<isize>,
    cnt: usize,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            cnt: n,
        }
    }

    pub fn root(&self, mut v: usize) -> usize {
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
        }
        v
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let mut u = self.root(u);
        let mut v = self.root(v);
        if u == v {
            return;
        }
        if self.parents[u] > self.parents[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.parents[u] += self.parents[v];
        self.parents[v] = u as isize;
        self.cnt -= 1;
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn size(&self, v: usize) -> usize {
        -self.parents[self.root(v)] as usize
    }

    pub fn cnt(&self) -> usize {
        self.cnt
    }
}
