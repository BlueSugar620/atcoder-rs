use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut dsu = DisjointSetUnion::new(n);
    let mut amari = vec![];

    for (i, &(a, b)) in ab.iter().enumerate() {
        if dsu.is_same(a, b) {
            amari.push(i);
        }
        dsu.unite(a, b);
    }

    let mut parent = vec![];
    let mut s = std::collections::HashSet::new();
    for i in 0..n {
        if dsu.root(i) == i {
            parent.push(i);
            s.insert(i);
        }
    }
    if parent.len() == 1 {
        println!("0");
        return;
    }
    println!("{}", parent.len() - 1);

    let mut x = vec![vec![]; n + 1];
    for &amari in &amari {
        let r = dsu.root(ab[amari].0);
        x[r].push(amari);
    }

    parent.sort_by_key(|i| !x[*i].len());

    let mut y = x[parent[0]].clone();
    let mut ans = vec![];
    for i in parent[1..].iter() {
        let j = y.pop().unwrap();
        let a = ab[j].0;
        ans.push((j, a, *i));
        y.extend(x[*i].iter());
    }

    for (x, y, z) in &ans {
        println!("{} {} {}", x + 1, y + 1, z + 1);
    }
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
