use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
use proconio::*;
use rand::{thread_rng, Rng};
use std::{collections::*, ops::RangeBounds};
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;
type Heap<T> = BinaryHeap<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
        q: usize,
    }

    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }

    let euler_tour = EulerTour::new(0, &e);
    let mut value = vec![0; 2 * n];
    for i in 0..n {
        value[euler_tour.in_time(i)] += 1;
    }
    let mut fentree = FenwickTree::<O>::new(&value);
    let mut tot_weight = n as u64;

    for _ in 0..q {
        input! {
            t: u8,
        }

        if t == 1 {
            input! {
                x: Usize1,
                w: u64,
            }
            fentree.add_at(euler_tour.in_time(x), w);
            tot_weight += w;
        } else {
            input! {
                y: Usize1,
            }
            let x = if euler_tour.in_time(uv[y].0) > euler_tour.in_time(uv[y].1) {
                uv[y].0
            } else {
                uv[y].1
            };
            let subtree_weight = fentree.fold(euler_tour.subtree(x));
            println!("{}", subtree_weight.abs_diff(tot_weight - subtree_weight));
        }
    }
}

use std::ops::{Range, RangeInclusive};

pub struct EulerTour {
    tot_time: usize,
    in_time: Box<[usize]>,
    out_time: Box<[usize]>,
}

impl EulerTour {
    pub fn new(root: usize, e: &Vec<Vec<usize>>) -> Self {
        let n = e.len() + 1;
        let mut in_time = vec![!0; n];
        let mut out_time = vec![!0; n];
        let mut stack = vec![root];
        let mut t = 0usize;
        while let Some(u) = stack.pop() {
            if in_time[u] == !0 {
                in_time[u] = t;
                t += 1;
                stack.push(u);
                for &v in &e[u] {
                    if in_time[v] == !0 {
                        stack.push(v);
                    }
                }
            } else if out_time[u] == !0 {
                out_time[u] = t;
                t += 1;
            }
        }
        Self {
            tot_time: 2 * n,
            in_time: in_time.into_boxed_slice(),
            out_time: out_time.into_boxed_slice(),
        }
    }

    pub fn in_time(&self, u: usize) -> usize {
        self.in_time[u]
    }

    pub fn out_time(&self, u: usize) -> usize {
        self.out_time[u]
    }

    pub fn tot_time(&self) -> usize {
        self.tot_time
    }

    pub fn subtree(&self, u: usize) -> Range<usize> {
        self.in_time[u]..self.out_time[u]
    }

    pub fn path(&self, u: usize, v: usize) -> RangeInclusive<usize> {
        self.in_time[u]..=self.in_time[v]
    }
}

enum O {}
impl FenwickTreeOp for O {
    type Value = u64;
    fn e() -> Self::Value {
        0
    }
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs.wrapping_add(*rhs)
    }
    fn inv(val: &Self::Value) -> Self::Value {
        val.wrapping_neg()
    }
}

pub trait FenwickTreeOp {
    type Value: Clone;
    fn e() -> Self::Value;
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct FenwickTree<T: FenwickTreeOp> {
    values: Vec<T::Value>,
}

impl<T: FenwickTreeOp> FenwickTree<T> {
    pub fn new(a: &[T::Value]) -> Self {
        let n = a.len();
        let mut values = vec![T::e(); n + 1];
        for (i, a) in a.iter().enumerate() {
            let i = i + 1;
            values[i] = T::add(&values[i], &a);
            let lsb = i & i.wrapping_neg();
            if i + lsb < n + 1 {
                values[i + lsb] = T::add(&values[i + lsb], &values[i]);
            }
        }
        Self { values }
    }

    pub fn push(&mut self, mut x: T::Value) {
        let n = self.values.len();
        let lsb = n & n.wrapping_neg();
        let mut d = 1;
        while d < lsb {
            x = T::add(&x, &self.values[n - d]);
            d *= 2;
        }
        self.values.push(x);
    }

    pub fn add_at(&mut self, mut i: usize, x: T::Value) {
        i += 1;
        while i < self.values.len() {
            self.values[i] = T::add(&self.values[i], &x);
            i += i & i.wrapping_neg();
        }
    }

    pub fn _fold(&self, mut r: usize) -> T::Value {
        let mut res = T::e();
        while r > 0 {
            res = T::add(&res, &self.values[r]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Value {
        let n = self.values.len();
        let (l, r) = unzip(range, n - 1);
        T::add(&self._fold(r), &T::inv(&self._fold(l)))
    }
}

fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    use std::ops::Bound;
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
}
