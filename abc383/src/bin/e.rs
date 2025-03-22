use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut uvw: [(Usize1, Usize1, usize); m],
        a: [Usize1; k],
        b: [Usize1; k],
    }

    uvw.sort_unstable_by_key(|x| x.2);
    let mut dsu = ValuedDSU::<O>::new(n);
    for &a in &a {
        let (cnt_a, cnt_b) = dsu.value(a);
        dsu.update_at(a, (cnt_a + 1, cnt_b));
    }
    for &b in &b {
        let (cnt_a, cnt_b) = dsu.value(b);
        dsu.update_at(b, (cnt_a, cnt_b + 1));
    }

    let mut ans = 0;
    for &(u, v, w) in &uvw {
        dsu.unite(u, v);
        let (cnt_a, cnt_b) = dsu.value(u);
        let cnt = cnt_a.min(cnt_b);
        ans += cnt * w;
        dsu.update_at(u, (cnt_a - cnt, cnt_b - cnt));
    }

    println!("{}", ans);
}

enum O {}
impl Monoid for O {
    type Value = (usize, usize);
    fn e() -> Self::Value {
        (0, 0)
    }
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}

pub trait Monoid {
    type Value: Copy;
    fn e() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct ValuedDSU<T: Monoid> {
    parents: Vec<isize>,
    values: Vec<T::Value>,
    cnt: usize,
}

impl<T: Monoid> ValuedDSU<T> {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            values: vec![T::e(); n],
            cnt: n,
        }
    }

    pub fn root(&self, mut v: usize) -> usize {
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
        }
        v
    }

    pub fn value(&self, v: usize) -> T::Value {
        self.values[self.root(v)]
    }

    pub fn update_at(&mut self, v: usize, value: T::Value) {
        let v = self.root(v);
        self.values[v] = value;
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
        self.values[u] = T::op(&self.values[u], &self.values[v]);
        self.values[v] = T::e();
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
