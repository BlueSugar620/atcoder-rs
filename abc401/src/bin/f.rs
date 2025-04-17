use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
use proconio::*;
use rand::{thread_rng, Rng};
use std::collections::*;
use superslice::Ext;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n0: usize,
        uv0: [(Usize1, Usize1); n0 - 1],
        n1: usize,
        uv1: [(Usize1, Usize1); n1 - 1],
    }

    let mut e0 = vec![vec![]; n0];
    for &(u, v) in &uv0 {
        e0[u].push(v);
        e0[v].push(u);
    }
    let mut e1 = vec![vec![]; n1];
    for &(u, v) in &uv1 {
        e1[u].push(v);
        e1[v].push(u);
    }

    let (diameter0, (a0, b0)) = diameter(&uv0);
    let (diameter1, (a1, b1)) = diameter(&uv1);

    // 最遠点までの距離を求める
    let mut que = Deque::<usize>::new();

    let mut d00 = vec![!0usize; n0];
    d00[a0] = 0;
    que.push_back(a0);
    while let Some(u) = que.pop_front() {
        for &v in &e0[u] {
            if d00[v] == !0 {
                d00[v] = d00[u] + 1;
                que.push_back(v);
            }
        }
    }
    let mut d01 = vec![!0usize; n0];
    d01[b0] = 0;
    que.push_back(b0);
    while let Some(u) = que.pop_front() {
        for &v in &e0[u] {
            if d01[v] == !0 {
                d01[v] = d01[u] + 1;
                que.push_back(v);
            }
        }
    }
    let mut d0 = d00
        .iter()
        .zip(d01.iter())
        .map(|(x, y)| *x.max(y))
        .collect_vec();

    let mut d10 = vec![!0usize; n1];
    d10[a1] = 0;
    que.push_back(a1);
    while let Some(u) = que.pop_front() {
        for &v in &e1[u] {
            if d10[v] == !0 {
                d10[v] = d10[u] + 1;
                que.push_back(v);
            }
        }
    }
    let mut d11 = vec![!0usize; n1];
    d11[b1] = 0;
    que.push_back(b1);
    while let Some(u) = que.pop_front() {
        for &v in &e1[u] {
            if d11[v] == !0 {
                d11[v] = d11[u] + 1;
                que.push_back(v);
            }
        }
    }
    let mut d1 = d10
        .iter()
        .zip(d11.iter())
        .map(|(x, y)| *x.max(y))
        .collect_vec();

    d0.sort_unstable();
    d0.reverse();
    d1.sort_unstable();

    let is1 = InclusiveScan::<O>::new(&d1);

    let mut ans = 0;
    let mut idx = 0;
    for &x0 in &d0 {
        while idx < n1 && x0 + d1[idx] + 1 < diameter0.max(diameter1) {
            idx += 1;
        }
        ans += (x0 + 1) * (n1 - idx) + is1.fold(idx..);
        ans += diameter0.max(diameter1) * idx;
    }
    println!("{}", ans);
}

pub fn diameter(e: &[(usize, usize)]) -> (usize, (usize, usize)) {
    let n = e.len() + 1;
    let _e = e;
    let mut e = vec![Vec::new(); n];
    for &(u, v) in _e {
        e[u].push(v);
        e[v].push(u);
    }
    let mut dist0 = vec![!0usize; n];
    dist0[0] = 0;
    let mut stack0 = vec![0];
    while let Some(u) = stack0.pop() {
        for &v in &e[u] {
            if dist0[v] == !0 {
                dist0[v] = dist0[u] + 1;
                stack0.push(v);
            }
        }
    }
    let idx0 = (0..n).max_by_key(|&i| dist0[i]).unwrap();
    let mut dist1 = vec![!0; n];
    dist1[idx0] = 0;
    let mut stack1 = vec![idx0];
    while let Some(u) = stack1.pop() {
        for &v in &e[u] {
            if dist1[v] == !0 {
                dist1[v] = dist1[u] + 1;
                stack1.push(v);
            }
        }
    }
    let idx1 = (0..n).max_by_key(|&i| dist1[i]).unwrap();
    (dist1[idx1], (idx0, idx1))
}

enum O {}
impl Group for O {
    type Value = usize;
    fn id() -> Self::Value {
        0
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs.wrapping_add(*rhs)
    }
    fn inv(val: &Self::Value) -> Self::Value {
        val.wrapping_neg()
    }
}

use std::ops::{Bound, RangeBounds};

pub trait Group {
    type Value: Clone;
    fn id() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct InclusiveScan<T: Group> {
    scan: Vec<T::Value>,
}

impl<T: Group> InclusiveScan<T> {
    pub fn new(a: &[T::Value]) -> Self {
        Self {
            scan: std::iter::once(T::id())
                .chain(a.to_vec())
                .scan(T::id(), |acc, a| {
                    *acc = T::mul(acc, &a);
                    Some(acc.clone())
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Value {
        let (l, r) = unzip(range, self.scan.len() - 1);
        T::mul(&T::inv(&self.scan[l]), &self.scan[r])
    }
}

fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
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
