use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut rcl: [(Usize1, Usize1, usize); n],
    }

    let mut idx = (0..n).collect::<Vec<usize>>();
    idx.sort_unstable_by_key(|i| !rcl[*i].0);
    let mut lst = LazySegmentTree::<O>::new(&vec![h; w]);
    let mut ans = vec![!0; n];
    for &i in &idx {
        let (_, c, l) = rcl[i];
        let x = lst.fold(c..c + l);
        ans[i] = x;
        lst.act(c..c + l, Some(x - 1));
    }
    println!("{}", ans.iter().join("\n"));
}

enum O {}
impl MonoidAct2Monoid for O {
    type Value = usize;
    type Map = Option<usize>;
    fn e() -> Self::Value {
        !0
    }
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        *lhs.min(rhs)
    }
    fn id() -> Self::Map {
        None
    }
    fn act(val: &mut Self::Value, map: &Self::Map) {
        if let Some(x) = map {
            *val = *x;
        }
    }
    fn comp(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map {
        if let Some(rhs) = rhs {
            Some(*rhs)
        } else {
            *lhs
        }
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
    len: usize,
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
            len: a.len(),
        }
    }

    pub fn get_at(&mut self, i: usize) -> T::Value {
        let n = self.values.len() / 2;
        let i = i + n;
        for k in (1..=n.trailing_zeros()).rev() {
            self.sink_map(i >> k);
        }
        self.values[i]
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

    pub fn update_at(&mut self, i: usize, x: T::Value) {
        let n = self.values.len() / 2;
        let i = i + n;
        for k in (1..=n.trailing_zeros()).rev() {
            self.sink_map(i >> k);
        }
        self.values[i] = x;
        for k in 1..=n.trailing_zeros() {
            self.float_value(i >> k);
        }
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

    pub fn max_right<P: Fn(&T::Value) -> bool>(&mut self, l: usize, f: P) -> usize {
        let n = self.values.len() / 2;
        if l == n {
            return self.len;
        }
        let mut l = l + n;
        let mut r = 2 * n;
        for k in (1..=n.trailing_zeros()).rev() {
            self.sink_map(l >> k);
        }
        let mut x = T::e();
        while l < r {
            if l & 1 == 1 {
                let y = T::op(&x, &self.values[l]);
                if !f(&y) {
                    while l < n {
                        self.sink_map(l);
                        l *= 2;
                        let z = T::op(&x, &self.values[l]);
                        if f(&z) {
                            x = z;
                            l += 1;
                        }
                    }
                    return l - n;
                } else {
                    x = y;
                }
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.len
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
