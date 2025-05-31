use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dsu = DisjointSetUnion::new(2 * 100_000 + 1);
    let mut ans = 0usize;
    for i in 0..n {
        if !dsu.is_same(a[i], a[n - 1 - i]) {
            dsu.unite(a[i], a[n - 1 - i]);
            ans += 1;
        }
    }
    println!("{}", ans);
}
pub struct DisjointSetUnion {
    parents: Vec<i32>,
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
        self.parents[v] = u as i32;
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
