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
        uv: [(Usize1, Usize1); m],
    }

    let mut dsu = DisjointSetUnion::new(n);
    for &(u, v) in &uv {
        dsu.unite(u, v);
    }

    println!(
        "{}",
        m - (0..n)
            .map(|i| if dsu.root(i) == i { dsu.size(i) - 1 } else { 0 })
            .sum::<usize>()
    );
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
