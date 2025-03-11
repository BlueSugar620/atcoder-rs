use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(Usize1, Usize1, u64); m],
    }

    let mut dsu = PotentialDSU::<O>::new(n);
    for &(x, y, z) in &xyz {
        let t = dsu.unite(x, y, z);
        if !t {
            println!("-1");
            return;
        }
    }

    let mut cnt = vec![vec![0; 50]; n];
    for i in 0..n {
        let (r, p) = dsu.root(i);
        for k in 0..30 {
            if (p >> k) & 1 == 1 {
                cnt[r][k] += 1;
            }
        }
    }

    let mut geta = vec![0; n];
    for i in 0..n {
        if dsu.root(i).0 == i {
            for k in 0..30 {
                if 2 * cnt[i][k] > dsu.size(i) {
                    geta[i] += 1 << k;
                }
            }
        }
    }

    println!(
        "{}",
        (0..n)
            .map(|i| {
                let (r, p) = dsu.root(i);
                p ^ geta[r]
            })
            .join(" ")
    );
}

enum O {}
impl Group for O {
    type Value = u64;
    fn id() -> Self::Value {
        0
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs ^ rhs
    }
    fn inv(val: &Self::Value) -> Self::Value {
        *val
    }
}

pub trait Group {
    type Value: Clone + PartialEq;
    fn id() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct PotentialDSU<T: Group> {
    parents: Vec<isize>,
    potentials: Vec<T::Value>,
    cnt: usize,
}

impl<T: Group> PotentialDSU<T> {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            potentials: vec![T::id(); n],
            cnt: n,
        }
    }

    pub fn root(&self, mut v: usize) -> (usize, T::Value) {
        let mut potential = self.potentials[v].clone();
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
            potential = T::mul(&self.potentials[v], &potential);
        }
        (v, potential)
    }

    pub fn unite(&mut self, from: usize, to: usize, d: T::Value) -> bool {
        let (mut from, p_from) = self.root(from);
        let (mut to, p_to) = self.root(to);
        if from == to {
            T::mul(&p_from, &d) == p_to
        } else {
            let mut d = T::mul(&T::mul(&p_from, &d), &T::inv(&p_to));
            if self.parents[from] > self.parents[to] {
                std::mem::swap(&mut from, &mut to);
                d = T::inv(&d);
            }
            self.parents[from] += self.parents[to];
            self.parents[to] = from as isize;
            self.potentials[to] = d;
            self.cnt -= 1;
            true
        }
    }

    pub fn poteintial(&self, from: usize, to: usize) -> Option<T::Value> {
        let (from, p_from) = self.root(from);
        let (to, p_to) = self.root(to);
        if from == to {
            Some(T::mul(&T::inv(&p_from), &p_to))
        } else {
            None
        }
    }

    pub fn size(&self, u: usize) -> usize {
        -self.parents[self.root(u).0] as usize
    }

    pub fn cnt(&self) -> usize {
        self.cnt
    }
}
