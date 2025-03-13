use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        f: [[u64; w]; h],
        q: usize,
        ask: [(Usize1, Usize1, u64, Usize1, Usize1, u64); q],
    }

    let mut e = vec![];
    for i in 0..h {
        for j in 0..w {
            if i > 0 {
                e.push((f[i - 1][j].min(f[i][j]), (i - 1) * w + j, i * w + j));
            }
            if j > 0 {
                e.push((f[i][j - 1].min(f[i][j]), i * w + j - 1, i * w + j));
            }
        }
    }
    e.sort_unstable_by_key(|e| !e.0);

    let mut ans = vec![(0, 1 << 20); q];
    while ans.iter().any(|(l, r)| r - l > 1) {
        let mut queries = vec![];
        for (i, (l, r)) in ans.iter().enumerate() {
            if r - l > 1 {
                queries.push(((l + r) / 2, i));
            }
        }
        queries.sort_unstable_by_key(|q| !q.0);

        let mut dsu = DisjointSetUnion::new(h * w);
        let mut idx = 0;
        for &(o, i) in queries.iter() {
            let (a, b, _, c, d, _) = ask[i];
            while idx < e.len() && o <= e[idx].0 {
                let (_, x, y) = e[idx];
                dsu.unite(x, y);
                idx += 1;
            }
            if dsu.is_same(a * w + b, c * w + d) {
                ans[i] = (o, ans[i].1);
            } else {
                ans[i] = (ans[i].0, o);
            }
        }
    }

    println!(
        "{}",
        ask.iter()
            .zip(ans.iter())
            .map(|(ask, ans)| {
                if ask.2.min(ask.5) <= ans.0 {
                    ask.2.abs_diff(ask.5)
                } else {
                    ask.2 + ask.5 - 2 * ans.0
                }
            })
            .join("\n")
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
