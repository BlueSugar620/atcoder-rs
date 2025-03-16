use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut l_cnt = Counter::new(&[]);
    let mut r_cnt = Counter::new(&a);
    let mut duplication = LazySegmentTree::<O>::new(&vec![0; n]);
    let mut idxs = vec![vec![]; n];
    for (i, a) in a.iter().enumerate().rev() {
        idxs[*a].push(i);
    }
    for idx in idxs.iter() {
        if idx.len() > 1 {
            duplication.act(idx[idx.len() - 1]..idx[0], 1);
        }
    }

    let mut ans = 0;
    for &a in &a {
        l_cnt.incr(a);
        r_cnt.decr(a);
        let x = idxs[a].pop().unwrap();
        if idxs[a].len() > 1 {
            duplication.act(*idxs[a].last().unwrap()..idxs[a][0], 1);
        }
        if idxs[a].len() > 0 {
            duplication.act(x..idxs[a][0], !0);
        }
        ans = ans.max(l_cnt.variety() + r_cnt.variety() + duplication.fold(..));
    }

    println!("{}", ans);
}

pub struct Counter<T: Copy + Eq + std::hash::Hash> {
    values: std::collections::HashMap<T, usize>,
    cnt: usize,
}

impl<T: Copy + Eq + std::hash::Hash> Counter<T> {
    pub fn new(a: &[T]) -> Self {
        let mut values = std::collections::HashMap::new();
        for &a in a {
            *values.entry(a).or_insert(0) += 1;
        }
        Self {
            values,
            cnt: a.len(),
        }
    }

    pub fn add(&mut self, x: T, n: usize) -> usize {
        *self.values.entry(x).or_insert(0) += n;
        self.cnt += n;
        *self.values.get(&x).unwrap()
    }

    pub fn incr(&mut self, x: T) -> usize {
        *self.values.entry(x).or_insert(0) += 1;
        self.cnt += 1;
        *self.values.get(&x).unwrap()
    }

    pub fn reduce(&mut self, x: T, n: usize) -> usize {
        let a = *self.values.get(&x).unwrap_or(&0);
        if a <= n {
            self.values.remove(&x).unwrap();
            self.cnt -= a;
            0
        } else {
            self.values.insert(x, a - n);
            self.cnt -= n;
            a - n
        }
    }

    pub fn decr(&mut self, x: T) -> usize {
        let a = *self.values.get(&x).unwrap_or(&0);
        if a <= 1 {
            self.values.remove(&x).unwrap();
            self.cnt -= a;
            0
        } else {
            self.values.insert(x, a - 1);
            self.cnt -= 1;
            a - 1
        }
    }

    pub fn variety(&self) -> usize {
        self.values.len()
    }

    pub fn total(&self) -> usize {
        self.cnt
    }
}

enum O {}
impl MonoidAct2Monoid for O {
    type Value = usize;
    type Map = usize;
    fn e() -> Self::Value {
        0
    }
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        *lhs.max(rhs)
    }
    fn id() -> Self::Map {
        0
    }
    fn act(val: &mut Self::Value, map: &Self::Map) {
        *val = val.wrapping_add(*map);
    }
    fn comp(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map {
        lhs.wrapping_add(*rhs)
    }
}

pub trait MonoidAct2Monoid {
    type Value: Copy;
    type Map: Copy;
    fn e() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn id() -> Self::Map;
    fn comp(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map;
    fn act(val: &mut Self::Value, map: &Self::Map);
}

use std::ops::RangeBounds;
pub struct LazySegmentTree<T: MonoidAct2Monoid> {
    values: Vec<T::Value>,
    maps: Vec<T::Map>,
}

impl<T: MonoidAct2Monoid> LazySegmentTree<T> {
    pub fn new(a: &[T::Value]) -> Self {
        let n = a.len().next_power_of_two();
        let mut values = vec![T::e(); 2 * n];
        values[n..n + a.len()].clone_from_slice(a);
        for i in (1..n).rev() {
            values[i] = T::op(&values[2 * i], &values[2 * i + 1]);
        }
        Self {
            values,
            maps: vec![T::id(); 2 * n],
        }
    }

    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> T::Value {
        let n = self.values.len() / 2;
        let (l, r) = unzip(range, n);
        let (mut l, mut r) = (l + n, r + n);

        for i in (1..=n.trailing_zeros()).rev() {
            if (l >> i) << i != l {
                self.sink_map(l >> i);
            }
            if (r >> i) << i != r {
                self.sink_map((r - 1) >> i);
            }
        }
        let mut left = T::e();
        let mut right = T::e();
        while l < r {
            if l & 1 == 1 {
                left = T::op(&left, &self.values[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = T::op(&self.values[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&left, &right)
    }

    pub fn act(&mut self, range: impl RangeBounds<usize>, x: T::Map) {
        let n = self.values.len() / 2;
        let (l, r) = unzip(range, n);
        let (l, r) = (l + n, r + n);

        for i in (1..=n.trailing_zeros()).rev() {
            if (l >> i) << i != l {
                self.sink_map(l >> i);
            }
            if (r >> i) << i != r {
                self.sink_map((r - 1) >> i);
            }
        }
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l & 1 == 1 {
                    self.apply(l, &x);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.apply(r, &x);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        for i in 1..=n.trailing_zeros() {
            if (l >> i) << i != l {
                self.float_value(l >> i);
            }
            if (r >> i) << i != r {
                self.float_value((r - 1) >> i);
            }
        }
    }

    fn apply(&mut self, i: usize, x: &T::Map) {
        T::act(&mut self.values[i], x);
        self.maps[i] = T::comp(&self.maps[i], x);
    }

    fn float_value(&mut self, i: usize) {
        self.values[i] = T::op(&self.values[2 * i], &self.values[2 * i + 1]);
    }

    fn sink_map(&mut self, i: usize) {
        let a = std::mem::replace(&mut self.maps[i], T::id());
        self.apply(2 * i, &a);
        self.apply(2 * i + 1, &a);
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
